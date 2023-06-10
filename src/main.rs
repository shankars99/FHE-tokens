#![allow(unused_imports, unused_variables, dead_code)]

use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_traits::*;
use rand::{rngs::OsRng, thread_rng};
use std::sync::Arc;

mod fhe_oracle;
use fhe_oracle::*;

mod event_handler;
use event_handler::*;

struct User {
    name: String,
    key_path: String,
    der_key: String,
    sk: SecretKey,
    pk: PublicKey,
    fhe_balance: Ciphertext,
}

fn create_tx<'a>(
    oracle: Oracle,
    sender: &'a User,
    receiver: &'a User,
    value: u64,
    parameters: Arc<fhe::bfv::BfvParameters>,
) -> (Ciphertext, Ciphertext) {
    let mut rng = thread_rng();

    assert!(user_balance(sender) >= value, "Insufficient funds");
    assert!(value > 0, "Value must be greater than 0");

    let fhe_value = Plaintext::try_encode(&[value], Encoding::poly(), &parameters).unwrap();

    let new_fhe_balance_receiver =
        &receiver.fhe_balance + &receiver.pk.try_encrypt(&fhe_value, &mut rng).unwrap();

    let new_fhe_balance_sender =
        &sender.fhe_balance - &sender.pk.try_encrypt(&fhe_value, &mut rng).unwrap();

    (new_fhe_balance_sender, new_fhe_balance_receiver)
}

fn user_balance(user: &User) -> u64 {
    let decrypted_plaintext = user.sk.try_decrypt(&user.fhe_balance).unwrap();
    let decrypted_vector = Vec::<u64>::try_decode(&decrypted_plaintext, Encoding::poly()).unwrap();
    println!("{}'s balance: {}", user.name, decrypted_vector[0]);

    decrypted_vector[0]
}

fn create_user(
    name: String,
    parameters: Arc<fhe::bfv::BfvParameters>,
    der_key: Option<String>,
    start_balance: Option<u64>,
) -> User {
    let mut rng = thread_rng();

    let der_key = der_key.unwrap_or("default".to_string());
    let start_balance = start_balance.unwrap_or(0);

    let mut key_path = "keys/".to_string() + &name;
    let sk = SecretKey::random_and_write_to_file(&parameters, &mut OsRng, &mut key_path);

    let pk = PublicKey::new(&sk, &mut rng);

    let balance: Plaintext =
        Plaintext::try_encode(&[start_balance], Encoding::poly(), &parameters).unwrap();
    let fhe_balance: Ciphertext = sk.try_encrypt(&balance, &mut rng).unwrap();

    User {
        name,
        key_path,
        der_key,
        sk,
        pk,
        fhe_balance,
    }
}

fn setup(
    alice_balance: u64,
    bob_balance: u64,
) -> (Arc<fhe::bfv::BfvParameters>, User, User, Oracle) {
    let parameters = oracle_parameters();

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

    let oracle_alice = OracleUser {
        name: alice.name.clone(),
        pk: alice.pk.clone(),
        fhe_balance: alice.fhe_balance.clone(),
    };

    let oracle_bob = OracleUser {
        name: bob.name.clone(),
        pk: bob.pk.clone(),
        fhe_balance: bob.fhe_balance.clone(),
    };

    let oracle = Oracle {
        users: vec![oracle_alice, oracle_bob],
        parameters: parameters.clone(),
    };

    (parameters, alice, bob, oracle)
}

fn main() {
    let (parameters, alice, bob, oracle) = setup(100, 50);

    println!("Creating users...");
    user_balance(&alice);
    user_balance(&bob);

    println!("\nBob public key: {:?}", bob.pk);
    println!("\nAlice public key: {:?}", alice.pk);

    println!("\nCreating tx where alice sends 50 to bob...");
    let (new_fhe_balance_alice, new_fhe_balance_bob) =
        create_tx(oracle, &alice, &bob, 50, parameters.clone());

    let alice = User {
        fhe_balance: new_fhe_balance_alice,
        ..alice
    };

    let bob = User {
        fhe_balance: new_fhe_balance_bob,
        ..bob
    };

    println!("\nChecking balances...");
    user_balance(&alice);
    user_balance(&bob);
}

#[test]
fn create_users() {
    let init_alice_balance = 100;
    let init_bob_balance = 50;

    let (parameters, alice, bob, oracle_users) = setup(init_alice_balance, init_bob_balance);

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

    let (parameters, alice, bob, oracle_users) = setup(init_alice_balance, init_bob_balance);

    let (new_fhe_balance_alice, new_fhe_balance_bob) = create_tx(
        oracle_users,
        &alice,
        &bob,
        delta_balance,
        parameters.clone(),
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
