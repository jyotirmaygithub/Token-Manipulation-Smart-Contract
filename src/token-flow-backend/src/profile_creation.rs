use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct User {
   pub username: String,
   pub  tokens: i32,
}
