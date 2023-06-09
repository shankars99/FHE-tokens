use std::sync::Arc;

use fhe::bfv::{
    BfvParameters, BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey,
};

pub struct OracleUser {
    pub(crate) name: String,
    pub(crate) pk: PublicKey,
    pub(crate) fhe_balance: Ciphertext,
}

pub struct Oracle {
    pub(crate) users: Vec<OracleUser>,
    pub(crate) parameters: Arc<fhe::bfv::BfvParameters>,
}

pub fn oracle_parameters() -> Arc<fhe::bfv::BfvParameters> {
    Arc::new(
        BfvParametersBuilder::new()
            .set_degree(2048)
            .set_moduli(&[0x3fffffff000001])
            .set_plaintext_modulus(1 << 10)
            .build()
            .unwrap(),
    )
}

pub fn return_user_fhe_balance(user: OracleUser) -> Ciphertext {
    user.fhe_balance
}

pub fn return_user_pk(user: OracleUser) -> PublicKey {
    user.pk
}
