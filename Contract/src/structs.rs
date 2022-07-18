use near_sdk::{near_bindgen, AccountId, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/*
** Enums
*/

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
pub enum Status {
    InProcess,
    Active,
    Closed
}

/*
** Structures
*/

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct Experience{
    pub title: String,
    pub owner: AccountId,
    pub description: String,
    pub url: String,
    pub topic: u8,
    pub reward: f64,
    pub exp_date: i64,
    pub moment: String,
    pub time: u16,
    pub pov: HashMap<AccountId, String>,
    pub winner: String,
    pub status: Status,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq)]
pub struct User{
    pub name: String,
    pub discord: String,
    pub email: String,
    pub interests: u8,
    pub my_exp: Vec<u128>,
    pub pov_exp: Vec<u128>,
    pub date: i64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract{
    pub users: UnorderedMap<AccountId, User>,
    pub experience: LookupMap<u128, Experience>,
    pub exp_by_topic: LookupMap< u8, Vec<u128> >,
    pub n_exp: u128,
    pub holdings: f64,
    // pub ss_wallet: AccountId,
    pub fee: f64,
}