
use pillar_crypto::types::StdByteArray;
use serde::{Deserialize, Serialize};

use crate::reputation::history::NodeHistory;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransactionStub{
    // The block hash of the block that created this transaction
    pub block_hash: StdByteArray,
    // The transaction hash of the transaction that created this account
    pub transaction_hash: StdByteArray,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct Account{
    // The address of the account is the public key
    pub address: StdByteArray,
    // The balance of the account
    pub balance: u64,
    // The nonce of the account, to prevent replay attacks
    pub nonce: u64,
    // a tracking of blocks/transactions that lead to this balance
    pub history: Option<NodeHistory>
}

impl Account{
    // Creates a new account with the given address and balance
    pub fn new(address: StdByteArray, balance: u64) -> Self {
        // for now, this placeholder will work; however, in the long run we need a coinbase account for initial distribution
        // TODO deal with coinbase
        Account {
            address,
            balance,
            nonce: 0,
            history: None,
        }
    }
}