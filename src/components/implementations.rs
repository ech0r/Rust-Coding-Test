use std::error::Error;

// LOCAL
use crate::components::data_structures::{Client, Transaction, RawClient, RawTransaction};

impl Client {
    pub fn new(id: u16) -> Self {
        Client {
            client: id,
            available: 0,
            held: 0,
            total: 0,
            transactions: Vec::new(),
            locked: false,
        }
    }
    pub fn is_frozen(&self) -> Result<(), Box<dyn Error>> {
        match self.locked {
            false => Ok(()),
            true => Err(format!("[ERROR]: Client's (id: {}) account is frozen, further transactions are not allowed.", self.client).into()),
        }
    }
}

impl Transaction {
    pub fn is_already_disputed(&self) -> Result<(), Box<dyn Error>> {
        match self.disputed {
            false => Ok(()),
            // taking care of edge case with multiple disputes
            true => Err(format!("[ERROR]: Transaction id: {}, is already disputed, further disputes are not allowed. Discarding transaction.", self.tx).into()),
        }
    }
    pub fn is_already_resolved(&self) -> Result<(), Box<dyn Error>> {
        match self.resolved {
            false => Ok(()),
            // taking care of edge case with multiple resolves
            true => Err(format!("[ERROR]: Transaction id: {}, is already resolved, further resolves and chargebacks are not allowed. Discarding transaction.", self.tx).into()),
        }
    }
}

impl From<RawTransaction> for Transaction { // with a From implementation, we automatically get an Into implementation
    fn from(raw_tx: RawTransaction) -> Transaction {
        match raw_tx.amount {
            Some(actual_amount) => {
                Transaction {
                    transaction_type: raw_tx.transaction_type,
                    client: raw_tx.client,
                    tx: raw_tx.tx,
                    disputed: false,
                    resolved: false,
                    amount: Some((actual_amount * 10000.0) as u64),
                }
            },
            None => {
                Transaction {
                    transaction_type: raw_tx.transaction_type,
                    client: raw_tx.client,
                    tx: raw_tx.tx,
                    disputed: false,
                    resolved: false,
                    amount: None,
                }
            }
        }
    }
}

impl From<Transaction> for RawTransaction { // with a From implementation, we automatically get an Into implementation
    fn from(tx: Transaction) -> RawTransaction {
        match tx.amount {
            Some(actual_amount) => {
                RawTransaction {
                    transaction_type: tx.transaction_type,
                    client: tx.client,
                    tx: tx.tx,
                    amount: Some((actual_amount as f64) / 10000.0 ),
                }
            },
            None => {
                RawTransaction {
                    transaction_type: tx.transaction_type,
                    client: tx.client,
                    tx: tx.tx,
                    amount: None,
                }
            }
        }
    }
}

impl From<RawClient> for Client {
    fn from(raw_cl: RawClient) -> Client {
        Client {
            client: raw_cl.client,
            available: (raw_cl.available * 10000.0) as u64, 
            held: (raw_cl.held * 10000.0) as u64, 
            total: (raw_cl.total * 10000.0) as u64, 
            transactions: Vec::new(),
            locked: raw_cl.locked, 
        }
    }
}

impl From<&Client> for RawClient {
    fn from(cl: &Client) -> RawClient {
        RawClient {
            client: cl.client,
            available: ((cl.available as f64) / 10000.0),
            held: ((cl.held as f64) / 10000.0),
            total: ((cl.total as f64) / 10000.0),
            locked: cl.locked,
        }
    }
}