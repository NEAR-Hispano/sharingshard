#[path = "./enumerations.rs"]
pub mod enumerations;
#[path = "./structs.rs"]
mod structs;
pub use crate::structs::*;
pub use crate::enumerations::*;
use near_sdk::{env, Promise, Balance, AccountId, near_bindgen};
use std::collections::HashMap;
pub const YOCTO_NEAR: Balance = 1_000_000_000_000_000_000_000_000;
pub const SEND_FUNDS: Balance = 4_500_000_000_000_000_000;
//https://docs.near.org/docs/concepts/storage-staking
//const STORAGE_PER_BYTE: Balance = 10_000_000_000_000_000_000;
pub const FEE: f64 = 1.1;

fn send_fee(receiver: AccountId, deposit: Balance, reward: f64, wallet: AccountId) { //make priv
    let fee = ((reward * FEE) - reward) as u128 * YOCTO_NEAR;
    Promise::new(receiver.clone()).transfer(fee);
    let diff = deposit - ((reward as u128 * YOCTO_NEAR) + fee);
    if diff > SEND_FUNDS{
        Promise::new(wallet).transfer(diff);
    }
}


#[near_bindgen]
impl Contract {
    pub fn set_user(
        &mut self,
        name: String,
        discord: String,
        email: String,
        interests: u8) {
        let wallet = env::signer_account_id();
        assert!(!self.users.contains_key(&wallet.clone()), "User already exists");
        self.users.insert(&wallet.clone(), &User{name: name,
            discord: discord,
            email: email,
            interests: interests,
            my_exp: Vec::new(),
            pov_exp: Vec::new(),
            date: 0});
    }

    #[payable]
    pub fn set_experience(&mut self,
        experience_name: String,
        description: String,
        url: String,
        reward: f64,
        moment: String,
        time: u16,
        expire_date: i64,
        topic: u8) ->u128 {
        self.verify_user(env::signer_account_id());
        let mut stat = Status::InProcess;
        if env::attached_deposit() > 0 {
            assert!(env::attached_deposit() >= ((reward * FEE) as u128 * YOCTO_NEAR),
            "Wrong amount of NEARs");
            send_fee(self.ss_wallet.clone(), env::attached_deposit(), reward.clone(),
            env::signer_account_id());
            stat = Status::Active;
        }
        self.n_exp += 1;
        self.experience.insert(&self.n_exp.clone(),
        &Experience{title: experience_name.clone(),
            owner: env::signer_account_id(),
            description: description,
            url: url,
            reward: reward,
            moment: moment,
            time : time,
            pov: HashMap::new(),
            topic: topic.clone(),
            exp_date: expire_date,
            status: stat});
        let mut vec;
        if !self.exp_by_topic.contains_key(&topic.clone()) {
            self.exp_by_topic.insert(&topic.clone(), &Vec::new());
        }
        vec = self.exp_by_topic.get(&topic.clone()).unwrap();
        vec.push(self.n_exp.clone());
        self.exp_by_topic.insert(&topic.clone(), &vec);
        let mut usr = self.users.get(&env::signer_account_id()).unwrap();
        usr.my_exp.push(self.n_exp.clone());
        self.users.insert(&env::signer_account_id(), &usr);
        self.n_exp
    }

    pub fn set_user_discord(&mut self, discord: String) {
        let wallet = env::signer_account_id();
        self.verify_user(wallet.clone());
        let mut user = self.users.get(&wallet.clone()).unwrap();
        user.discord = discord;
        self.users.insert(&wallet, &user);
    }

    pub fn set_user_email(&mut self, email: String) {
        let wallet = env::signer_account_id();
        self.verify_user(wallet.clone());
        let mut user = self.users.get(&wallet.clone()).unwrap();
        user.email = email;
        self.users.insert(&wallet, &user);
    }

    pub fn set_user_interests(&mut self, interests: u8) {
        let wallet = env::signer_account_id();
        self.verify_user(wallet.clone());
        let mut user = self.users.get(&wallet.clone()).unwrap();
        user.interests = interests;
        self.users.insert(&wallet, &user);
    }

    pub fn set_user_name(&mut self, name: String) {
        let wallet = env::signer_account_id();
        self.verify_user(wallet.clone());
        let mut user = self.users.get(&wallet.clone()).unwrap();
        user.name = name;
        self.users.insert(&wallet, &user);
    }

    pub fn set_moment_comment(&mut self, video_n: u128, comment: String) {
        self.verify_exp(video_n.clone());
        self.verify_exp_owner(video_n.clone(), env::signer_account_id());
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        exp.moment = comment;
        self.experience.insert(&video_n.clone(), &exp);
    }

    pub fn set_moment_time(&mut self, video_n: u128, time: u16) {
        self.verify_exp(video_n.clone());
        self.verify_exp_owner(video_n.clone(), env::signer_account_id());
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        exp.time = time;
        self.experience.insert(&video_n.clone(), &exp);
    }

    pub fn set_experience_description(&mut self, video_n: u128, description: String) {
        self.verify_exp(video_n.clone());
        self.verify_exp_owner(video_n.clone(), env::signer_account_id());
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        exp.description = description;
        self.experience.insert(&video_n.clone(), &exp);
    }

    pub fn set_experience_expire_date(&mut self, video_n: u128, date: i64) {
        self.verify_exp(video_n.clone());
        self.verify_exp_owner(video_n.clone(), env::signer_account_id());
        assert_eq!(self.experience.get(&video_n.clone()).unwrap().status,
        Status::InProcess, "Experience not in process");
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        exp.exp_date = date;
        self.experience.insert(&video_n.clone(), &exp);
    }

    pub fn set_pov(&mut self, video_n: u128, pov: String, date: i64) {
        let wallet = env::signer_account_id();
        self.verify_exp_status(video_n.clone(), Status::Active);
        self.verify_user(wallet.clone());
        assert_ne!(self.experience.get(
            &video_n.clone()).unwrap().owner.clone(),
            wallet.clone(),
            "You can't put a pov in your own experience"
        );
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        assert_eq!(exp.pov.get(&wallet.clone()), None,
        "User has already given a pov for this experience");
        exp.pov.insert(wallet.clone(), pov);
        self.experience.insert(&video_n.clone(), &exp);
        let mut usr = self.users.get(&wallet.clone()).unwrap();
        usr.pov_exp.push(video_n.clone());
        usr.date = date;
        self.users.insert(&wallet.clone(), &usr);
    }

/*
** Transactions
*/

    pub fn pay_reward(&mut self, experience_number: u128, wallet: AccountId) {
        let caller = env::signer_account_id();
        self.verify_exp_owner(experience_number.clone(), caller.clone());
        assert_eq!(self.get_exp_status(experience_number.clone()),
        Status::Active, "Experience not active");
        assert_ne!(self.experience.get(
            &experience_number.clone()).unwrap().pov.get(&wallet.clone()),
            None,
            "{} did not give a PoV for this experience", wallet.clone());
        Promise::new(wallet).transfer(
            (self.get_reward(experience_number.clone()) as Balance)
            * YOCTO_NEAR);
        let mut exp = self.experience.get(&experience_number.clone()).unwrap();
        exp.status = Status::Closed;
        self.experience.insert(&experience_number.clone() , &exp);
    }

    #[payable]
    pub fn activate_experience(&mut self, video_n: u128) {
        self.verify_user(env::signer_account_id());
        self.verify_exp(video_n.clone());
        assert_eq!(self.experience.get(&video_n.clone()).unwrap().status,
        Status::InProcess, "Experience already activated");
        self.verify_exp_owner(video_n.clone(), env::signer_account_id());
        let reward = self.experience.get(&video_n.clone()).unwrap().reward.clone();
        assert!(env::attached_deposit() >= ((reward * FEE) as u128 * YOCTO_NEAR),
        "Not enough tokens");
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        exp.status = Status::Active;
        self.experience.insert(&video_n.clone(), &exp);
        send_fee(self.ss_wallet.clone() , env::attached_deposit(), reward.clone(),
        env::signer_account_id());
    }
}