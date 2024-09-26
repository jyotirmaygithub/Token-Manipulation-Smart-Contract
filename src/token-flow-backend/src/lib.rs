use candid::{CandidType, Deserialize};
use candid::{Decode, Encode, Principal};
use ic_cdk::{export_candid, query, update};
use std::collections::HashMap;

#[derive(CandidType, Debug, Clone)] // Use candid::Encode and candid::Decode
struct User {
    username: String,
    tokens: i32,
}

// Use lazy_static to create a global static HashMap
#[macro_use]
extern crate lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref LEDGER: std::sync::Mutex<HashMap<String, User>> = std::sync::Mutex::new(HashMap::new());
}

#[ic_cdk::update]
fn create_user(principal: String, name: String) {
    let user = User {
        username: name,
        tokens: 0,
    };

    // Insert the user into the ledger
    match LEDGER.lock() {
        Ok(mut ledger) => {
            ledger.insert(principal.clone(), user);
        }
        Err(err) => {
            ic_cdk::println!("Failed to lock the ledger: {:?}", err);
        }
    }
}

#[ic_cdk::query]
fn getting_user() -> HashMap<String, User> {
    let ledger = LEDGER.lock().unwrap();
    ic_cdk::println!(
        "Length of the ledger: {}",ledger.len()
    );
    let mut users: HashMap<String, User> = HashMap::new();
    for (key, user) in ledger.iter() {
        ic_cdk::println!("key {} and value {:?}", key, user);
        users.insert(key.clone(), user.clone());
    }
    users
}

export_candid!();
