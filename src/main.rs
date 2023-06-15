#![allow(unused_imports, unused_variables, dead_code)]

use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_traits::*;
use rand::{rngs::OsRng, thread_rng};
use std::sync::Arc;

mod client {
    pub(crate) mod account_handler;
    pub(crate) mod event_handler;
    pub(crate) mod fhe_deployer;
    pub(crate) mod tx_handler;
}

mod fhe_node {
    pub(crate) mod fhe_account_handler;
    pub(crate) mod fhe_oracle;
    pub(crate) mod fhe_tx_execution;
}

use client::{account_handler::*, event_handler::*};
use fhe_node::{fhe_account_handler::*, fhe_oracle::*};

fn main() {
    println!("Hello, world!")
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
