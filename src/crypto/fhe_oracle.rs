use fhe::bfv::{
    BfvParameters, BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey,
};
use fhe_traits::*;
use std::collections::HashMap;
use std::sync::Arc;

pub struct OracleUser {
    address: String,
    pk: PublicKey,
    fhe_balance: Ciphertext,
}

impl OracleUser {
    pub fn new(address: String, pk: PublicKey, fhe_balance: Ciphertext) -> Self {
        Self {
            address,
            pk,
            fhe_balance,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            address: self.address.clone(),
            pk: self.pk.clone(),
            fhe_balance: self.fhe_balance.clone(),
        }
    }
}
pub struct Oracle {
    pub(crate) users: HashMap<String, OracleUser>,
    pub(crate) parameters: Arc<fhe::bfv::BfvParameters>,
}

impl Oracle {
    pub fn new() -> Self {
        let parameters = Arc::new(
            BfvParametersBuilder::new()
                .set_degree(2048)
                .set_moduli(&[0x3fffffff000001])
                .set_plaintext_modulus(1 << 10)
                .build()
                .unwrap(),
        );
        let users = HashMap::new();

        Self { users, parameters }
    }

    pub fn add_user(&mut self, address: String, user: OracleUser) {
        self.users.insert(address.to_string(), user);
    }

    pub fn return_user_fhe_balance(&self, address: String) -> Ciphertext {
        self.users[&address].fhe_balance.clone()
    }

    pub fn return_user_pk(&self, address: String) -> PublicKey {
        self.users[&address].pk.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::fhe_oracle::{Oracle, OracleUser};
    use fhe::bfv::{BfvParameters, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
    use fhe_traits::{FheDecoder, FheDecrypter, FheEncoder, FheEncrypter};
    use rand::thread_rng;

    #[test]
    fn test_oracle_user() {
        let mut oracle = Oracle::new();
        let oracle_parameters = oracle.parameters.clone();
        let mut rng = rand::thread_rng();

        let sk = SecretKey::random(&oracle_parameters, &mut rng);
        let pk = PublicKey::new(&sk, &mut rng);

        let balance: Plaintext =
            Plaintext::try_encode(&[0_u64], Encoding::poly(), &oracle_parameters).unwrap();
        let fhe_balance: Ciphertext = sk.try_encrypt(&balance, &mut rng).unwrap();

        let address = "0x123".to_string();
        let user = OracleUser::new(address.clone(), pk, fhe_balance);
        let address_clone = address.clone();
        oracle.add_user(address_clone, user.clone());

        assert_eq!(user.address, oracle.users[&address].address);

        let decrypted_plaintext = sk.try_decrypt(&user.fhe_balance).unwrap();
        let decrypted_vector =
            Vec::<u64>::try_decode(&decrypted_plaintext, Encoding::poly()).unwrap();
        println!("{}'s balance: {}", user.address, decrypted_vector[0]);
    }
}
