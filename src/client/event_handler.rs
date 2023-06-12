use crate::client::fhe_deployer::get_deployed_address;

use ethers::{
    abi::{decode, ParamType, Token},
    core::types::{Address, Filter, U256},
    prelude::k256::elliptic_curve::consts::U8,
    providers::{Http, Middleware, Provider},
    types::Uint8,
};

use eyre::Result;
use std::{sync::Arc, u8};

use std::convert::TryFrom;

pub async fn deposit_event_handler(url: &str, contract_addr: &str, start_block: u64) -> Result<()> {
    let provider = Provider::<Http>::try_from(url)?;
    let client = Arc::new(provider);
    let filter = Filter::new()
        .address(contract_addr.parse::<Address>()?)
        .event("Deposit(address,uint256,string)")
        .from_block(start_block);
    let logs = client.get_logs(&filter).await?;
    for log in logs.iter() {
        let from = Address::from(log.topics[1]);

        let decoded: Vec<Token> = decode(&[ParamType::Uint(256), ParamType::String], &log.data)?;
        for token in decoded {
            match token {
                Token::String(string) => {
                    // Handle String value
                    let string_value: String = string.into();
                    println!("Public Key: {}", string_value);
                }
                _ => {}
            }
        }
    }
    Ok(())
}

pub async fn recvnewtx_event_handler(
    url: &str,
    contract_addr: &str,
    start_block: u64,
) -> Result<()> {
    let provider = Provider::<Http>::try_from(url)?;
    let client = Arc::new(provider);
    let filter = Filter::new()
        .address(contract_addr.parse::<Address>()?)
        .event("RecvNewTx(uint256,address,address,string,string)")
        .from_block(start_block);
    let logs = client.get_logs(&filter).await?;

    for log in logs.iter() {
        let id = Address::from(log.topics[1]);
        let from = Address::from(log.topics[2]);
        let to = Address::from(log.topics[3]);

        println!("id = {}", id);
        println!("from = {}", from);
        println!("to = {}", to);

        let event_names = vec!["fhe_tx", "proof"];

        let decoded: Vec<Token> = decode(&[ParamType::Bytes, ParamType::Bytes], &log.data)?;

        for (i, token) in decoded.iter().enumerate() {
            match token {
                Token::Uint(uint) => {
                    // Handle Uint value
                    let uint_value: U256 = uint.into();
                    println!("{}:{}", event_names[i], uint_value);
                }
                Token::String(string) => {
                    // Handle Bytes value
                    let string_value: String = string.into();
                    println!("{}:{}", event_names[i], string_value);
                }
                _ => {}
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::fhe_deployer::get_deployed_address;
    use ethers::providers::Http;
    use std::convert::TryFrom;

    #[tokio::test]
    async fn test_deposit_listener() {
        let url = "http://127.0.0.1:8545";
        let contract_addr = get_deployed_address();
        let start_block = 0;
        let res = deposit_event_handler(url, contract_addr, start_block).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_recvtx_listener() {
        let url = "http://127.0.0.1:8545";
        let contract_addr = get_deployed_address();
        let start_block = 0;
        let res = recvnewtx_event_handler(url, contract_addr, start_block).await;

        assert!(res.is_ok());
    }
}
