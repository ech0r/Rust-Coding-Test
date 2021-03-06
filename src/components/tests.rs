use std::path::Path;
use std::collections::HashMap;
use std::ffi::OsString;
use crate::components::utilities::process_transaction_data;
use crate::components::data_structures::{RawClient};

#[test]
pub fn positive_dispute() {
    let file_path: OsString = "test_data/positive_dispute.csv".to_string().into();
    let file = Path::new(&file_path);
    assert_eq!(file.is_file(), true);
    let mut client_data = HashMap::new();
    process_transaction_data(&file_path, &mut client_data).unwrap();
    let test_client1 = RawClient {
        client: 1,
        available: 0.0,
        held: 1.0, 
        total: 1.0,
        locked: false,
    };
    let test_client2 = RawClient {
        client: 2,
        available: 2.0,
        held: 1.0, 
        total: 3.0,
        locked: false,
    };
    let client1: RawClient = client_data.get(&1).unwrap().into();
    let client2: RawClient = client_data.get(&2).unwrap().into();
    assert_eq!(test_client1, client1);
    assert_eq!(test_client2, client2);
} 

#[test]
pub fn negative_dispute() {
    let file_path: OsString = "test_data/negative_dispute.csv".to_string().into();
    let file = Path::new(&file_path);
    assert_eq!(file.is_file(), true);
    let mut client_data = HashMap::new();
    process_transaction_data(&file_path, &mut client_data).unwrap();
    let test_client1 = RawClient {
        client: 1,
        available: 1.5,
        held: 0.0, 
        total: 1.5,
        locked: false,
    };
    let test_client2 = RawClient {
        client: 2,
        available: 2.0,
        held: 0.0, 
        total: 2.0,
        locked: false,
    };
    let client1: RawClient = client_data.get(&1).unwrap().into();
    let client2: RawClient = client_data.get(&2).unwrap().into();
    assert_eq!(test_client1, client1);
    assert_eq!(test_client2, client2);
} 

#[test]
pub fn postive_resolve() {
    let file_path: OsString = "test_data/positive_resolve.csv".to_string().into();
    let file = Path::new(&file_path);
    assert_eq!(file.is_file(), true);
    let mut client_data = HashMap::new();
    process_transaction_data(&file_path, &mut client_data).unwrap();
    let test_client1 = RawClient {
        client: 1,
        available: 5.0,
        held: 0.0, 
        total: 5.0,
        locked: false,
    };
    let test_client2 = RawClient {
        client: 2,
        available: 2.0,
        held: 0.0, 
        total: 2.0,
        locked: false,
    };
    let client1: RawClient = client_data.get(&1).unwrap().into();
    let client2: RawClient = client_data.get(&2).unwrap().into();
    assert_eq!(test_client1, client1);
    assert_eq!(test_client2, client2);
} 

#[test]
pub fn negative_resolve() {
    let file_path: OsString = "test_data/negative_resolve.csv".to_string().into();
    let file = Path::new(&file_path);
    assert_eq!(file.is_file(), true);
    let mut client_data = HashMap::new();
    process_transaction_data(&file_path, &mut client_data).unwrap();
    let test_client1 = RawClient {
        client: 1,
        available: 1.5,
        held: 3.5, 
        total: 5.0,
        locked: false,
    };
    let test_client2 = RawClient {
        client: 2,
        available: 2.0,
        held: 0.0, 
        total: 2.0,
        locked: false,
    };
    let client1: RawClient = client_data.get(&1).unwrap().into();
    let client2: RawClient = client_data.get(&2).unwrap().into();
    assert_eq!(test_client1, client1);
    assert_eq!(test_client2, client2);
} 


#[test]
pub fn positive_chargeback() {
    let file_path: OsString = "test_data/positive_chargeback.csv".to_string().into();
    let file = Path::new(&file_path);
    assert_eq!(file.is_file(), true);
    let mut client_data = HashMap::new();
    process_transaction_data(&file_path, &mut client_data).unwrap();
    let test_client1 = RawClient {
        client: 1,
        available: 80.0,
        held: 0.0, 
        total: 80.0,
        locked: true,
    };
    let client1: RawClient = client_data.get(&1).unwrap().into();
    assert_eq!(test_client1, client1);
} 

#[test]
pub fn negative_chargeback() {
    let file_path: OsString = "test_data/negative_chargeback.csv".to_string().into();
    let file = Path::new(&file_path);
    assert_eq!(file.is_file(), true);
    let mut client_data = HashMap::new();
    process_transaction_data(&file_path, &mut client_data).unwrap();
    let test_client1 = RawClient {
        client: 1,
        available: 1.5,
        held: 0.0, 
        total: 1.5,
        locked: false,
    };
    let test_client2 = RawClient {
        client: 2,
        available: 2.0,
        held: 0.0, 
        total: 2.0,
        locked: false,
    };
    let client1: RawClient = client_data.get(&1).unwrap().into();
    let client2: RawClient = client_data.get(&2).unwrap().into();
    assert_eq!(test_client1, client1);
    assert_eq!(test_client2, client2);
}

#[test]
pub fn withdrawals_and_deposits() {
    let file_path: OsString = "test_data/withdrawals_and_deposits.csv".to_string().into();
    let file = Path::new(&file_path);
    assert_eq!(file.is_file(), true);
    let mut client_data = HashMap::new();
    process_transaction_data(&file_path, &mut client_data).unwrap();
    let test_client1 = RawClient {
        client: 1,
        available: 8.0,
        held: 0.0, 
        total: 8.0,
        locked: false,
    };
    let test_client2 = RawClient {
        client: 2,
        available: 0.0001,
        held: 0.0, 
        total: 0.0001,
        locked: false,
    };
    let client1: RawClient = client_data.get(&1).unwrap().into();
    let client2: RawClient = client_data.get(&2).unwrap().into();
    assert_eq!(test_client1, client1);
    assert_eq!(test_client2, client2);
} 

#[test]
pub fn mixed_test() {
    let file_path: OsString = "test_data/mixed_test.csv".to_string().into();
    let file = Path::new(&file_path);
    assert_eq!(file.is_file(), true);
    let mut client_data = HashMap::new();
    process_transaction_data(&file_path, &mut client_data).unwrap();
    let test_client1 = RawClient {
        client: 1,
        available: 3453.0,
        held: 0.0, 
        total: 3453.0,
        locked: true,
    };
    let test_client2 = RawClient {
        client: 2,
        available: 2.0431,
        held: 0.0, 
        total: 2.0431,
        locked: false,
    };
    let client1: RawClient = client_data.get(&1).unwrap().into();
    let client2: RawClient = client_data.get(&2).unwrap().into();
    assert_eq!(test_client1, client1);
    assert_eq!(test_client2, client2);
} 