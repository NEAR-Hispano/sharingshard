#[path = "./structs.rs"]
pub mod structs;
pub use crate::structs::*;
use near_sdk::{AccountId, near_bindgen};
use std::collections::HashMap;

/*
** Getters
*/

#[near_bindgen]
impl Contract {
    pub fn get_experience(&self, video_n: u128) ->Experience {
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n.clone()).unwrap()
    }

    pub fn get_user(&self, wallet: AccountId) ->User {
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap()
    }

    pub fn get_title(&self, video_n: u128) ->String {
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n.clone()).unwrap().title
    }

    pub fn get_exp_owner(&self, video_n: u128) ->AccountId {
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n.clone()).unwrap().owner
    }

    pub fn get_exp_description(&self, video_n: u128) -> String {
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n.clone()).unwrap().description
    }

    pub fn get_url(&self, video_n: u128) -> String {
        self.verify_exp(video_n.clone());
        let exp = self.experience.get(&video_n.clone()).unwrap();
        exp.url
    }

    pub fn get_topic(&self, video_n: u128) -> u8 {
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n.clone()).unwrap().topic
    }

    pub fn get_reward(&self, video_n: u128) -> f64 {
        self.verify_exp(video_n.clone());
        (self.experience.get(&video_n.clone())).unwrap().reward
    }

    pub fn get_expiration_date(&self, video_n: u128) ->i64 {
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n).unwrap().exp_date
    }

    pub fn get_moment_coment(&self, video_n: u128) ->String {
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n).unwrap().moment
    }

    pub fn get_moment_time(&self, video_n: u128) ->u16 {
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n).unwrap().time
    }

    pub fn get_pov_of_vid(&self, video_n: u128) ->HashMap<AccountId,String> {
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n).unwrap().pov
    }

    pub fn get_exp_status(&self, video_n: u128) ->Status {
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n).unwrap().status
    }

    pub fn get_exp_by_topic(&self, topic: u8) ->Vec<u128> {
        self.exp_by_topic.get(&topic).unwrap()
    }

    pub fn get_user_name(&self, wallet: AccountId) ->String {
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().name
    }

    pub fn get_user_discord(&self, wallet: AccountId) ->String {
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().discord
    }

    pub fn get_user_email(&self, wallet: AccountId) ->String {
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().email
    }

    pub fn get_user_interests(&self, wallet: AccountId) ->u8 {
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().interests
    }

    pub fn get_user_exp(&self, wallet: AccountId) ->Vec<u128> {
        self.verify_user(wallet.clone());
        let usr = self.users.get(&wallet.clone()).unwrap();
        usr.my_exp.to_vec()
    }

    pub fn get_user_exp_pov(&self, wallet: AccountId) ->Vec<u128> {
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().pov_exp.to_vec()
    }

    pub fn get_user_date(&self, wallet: AccountId) ->i64 {
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().date
    }

    pub fn get_number_of_experiences(&self) ->u128 {
        self.n_exp
    }

    pub fn get_fee(&self) ->f64 {
        (self.fee - 1.0) * 100.0
    }

    pub fn get_list_of_users(&self) ->Vec<AccountId> {
        self.users.keys_as_vector().to_vec()
    }

    pub fn get_winner(&self, video_n: u128) ->String {
        self.experience.get(&video_n).unwrap().winner
    }
}