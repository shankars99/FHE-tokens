#![allow(unused_imports, unused_variables, dead_code)]

use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_traits::*;
use rand::{rngs::OsRng, thread_rng};
use std::sync::Arc;

use crate::crypto::fhe_oracle;

use fhe_oracle::{Oracle, OracleUser};

struct User {
    address: String,
    key_path: String,
    der_key: String,
    sk: SecretKey,
    pk: PublicKey,
    fhe_balance: Ciphertext,
}

impl User {
    fn new(
        address: String,
        key_path: String,
        der_key: String,
        sk: SecretKey,
        pk: PublicKey,
        fhe_balance: Ciphertext,
    ) -> Self {
        Self {
            address,
            key_path,
            der_key,
            sk,
            pk,
            fhe_balance,
        }
    }

    fn update_fhe_balance(&mut self, new_fhe_balance: Ciphertext) {
        self.fhe_balance = new_fhe_balance;
    }

    fn clone(&self) -> Self {
        Self {
            address: self.address.clone(),
            key_path: self.key_path.clone(),
            der_key: self.der_key.clone(),
            sk: self.sk.clone(),
            pk: self.pk.clone(),
            fhe_balance: self.fhe_balance.clone(),
        }
    }
}

fn create_tx(
    oracle: &Oracle,
    sender: User,
    receiver: User,
    value: u64,
    parameters: Arc<fhe::bfv::BfvParameters>,
) -> (Ciphertext, Ciphertext) {
    let mut rng = thread_rng();

    assert!(user_balance(&sender) >= value, "Insufficient funds");
    assert!(value > 0, "Value must be greater than 0");

    let fhe_value = Plaintext::try_encode(&[value], Encoding::poly(), &parameters).unwrap();

    let new_fhe_balance_receiver =
        &receiver.fhe_balance + &receiver.pk.try_encrypt(&fhe_value, &mut rng).unwrap();

    let new_fhe_balance_sender =
        &sender.fhe_balance - &sender.pk.try_encrypt(&fhe_value, &mut rng).unwrap();

    let mut sender = sender; // Make sender mutable
    let mut receiver = receiver; // Make receiver mutable
    sender.update_fhe_balance(new_fhe_balance_sender);
    receiver.update_fhe_balance(new_fhe_balance_receiver);

    (sender.fhe_balance.clone(), receiver.fhe_balance.clone())
}

fn user_balance(user: &User) -> u64 {
    let decrypted_plaintext = user.sk.try_decrypt(&user.fhe_balance).unwrap();
    let decrypted_vector = Vec::<u64>::try_decode(&decrypted_plaintext, Encoding::poly()).unwrap();
    println!("{}'s balance: {}", user.address, decrypted_vector[0]);

    decrypted_vector[0]
}

fn create_user(
    address: String,
    parameters: Arc<fhe::bfv::BfvParameters>,
    der_key: Option<String>,
    start_balance: Option<u64>,
) -> User {
    let mut rng = thread_rng();

    let der_key = der_key.unwrap_or("default".to_string());
    let start_balance = start_balance.unwrap_or(0);

    let mut key_path = "keys/".to_string() + &address;
    let sk = SecretKey::random_and_write_to_file(&parameters, &mut OsRng, &mut key_path);

    let pk = PublicKey::new(&sk, &mut rng);

    let balance: Plaintext =
        Plaintext::try_encode(&[start_balance], Encoding::poly(), &parameters).unwrap();
    let fhe_balance: Ciphertext = sk.try_encrypt(&balance, &mut rng).unwrap();

    User::new(address, key_path, der_key, sk, pk, fhe_balance)
}

fn setup(alice_balance: u64, bob_balance: u64) -> (Oracle, User, User) {
    let mut fhe_oracle = Oracle::new();
    let parameters = fhe_oracle.parameters.clone();

    let alice = create_user(
        "alice".to_string(),
        parameters.clone(),
        None,
        Some(alice_balance),
    );
    let bob = create_user(
        "bob".to_string(),
        parameters.clone(),
        None,
        Some(bob_balance),
    );

    let oracle_alice = OracleUser::new(
        alice.address.to_string(),
        alice.pk.clone(),
        alice.fhe_balance.clone(),
    );

    let oracle_bob = OracleUser::new(
        bob.address.to_string(),
        bob.pk.clone(),
        bob.fhe_balance.clone(),
    );

    fhe_oracle.add_user(alice.address.clone(), oracle_alice);
    fhe_oracle.add_user(bob.address.clone(), oracle_bob);

    (fhe_oracle, alice, bob)
}

fn main() {
    let (fhe_oracle, mut alice, mut bob) = setup(100, 50);

    let parameters = fhe_oracle.parameters.clone();

    println!("Creating users...");
    user_balance(&alice);
    user_balance(&bob);

    println!("\nBob public key: {:?}", bob.pk);
    println!("\nAlice public key: {:?}", alice.pk);

    println!("\nCreating tx where alice sends 50 to bob...");
    let (new_fhe_balance_alice, new_fhe_balance_bob) = create_tx(
        &fhe_oracle,
        alice.clone(),
        bob.clone(),
        50,
        parameters.clone(),
    );
    alice.update_fhe_balance(new_fhe_balance_alice);
    bob.update_fhe_balance(new_fhe_balance_bob);

    println!("\nChecking balances...");
    user_balance(&alice);
    user_balance(&bob);
}

#[test]
fn test_create_users() {
    let init_alice_balance = 100;
    let init_bob_balance = 50;

    let (fhe_oracle, alice, bob) = setup(init_alice_balance, init_bob_balance);

    assert!(
        user_balance(&alice) == init_alice_balance,
        "Alice's balance is incorrect"
    );
    assert!(
        user_balance(&bob) == init_bob_balance,
        "Bob's balance is incorrect"
    );
}

#[test]
fn test_tx_send_and_receive() {
    let init_alice_balance = 100;
    let init_bob_balance = 50;
    let delta_balance = 10;

    let (fhe_oracle, alice, bob) = setup(init_alice_balance, init_bob_balance);

    let (new_fhe_balance_alice, new_fhe_balance_bob) = create_tx(
        &fhe_oracle,
        alice.clone(),
        bob.clone(),
        delta_balance,
        fhe_oracle.parameters.clone(),
    );
    let alice = User {
        fhe_balance: new_fhe_balance_alice,
        ..alice
    };

    let bob = User {
        fhe_balance: new_fhe_balance_bob,
        ..bob
    };

    assert!(
        user_balance(&alice) == init_alice_balance - delta_balance,
        "Alice's balance is incorrect"
    );
    assert!(
        user_balance(&bob) == init_bob_balance + delta_balance,
        "Bob's balance is incorrect"
    );
}
