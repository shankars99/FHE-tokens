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

mod crypto {
    pub(crate) mod fhe_compute_handler;
    pub(crate) mod fhe_oracle;
}

use client::event_handler::*;
use crypto::fhe_oracle::*;

fn main() {
    println!("Hello, world!")
}
