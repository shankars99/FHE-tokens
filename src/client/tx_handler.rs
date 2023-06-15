use ethers::abi::{decode, encode, Token};
use fhe::bfv::{Ciphertext, Plaintext, PublicKey};
use fhe_traits::Serialize;
use std::process::Output;
use std::str;

use crate::client::fhe_deployer::get_deployed_address;

use tokio::io::AsyncReadExt;
use tokio::process::Command;

async fn buy_tokens_tx_sender(
    pk: &PublicKey,
    priv_key: &String,
    amount: &String,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
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
        .arg(amount)
        .output()
        .await?;

    match get_tx_hash(output).await {
        Ok(tx_hash) => Ok(tx_hash),
        Err(error) => {
            eprintln!("Failed to execute script: {}", error);
            Ok(None)
        }
    }
}

/*
function recvTx(
        string calldata _to,
        bytes calldata _fhe_tx_sender,
        bytes calldata _fhe_tx_receiver,
        bytes calldata _fhe_proof
    )
*/

pub async fn recvtx_tx_sender(
    to_address: &str,
    fhe_tx_sender: &str,
    fhe_tx_receiver: &str,
    fhe_proof: &str,
    priv_key: &String,
    amount: &String,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let deployed_address = get_deployed_address();

    let output = Command::new("/home/shankar/.foundry/bin/cast")
        .arg("send")
        .arg(deployed_address)
        .arg("recvTx(address,bytes,bytes,bytes)")
        .arg(to_address)
        .arg(fhe_tx_sender)
        .arg(fhe_tx_receiver)
        .arg(fhe_proof)
        .arg("--private-key")
        .arg(priv_key)
        .arg("--value")
        .arg(amount)
        .output()
        .await?;

    match get_tx_hash(output).await {
        Ok(tx_hash) => Ok(tx_hash),
        Err(error) => {
            eprintln!("Failed to execute script: {}", error);
            Ok(None)
        }
    }
}

async fn get_tx_hash(output: Output) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if output.status.success() {
        let mut stdout = String::new();
        tokio::io::AsyncReadExt::read_to_string(&mut &output.stdout[..], &mut stdout).await?;

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
            .trim_matches('\"')
            .to_string();

        Ok(Some(tx_hash))
    } else {
        eprintln!("Error: {:?}", output.stderr);
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fhe::bfv::{BfvParameters, Encoding, SecretKey};
    use fhe_traits::{FheEncoder, FheEncrypter};
    use rand::thread_rng;
    use std::sync::Arc;

    use crate::client::{
        account_handler::{get_keys, tests::create_users},
        fhe_deployer::FEE,
    };
    use crate::fhe_node::{
        fhe_account_handler::create_user, fhe_oracle::Oracle, fhe_tx_execution::Tx,
    };
    #[tokio::test]
    async fn test_buy_tokens() {
        let rng = thread_rng();

        let (fhe_oracle, alice, bob, owner) = create_users(100, 50);

        let priv_key = get_keys("owner").unwrap().private_key.to_string();
        let pk = owner.pk.clone();

        let tx_hash = buy_tokens_tx_sender(&pk, &priv_key, &FEE.to_string()).await;
        assert!(tx_hash.is_ok());
    }

    #[tokio::test]
    async fn test_recvtx() {
        let rng = thread_rng();

        let (fhe_oracle, alice, bob, owner) = create_users(100, 50);

        let priv_key = get_keys("owner").unwrap().private_key.to_string();
        let pk = owner.pk.clone();

        let tx_hash = buy_tokens_tx_sender(&pk, &priv_key, &FEE.to_string()).await;

        let tx = alice.create_tx(bob.clone(), &fhe_oracle, 10);

        let (tx_sender, tx_receiver) = tx.serialize_ct_tx_string();

        let tx_hash = recvtx_tx_sender(
            &bob.address.clone(),
            &tx_sender,
            &tx_receiver,
            &tx.tx_proof,
            &priv_key,
            &FEE.to_string(),
        )
        .await;

        assert!(tx_hash.is_ok());
    }
}
