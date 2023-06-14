#![allow(unused_imports, unused_variables, dead_code)]
use crate::{
    client::account_handler::get_keys,
    fhe_node::{fhe_oracle::*, fhe_tx_execution::Tx},
};

use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_traits::*;
use rand::{rngs::OsRng, thread_rng};
use std::sync::Arc;

#[derive(Clone)]
pub struct User {
    pub address: String,
    pub key_path: String,
    pub der_key: String,
    pub sk: SecretKey,
    pub pk: PublicKey,
    pub fhe_balance: Ciphertext,
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

    fn create_tx(
        &self,
        receiver: User,
        oracle: &Oracle,
        value: u64,
        parameters: Arc<fhe::bfv::BfvParameters>,
    ) -> Tx {
        let sender = self.clone();

        let mut rng = thread_rng();

        assert!(user_balance(&sender) >= value, "Insufficient funds");
        assert!(value > 0, "Value must be greater than 0");

        let fhe_value = Plaintext::try_encode(&[value], Encoding::poly(), &parameters).unwrap();

        let txs: [Ciphertext; 2] = [
            sender.pk.try_encrypt(&fhe_value, &mut rng).unwrap(),
            receiver.pk.try_encrypt(&fhe_value, &mut rng).unwrap(),
        ];

        Tx::new(
            String::new(),
            self.address.clone(),
            receiver.address.clone(),
            txs[0].clone(),
            txs[1].clone(),
        )
    }
}

pub fn user_balance(user: &User) -> u64 {
    let decrypted_plaintext = user.sk.try_decrypt(&user.fhe_balance).unwrap();
    let decrypted_vector = Vec::<u64>::try_decode(&decrypted_plaintext, Encoding::poly()).unwrap();

    decrypted_vector[0]
}

pub fn create_user(
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

    User::new(address, key_path, der_key, sk, pk, fhe_balance).clone()
}

fn setup(alice_balance: u64, bob_balance: u64) -> (Oracle, User, User) {
    let mut fhe_oracle = Oracle::new();
    let parameters = fhe_oracle.parameters.clone();

    let alice = create_user(
        get_keys("alice").unwrap().public_key.to_string(),
        parameters.clone(),
        None,
        Some(alice_balance),
    );
    let bob = create_user(
        get_keys("bob").unwrap().public_key.to_string(),
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

#[cfg(test)]
mod test {
    use super::*;

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

        let txs = alice.create_tx(
            bob.clone(),
            &fhe_oracle,
            delta_balance,
            fhe_oracle.parameters.clone(),
        );

        let fhe_oracle = txs.execute_tx(&mut fhe_oracle.clone());

        let alice_oracle = fhe_oracle.users[&alice.address].clone();
        let bob_oracle = fhe_oracle.users[&bob.address].clone();

        let alice = User {
            fhe_balance: alice_oracle.fhe_balance,
            ..alice
        };

        let bob = User {
            fhe_balance: bob_oracle.fhe_balance,
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
}
