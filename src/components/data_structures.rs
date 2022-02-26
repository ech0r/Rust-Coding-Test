use serde::{Serialize, Deserialize};

// this enum represents all the forms a "transaction" can take
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TXType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

// type interface for TXType
pub type TransactionType = TXType;

#[derive(Debug)]
pub struct Client {
    pub client: u16, 
    pub available: u64,
    pub held: u64, 
    pub total: u64,
    pub transactions: Vec<Transaction>,
    pub locked: bool,
}

// RawClient is what gets written to the output file then RawClient gets converted "Into" Client
#[derive(Debug, Serialize, PartialEq)]
pub struct RawClient {
    pub client: u16, 
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

// Transaction is our "source of truth" for each transaction while we are processing the data, it contains a flag which allows us to tell if it's been disputed
// We store the decimal values as integers in Transaction while we are performing arithmetic operations to avoid rounding errors and the performance hit of other crates
#[derive(Debug)]
pub struct Transaction {
    pub transaction_type: TransactionType, 
    pub client: u16,
    pub tx: u32,
    pub disputed: bool,
    pub resolved: bool,
    pub amount: Option<u64>, // we use an Option here since not all transaction types have an amount.
}

// RawTransaction is read directly from the file then RawTransaction gets converted "Into" a Transaction
#[derive(Debug, Serialize, Deserialize)]
pub struct RawTransaction {
    #[serde(rename = "type")] // "type" is a reserved keyword in Rust so we use serde to rename it dynamically
    pub transaction_type: TransactionType, 
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>, // we use an Option here since not all transaction types have an amount.
}