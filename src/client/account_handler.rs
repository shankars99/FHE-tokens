pub struct KeyPair {
    pub public_key: &'static str,
    pub private_key: &'static str,
}

impl KeyPair {
    pub fn new(public_key: &'static str, private_key: &'static str) -> Self {
        Self {
            public_key,
            private_key,
        }
    }
}

/*
Contains pub-priv key pairs from ganache-cli
DO NOT USE IN PRODUCTION
*/
pub fn get_keys(user: &str) -> Option<KeyPair> {
    match user {
        "owner" => Some(KeyPair::new(
            "0x40f2AC22966Ec049555FE7876c1803AFe49A717A",
            "0xa584b5b2e4c973c4d90ff63f930df0c2938c4c5af029373881b35ea6ec1839a0",
        )),
        "alice" => Some(KeyPair::new(
            "0xA39f5AcC00c3Ba685133e2cb3067414eAAC69A43",
            "0xa062ba0a7168f42b5cd45f05672cb2bdde0fe72a5f1056d5cc994bfd272005c3",
        )),
        "bob" => Some(KeyPair::new(
            "0xD39DBd7603ED1d9755151Fe6532d33D76Dc909D0",
            "0xbfb34b71033d5103c25b9fbbcf0149f8badc06610969f4d13c6b1efb02c28951",
        )),
        "charlie" => Some(KeyPair::new(
            "0xbE8cF79fa5bF19C0106023A0f3be6fd0e5b1D074",
            "0x5642314d8f1f9bfadbf52790bc68d288ddbabae8488c060b0f8871950f500677",
        )),
        "dave" => Some(KeyPair::new(
            "0x0e5Abeb462A67d7E499d95b4Ad777e0e8DCbF27d",
            "0x1d197f1c9d17cbbca2d1f746d73d8d04befdf4c22ef1e62c8024db2bec52ad5c",
        )),
        _ => None,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    // creates alice and bob, adds then to the oracle and returns the oracle and returns them
    use super::*;
    use crate::fhe_node::{
        fhe_account_handler::{create_user, User},
        fhe_oracle::{Oracle, OracleUser},
    };

    pub fn create_users(alice_balance: u64, bob_balance: u64) -> (Oracle, User, User, User) {
        let mut oracle = Oracle::new();

        let owner = create_user(
            get_keys("owner").unwrap().public_key.to_string(),
            oracle.parameters.clone(),
            None,
            Some(100),
        );

        let alice = create_user(
            get_keys("alice").unwrap().public_key.to_string(),
            oracle.parameters.clone(),
            None,
            Some(alice_balance),
        );

        let bob = create_user(
            get_keys("bob").unwrap().public_key.to_string(),
            oracle.parameters.clone(),
            None,
            Some(bob_balance),
        );

        oracle.add_user(owner.address.clone(), OracleUser::from_user(owner.clone()));
        oracle.add_user(alice.address.clone(), OracleUser::from_user(alice.clone()));
        oracle.add_user(bob.address.clone(), OracleUser::from_user(bob.clone()));

        (oracle, alice, bob, owner)
    }
}
