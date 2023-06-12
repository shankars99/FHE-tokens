use crate::fhe_node::fhe_oracle::Oracle;
use fhe::bfv::{BfvParametersBuilder, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_traits::*;

#[derive(Clone)]
pub struct Tx {
    pub id: u128,
    pub sender: String,
    pub receiver: String,
    pub sender_tx: Ciphertext,
    pub receiver_tx: Ciphertext,
}

impl Tx {
    pub fn new(
        sender: String,
        receiver: String,
        sender_tx: Ciphertext,
        receiver_tx: Ciphertext,
    ) -> Self {
        let id = unsafe {
            LAST_TX_ID[1] += 1;
            LAST_TX_ID[1]
        };
        Self {
            id,
            sender,
            receiver,
            sender_tx,
            receiver_tx,
        }
    }

    pub fn execute_tx(&self, oracle: &mut Oracle) -> Oracle {
        let tx = self.clone();
        let sender = tx.sender.clone();
        let receiver = tx.receiver.clone();
        let sender_tx = tx.sender_tx.clone();
        let receiver_tx = tx.receiver_tx.clone();

        let sender_fhe_balance = oracle.return_user_fhe_balance(sender.clone());
        let receiver_fhe_balance = oracle.return_user_fhe_balance(receiver.clone());

        let sender_fhe_balance = &sender_fhe_balance - &sender_tx;
        let receiver_fhe_balance = &receiver_fhe_balance + &receiver_tx;

        oracle.update_user_fhe_balance(sender.clone(), sender_fhe_balance);
        oracle.update_user_fhe_balance(receiver.clone(), receiver_fhe_balance);

        oracle.clone()
    }
}

// Block, Tx
static mut LAST_TX_ID: [u128; 2] = [0, 0];
