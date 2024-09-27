mod profile_creation;
mod stable;

use core::num;
use std::sync::mpsc::Receiver;

use ic_cdk::{export_candid, query, update};
use profile_creation::User;
use stable::{mutate_state, read_state};

#[update]
pub fn admin_profile(user: User) {
    let caller = ic_cdk::api::caller();
    if get_user_profile(caller.to_string()).is_some() {
        // If the user already exists, you can either return an error or handle it
        panic!("User with ID {} already exists!", caller);
    } else {
        // If the user does not exist, insert the new user profile
        mutate_state(|state| {
            state.user_profiles.insert(caller.to_string(), user);
        });
    }
}

#[update]
pub fn add_user_profile(user_id: String, user: User) {
    // Check if a user profile with the given user_id already exists
    if get_user_profile(user_id.clone()).is_some() {
        // If the user already exists, you can either return an error or handle it
        panic!("User with ID {} already exists!", user_id);
    } else {
        // If the user does not exist, insert the new user profile
        mutate_state(|state| {
            state.user_profiles.insert(user_id, user);
        });
    }
}

#[update]
pub fn assign_tokens(user_id: String, amount: i32) {
    let caller = ic_cdk::api::caller();

    let mut admin_profile =
        get_user_profile(caller.to_string()).expect("Admin does not exist to assign tokens!");
    let mut user_profile =
        get_user_profile(user_id.clone()).expect("User with ID does not exist to assign tokens!");

    if admin_profile.tokens < amount {
        panic!(
            "Admin does not have enough tokens to assign {} tokens!",
            amount
        );
    }

    // Update balances
    admin_profile.tokens -= amount; // Deduct from admin
    user_profile.tokens += amount; // Add to user

    // Update the state with the new profiles
    mutate_state(|state| {
        state
            .user_profiles
            .insert(caller.to_string(), admin_profile);
        state.user_profiles.insert(user_id.clone(), user_profile);
    });
}

#[update]
pub fn minter() {
    let minted_tokens = 100000;
    let caller = ic_cdk::api::caller();
    let mut admin_profile =
        get_user_profile(caller.to_string()).expect("Admin does not exist to assign tokens!");
    admin_profile.tokens += minted_tokens;

    // Update the state with the new minted tokens.
    mutate_state(|state| {
        state
            .user_profiles
            .insert(caller.to_string(), admin_profile);
    });
}

#[update]
pub fn transfer_tokens(sender: String, reciver: String, number_tokens: i32) {
    let mut sender_profile = get_user_profile(sender.clone()).expect("sender does not exist!");
    let mut receiver_profile = get_user_profile(reciver.clone()).expect("reciver does not exist!");

    if sender_profile.tokens < number_tokens {
        panic!("sender does not have sufficenet amount of tokens to send");
    }

    sender_profile.tokens -= number_tokens;
    receiver_profile.tokens += number_tokens;

    // Update the state with the new tokens.
    mutate_state(|state| {
        state.user_profiles.insert(sender, sender_profile);
        state.user_profiles.insert(reciver, receiver_profile);
    });
}

#[update]
pub fn burn_tokens(burner_id: String, number_tokens: i32) {
    let mut burner_profile = get_user_profile(burner_id.clone()).expect("user does not exist!");
    if burner_profile.tokens < number_tokens {
        panic!("user does not have sufficenet amount of tokens to burn");
    }

    burner_profile.tokens -= number_tokens;

    // Update the state with the new tokens.
    mutate_state(|state| {
        state.user_profiles.insert(burner_id, burner_profile);
    });
    
}

#[query]
pub fn get_user_profile(user_id: String) -> Option<User> {
    let caller = ic_cdk::api::caller();
    ic_cdk::println!("pricipal id = {}", caller);
    read_state(|state| state.user_profiles.get(&user_id).clone())
}

export_candid!();
