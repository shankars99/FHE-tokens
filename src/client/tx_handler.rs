use ethers::abi::{decode, encode, Token};
use fhe::bfv::{Ciphertext, Plaintext, PublicKey};
use fhe_traits::Serialize;
use std::process::{Command, Output};
use std::str;

use crate::client::fhe_deployer::get_deployed_address;

fn buy_tokens_tx_sender(pk: &PublicKey, amount: u128, priv_key: &str) -> Option<String> {
    let deployed_address = get_deployed_address();

    let pk_bytes = pk.to_bytes();
    let pk_encoded = format!("0x{}", Token::Bytes(pk_bytes).to_string());

    let output = Command::new("/home/shankar/.foundry/bin/cast")
        .arg("send")
        .arg(deployed_address)
        .arg("buy_tokens(bytes)")
        .arg(pk_encoded)
        .arg("--private-key")
        .arg(priv_key)
        .arg("--value")
        .arg(amount.to_string())
        .output();

    match output {
        Ok(output) => get_tx_hash(output),
        Err(error) => {
            println!("Failed to execute script: {}", error);
            None
        }
    }
}

fn recvtx_tx_sender(
    to_address: &str,
    fhe_tx: &str,
    fhe_proof: &str,
    priv_key: &str,
) -> Option<String> {
    let deployed_address = get_deployed_address();

    let output = Command::new("/home/shankar/.foundry/bin/cast")
        .arg("send")
        .arg(deployed_address)
        .arg("recvTx(address,bytes,bytes)")
        .arg(to_address)
        .arg(fhe_tx)
        .arg(fhe_proof)
        .arg("--private-key")
        .arg(priv_key)
        .output();

    match output {
        Ok(output) => get_tx_hash(output),
        Err(error) => {
            println!("Failed to execute script: {}", error);
            None
        }
    }
}

fn get_tx_hash(output: Output) -> Option<String> {
    if output.status.success() {
        let stdout: std::borrow::Cow<str> = String::from_utf8_lossy(&output.stdout);
        let stdout = stdout.to_string();
        let tx_hash = stdout
            .split("transactionHash")
            .nth(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(",")
            .nth(0)
            .unwrap()
            .trim()
            .trim_matches('\"');

        Some(tx_hash.to_string())
    } else {
        println!("Error: {:?}", output.stderr);
        None
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use fhe::bfv::{BfvParameters, Encoding, SecretKey};
    use fhe_traits::{FheEncoder, FheEncrypter};
    use rand::thread_rng;
    use std::sync::Arc;

    use crate::client::account_handler::get_keys;
    use crate::fhe_node::fhe_account_handler::create_user;
    use crate::fhe_node::fhe_oracle::Oracle;

    #[test]
    fn test_buy_tokens() {
        let mut rng = thread_rng();

        let oracle = Oracle::new();

        let sk = SecretKey::random(&oracle.parameters, &mut rng);
        let pk = PublicKey::new(&sk, &mut rng);

        let priv_key = get_keys("owner").unwrap().private_key;
        let tx_hash = buy_tokens_tx_sender(&pk, 10, priv_key);

        assert!(tx_hash.is_some());
    }

    #[test]
    fn test_recvtx() {
        let mut rng = thread_rng();

        let oracle = Oracle::new();

        let sk_sender = SecretKey::random(&oracle.parameters, &mut rng);
        let pk_sender = PublicKey::new(&sk_sender, &mut rng);

        let sk_receiver = SecretKey::random(&oracle.parameters, &mut rng);
        let pk_receiver = PublicKey::new(&sk_receiver, &mut rng);

        let priv_key = get_keys("alice").unwrap().private_key;

        let parameters = oracle.parameters.clone();

        let tx_hash = buy_tokens_tx_sender(&pk_sender, 10, priv_key.clone());

        assert!(tx_hash.is_some());

        let fhe_balance: Plaintext =
            Plaintext::try_encode(&[10_u64], Encoding::poly(), &parameters).unwrap();
        let fhe_balance: Ciphertext = sk_sender.try_encrypt(&fhe_balance, &mut rng).unwrap();

        let fhe_proof: Plaintext =
            Plaintext::try_encode(&[10_u64], Encoding::poly(), &parameters).unwrap();

        let fhe_proof: Ciphertext = sk_sender.try_encrypt(&fhe_proof, &mut rng).unwrap();
    }
}
