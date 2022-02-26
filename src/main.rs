use std::process;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

// LOCAL
mod components;
use components::utilities::{parse_args, process_transaction_data, output_accounts, generate_test_data};

fn run_payments_engine() -> Result<(), Box<dyn Error>> {
    //generate_test_data()?;
    let input_filename = parse_args()?;
    File::create("transactions.log")?;
    let mut client_data = HashMap::new();
    process_transaction_data(&input_filename, &mut client_data)?;
    output_accounts(&client_data)?;
    Ok(())
}

fn main() {
    match run_payments_engine() {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err.to_string());
            process::exit(1)
        },
    }
}