mod stable;
mod profile_creation;

use ic_cdk::{export_candid, query, update};
use profile_creation::User;
use stable::{mutate_state, read_state};


#[ic_cdk::update]
pub fn add_user_profile(user_id: String, user: User) {
    mutate_state(|state| {
        state.user_profiles.insert(user_id, user);
    });
}

#[ic_cdk::query]
pub fn get_user_profile(user_id: String) -> Option<User> {
    read_state(|state| state.user_profiles.get(&user_id).clone())
}

export_candid!();
