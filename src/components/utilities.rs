use std::error::Error;
use std::collections::HashMap;
use std::ffi::OsString;
use std::env::args_os;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, stdout};
use csv::{ReaderBuilder, Writer, Trim};
use rand::{thread_rng, Rng};
use math::round;

// LOCAL
use crate::components::data_structures::{Client, RawClient, Transaction, RawTransaction, TransactionType};

#[allow(dead_code)]
fn generate_deposit() -> RawTransaction {
    RawTransaction {
        transaction_type: TransactionType::Deposit,
        client: 1,
        tx: 1,
        amount: Some(5.0),
    }
}

#[allow(dead_code)]
pub fn generate_test_data() -> Result<(), Box<dyn Error>> {
    let first_transaction = generate_deposit();
    let mut test_txs: Vec<RawTransaction> = vec![first_transaction];
    let mut rng = thread_rng();
    for x in 2..=30 {
        let type_num: u8 = rng.gen_range(1..=5);
        let tx_type = match type_num {
            1 => TransactionType::Withdrawal,
            2 => TransactionType::Deposit,
            3 => TransactionType::Dispute,
            4 => TransactionType::Resolve,
            _ => TransactionType::Chargeback,
        };
        let client_num: u16 = rng.gen_range(1..=3);
        let tx = match tx_type {
            // if it's a dispute, resolve, or chargeback we want to reference other txs
            TransactionType::Dispute => {
                let random_tx = rng.gen_range(0..test_txs.len());
                test_txs[random_tx].tx
            },
            TransactionType::Resolve => {
                let random_tx = rng.gen_range(0..test_txs.len());
                test_txs[random_tx].tx
            }
            TransactionType::Chargeback => {
                let random_tx = rng.gen_range(0..test_txs.len());
                test_txs[random_tx].tx
            }
            _ => x,
        };
        let tx_amount = match tx_type {
            // if it's a dispute, resolve, or chargeback we want to reference other txs
            TransactionType::Withdrawal => {
                let amount: f64 = rng.gen_range(0.0..99999.0);
                Some(round::ceil(amount,4))
            },
            TransactionType::Deposit => {
                let amount: f64 = rng.gen_range(0.0..99999.0);
                Some(round::ceil(amount,4))
            },
            _ => None,
        };
        test_txs.push(
            RawTransaction {
                transaction_type: tx_type, 
                client: client_num,
                tx: tx,
                amount: tx_amount,
            }
        );
    }
    let mut writer = Writer::from_path("test_data.csv")?;
    for tx in test_txs {
        writer.serialize(tx)?;
        writer.flush()?;
    }
    Ok(())
}

// function to parse input filename from command line
pub fn parse_args() -> Result<OsString, Box<dyn Error>> {
    // collect args into vector, I chose OsString to avoid any encoding issues on different platforms
    let args: Vec<OsString> = args_os().collect();
    // the first element is the name of the program itself, the second element is the "first argument" - this is convention in major OSes
    // we check to see if argument is regular file and perform our own error handling with match control flow.
    let file_path = Path::new(&args[1]);
    match file_path.is_file() {
        true => Ok(args[1].clone()),
        false => Err("file does not exist".into()),
    }
}

pub fn output_accounts(client_data: &HashMap<u16, Client>) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_writer(stdout()); // initialize writer to STDOUT
    for client in client_data.values() {
        let raw_client: RawClient = client.into(); // Convert our Client struct into RawClient for writing
        writer.serialize(raw_client)?; // serialize our RawClient struct into a csv record
        writer.flush()?; // "flush" to STDOUT
    }
    Ok(())
}

// first pass at function to process transaction data in chunks
pub fn process_transaction_data(filename: &OsString, client_data: &mut HashMap<u16, Client>) -> Result<(), Box<dyn Error>> {
    // this source could be a TcpStream, etc.
    let transaction_data_file = File::open(filename)?; 
    // default buffer capacity is 8kb -> BufReader streams in 8kb at a time
    let buf_reader = BufReader::new(transaction_data_file); 
    // build custom csv reader with our options
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All) // we use Trim::All to avoid any whitespace issues in the input file
        .from_reader(buf_reader);
    let mut record_num: u32 = 0;
    for record in reader.deserialize() { // this should be ~O(n) where n is the number of Transactions
        record_num += 1;
        // implicit Deserialization from serde
        let raw_transaction: RawTransaction = record?;
        // perform conversion of RawTransaction -> Transaction 
        let transaction: Transaction = raw_transaction.into();
        let client_id = transaction.client;
        // get mutable reference to Client, if it doesn't already exist we create it
        let client = get_or_insert(client_id, client_data)?; // if we have an error here we should probably terminate - something else is going on ;)
        // Do our processing here
        match transaction_handler(client, &transaction) {
            Ok(()) => {
                let mut log_file = OpenOptions::new().append(true).open("transactions.log")?;
                client.transactions.push(transaction); // good transaction - we add it to the list of transactions for that client
                let success_msg = format!("[RECORD #{}][SUCCESS]: Transaction processed successfully.", record_num);
                write!(log_file, "{}\n", success_msg)?;
            },
            Err(error_msg) => { // do something with potential error messages -> write to error.log
                let mut log_file = OpenOptions::new().append(true).open("transactions.log")?;
                write!(log_file, "[RECORD #{}]{}\n", record_num, error_msg)?;
            }, 
        }
    }
    Ok(())
}

// gets mutable ref to Client or inserts new client and gets mutable ref to THAT client
pub fn get_or_insert<'a>(id: u16, client_data: &'a mut HashMap<u16, Client>) -> Result<&'a mut Client, Box<dyn Error>> {
    let error_msg = "[ERROR]: Issue retrieving value from HashMap";
    if client_data.contains_key(&id) {
        client_data.get_mut(&id).ok_or(error_msg.into())
    } else {
        client_data.insert(id, Client::new(id));    
        client_data.get_mut(&id).ok_or(error_msg.into())
    }
}

// entrypoint for different transaction types
pub fn transaction_handler(client: &mut Client, incoming_tx: &Transaction) -> Result<(), Box<dyn Error>> {
    client.is_frozen()?; // Assumption: if client account is frozen we do nothing.
    match incoming_tx.transaction_type {
        TransactionType::Deposit => {
            handle_deposit(client, incoming_tx)?;
        }, 
        TransactionType::Withdrawal => {
            handle_withdrawal(client, incoming_tx)?;
        }
        TransactionType::Dispute => {
            handle_dispute(client, incoming_tx)?;
        }, 
        TransactionType::Resolve => {
            handle_resolve(client, incoming_tx)?;
        }
        TransactionType::Chargeback => {
            handle_chargeback(client, incoming_tx)?;
        }
    }
    Ok(())
}

// function to find withdrawal or deposit referenced by tx_id, returning a mutable reference to that original withdrawal or deposit in Vec<Transaction>
fn find_referenced_transaction<'a>(tx_id: &u32, transactions: &'a mut Vec<Transaction>) -> Result<&'a mut Transaction, Box<dyn Error>> {
    // find referenced transaction, tx_id should match AND we should have Some(amount)
    let referenced_transaction = transactions.iter_mut()
                    .find(|t| t.tx == *tx_id && t.amount.is_some())
                    .ok_or(format!("[ERROR]: Could not find referenced transaction id: {}", tx_id))?;
    Ok(&mut *referenced_transaction)
}
 
// function to handle deposits
fn handle_deposit(client: &mut Client, incoming_tx: &Transaction) -> Result<(), Box<dyn Error>> {
    let amount = incoming_tx.amount.ok_or(format!("[ERROR]: Client id: {}, Transaction id: {} A deposit requires an amount. Discarding transaction.", incoming_tx.client, incoming_tx.tx))?;
    // perform checked add on available balance, in case of overflow
    client.available = client.available.checked_add(amount)
        .ok_or(format!("[ERROR]: Deposit tx: {}, amount: {}, will cause an overflowed (u64::MAX/10e3) account balance for client: {}. Discarding transaction.", incoming_tx.tx, (amount as f64)/10000.0, incoming_tx.client))?;
    // update total
    client.total = client.available + client.held;
    Ok(())
}

// function to handle withdrawals
fn handle_withdrawal(client: &mut Client, incoming_tx: &Transaction) -> Result<(), Box<dyn Error>> {
    let amount = incoming_tx.amount.ok_or(format!("[ERROR]: Client id: {}, Transaction id: {}, A withdrawal requires an amount. Discarding transaction.", incoming_tx.client, incoming_tx.tx))?;
    // perform checked subtract on available balance, in case of overflow
    client.available = client.available.checked_sub(amount)
        .ok_or(format!("[ERROR]: Withdrawal tx: {}, amount: {}, will cause an underflowed (u64) or negative account balance for client: {}. Discarding transaction.", incoming_tx.tx, (amount as f64)/10000.0, incoming_tx.client))?;
    // update total
    client.total = client.available + client.held;
    Ok(())
}

// function to handle disputes
fn handle_dispute(client: &mut Client, incoming_tx: &Transaction) -> Result<(), Box<dyn Error>> {
    let mut referenced_tx = find_referenced_transaction(&incoming_tx.tx, &mut client.transactions)?; 
    // make sure referenced tx isn't already being disputed
    referenced_tx.is_already_disputed()?;
    // make sure referenced tx actually contains an amount
    let amount = referenced_tx.amount
        .ok_or(format!("[ERROR]: Disputed transaction: {} does not have an amount! Discarding transaction.", incoming_tx.tx))?;
    // I make an assumption that different logic is required to dispute a Deposit vs a Withdrawal
    match referenced_tx.transaction_type {
        TransactionType::Deposit => {
            // available funds decrease, perform checked subtraction on available balance, in case of overflow
            client.available = client.available.checked_sub(amount)
                .ok_or(format!("[ERROR]: Dispute on tx: {}, for amount: {}, will cause a underflowed (u64) or negative available balance if disputed. Discarding transaction.", incoming_tx.tx, amount))?;
            // held funds increase, perform checked add on held balance, in case of overflow
            client.held = client.held.checked_add(amount)
                .ok_or(format!("[ERROR]: Dispute on tx: {}, for amount: {}, will cause an overflowed (MAX::u64/10e3) held balance for client {}. Discarding transaction.", incoming_tx.tx, amount, client.client))?;
            // at this point we know we have a valid dispute, so we can go ahead and change the disputed flag, on the referenced tx
            referenced_tx.disputed = true;
            // total funds remain the same
            Ok(())
        },
        TransactionType::Withdrawal => {
            // no change to available funds when disputing a withdrawal
            // held funds increase, perform checked add on held balance, in case of overflow
            client.held = client.held.checked_add(amount)
                .ok_or(format!("[ERROR]: Dispute on tx: {}, for amount: {}, will cause an overflowed (MAX::u64/10e3) held balance for client {}. Discarding transaction.", incoming_tx.tx, amount, client.client))?;
            // at this point we know we have a valid dispute, so we can go ahead and change the disputed flag, on the referenced tx
            referenced_tx.disputed = true;
            // total funds have increased since we are giving a potential refund
            client.total = client.available + client.held;
            Ok(())
        },
        _ => Err(format!("[ERROR]: Cannot dispute any transaction other than a Withdrawal or Deposit").into()),
    } 
}

// function to handle resolutions
fn handle_resolve(client: &mut Client, incoming_tx: &Transaction) -> Result<(), Box<dyn Error>> {
    // find transaction referenced by resolve
    let referenced_tx = find_referenced_transaction(&incoming_tx.tx, &mut client.transactions)?;
    // make sure transaction doesn't already have a resolve
    referenced_tx.is_already_resolved()?;
    // make sure referenced tx actually contains an amount
    let amount = referenced_tx.amount
        .ok_or(format!("[ERROR]: Resolve references a tx: {}, which does not have an amount! Discarding transaction.", incoming_tx.tx))?;
    // check to see if transaction is disputed
    match referenced_tx.disputed {
        // referenced tx IS NOT disputed
        false => Err(format!("[ERROR]: Resolve references a tx: {}, that isn't under dispute. Discarding transaction.", incoming_tx.tx).into()),
        // referenced tx IS disputed
        true => {
            // held funds decrease, checked subraction on held balance, in case of overflow
            client.held = client.held.checked_sub(amount)
                .ok_or(format!("[ERROR]: Resolve on tx: {}, for amount: {}, will cause an underflow (u64) on held balance for client {}. Discarding transaction.", incoming_tx.tx, amount, client.client))?;
            // available funds increase, checked addition on available balance, in case of overflow
            client.available = client.available.checked_add(amount)
                .ok_or(format!("[ERROR]: Resolve on tx: {}, for amount: {}, will cause an overflow (MAX::u64/10e3) on available balance for client {}. Discarding transaction.", incoming_tx.tx, amount, client.client))?;
            Ok(())
        } 
    }
}

// function to handle chargebacks
fn handle_chargeback(client: &mut Client, incoming_tx: &Transaction) -> Result<(), Box<dyn Error>> {
    // find transaction referenced by chargeback
    let referenced_tx = find_referenced_transaction(&incoming_tx.tx, &mut client.transactions)?; 
    // make sure referenced tx doesn't already have a chargeback
    referenced_tx.is_already_resolved()?;
    // make sure referenced tx actually contains an amount
    let amount = referenced_tx.amount
        .ok_or(format!("[ERROR]: Chargeback references tx: {}, which does not have an amount! Discarding transaction", incoming_tx.tx))?;
    // check to see if transaction is disputed
    match referenced_tx.disputed {
        // referenced tx IS NOT disputed
        false => Err(format!("[ERROR]: Chargeback for client: {}, references tx: {}, which isn't under dispute. Discarding transaction.", incoming_tx.client, incoming_tx.tx).into()),
        // referenced tx IS disputed
        true => {
            // held funds decrease, checked subraction on held balance, in case of overflow
            client.held = client.held.checked_sub(amount)
                .ok_or(format!("[ERROR]: Chargeback on tx: {}, for amount: {}, will cause an underflow (u64) on the held balance for client {}. Discarding transaction.", incoming_tx.tx, amount, client.client))?;
            // total funds decrease by the amount subtracted from held
            client.total = client.available + client.held; 
            // at this point we have a valid charge back and have performed the adjustments on the client's held and available funds
            // freeze client's account
            client.locked = true;
            Ok(())
        } 
    }
}