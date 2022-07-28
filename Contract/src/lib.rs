pub mod structs;
pub use crate::structs::*;
mod setters;
pub use crate::setters::*;
use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::collections::{LookupMap, UnorderedMap};

/*
** Initialization
*/

impl Default for Contract {
    fn default() ->Self {
        Self {
            users: UnorderedMap::new(b"a"),
            experience: LookupMap::new(b"m"),
            exp_by_topic: LookupMap::new(b"c"),
            n_exp: 0,
            holdings: 0.0,
            earnings: 0.0,
            ss_wallet: "jciglesias.testnet".parse().unwrap(), //to change wallet
            fee: 0.1
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new(wallet: AccountId, fee: f64) -> Self {
        if env::state_read::<Self>().is_none() != true{
            env::panic_str("<<<Already initialized>>>");
        }
        if env::is_valid_account_id(wallet.as_bytes()) == false {
            env::panic_str("<<<Is not a valid account>>>");
        }
        if (fee > 20.0) || (fee < 0.0) {
            env::panic_str("<<<Fee must be [0.0, 20.0]>>>");
        }
        Self{
            users: UnorderedMap::new(b"a"),
            experience: LookupMap::new(b"m"),
            exp_by_topic: LookupMap::new(b"c"),
            n_exp: 0,
            holdings: 0.0,
            earnings: 0.0,
            ss_wallet: wallet,
            fee: fee
        }
    }

/*
** Deleters
*/

    pub fn delete_experience(&mut self, video_n: u128) {
        let user = env::signer_account_id();
        self.verify_user(user.clone());
        self.verify_exp_status(video_n.clone(), Status::InProcess);
        self.verify_exp_owner(video_n.clone(), user.clone());
        let it = self.users.get(&user.clone()).unwrap().my_exp.to_vec();
        let mut i = 0;
        while it[i] != video_n {
            i += 1;
        }
        self.experience.remove(&video_n.clone());
        self.users.get(&user.clone()).unwrap().my_exp.swap_remove(i);
    }

    pub fn delete_pov(&mut self, video_n: u128) {
        let user = env::signer_account_id();
        self.verify_user(user.clone());
        self.verify_exp_status(video_n.clone(), Status::Active);
        if self.experience.get(&video_n.clone()).unwrap().pov.get(&user.clone()) != None {
            env::panic_str("<<<User has not given a pov for this experience>>>");
        }
        self.experience.get(&video_n.clone()).unwrap().pov.remove(&user.clone());
        let it = self.users.get(&user.clone()).unwrap().pov_exp.to_vec();
        let mut i = 0;
        while it[i] != video_n {
            i += 1;
        }
        self.users.get(&user.clone()).unwrap().pov_exp.swap_remove(i);
    }
/*
** Verifiers
*/
    fn verify_exp(&self, video_n: u128) {
        if self.experience.contains_key(&video_n.clone()) == false {
            env::panic_str("<<<Experience does not exist>>>");
        }
    }

    fn verify_exp_owner(&self, video_n: u128, wallet: AccountId) {
        if self.experience.get(&video_n.clone()).unwrap().owner != wallet {
            env::panic_str("<<<You are not the owner of the experience>>>");
        }
    }

    fn verify_exp_status(&self, video_n: u128, status: Status) {
        self.verify_exp(video_n.clone());
        let exp = self.experience.get(&video_n.clone()).unwrap().status;
        assert_eq!(exp, status, "<<<Experience number {} not {:?}>>>", video_n, status);
    }

    fn verify_user(&self, wallet: AccountId) {
        if self.users.get(&wallet.clone()) == None {
            env::panic_str("<<<No user for this wallet>>>");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};

    fn get_context(wallet: &str, deposit: u128, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "jane.testnet".parse().unwrap(),
            signer_account_id: wallet.parse().unwrap(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "bob.testnet".parse().unwrap(),
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: deposit,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            view_config: None,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    fn set_new_user(contract: &mut Contract, name: String) {
        (*contract).set_user(
            name.clone(),
            name.clone() + "discord",
            name.clone() + "mail",
            8
        );
    }

    fn add_exp(contract: &mut Contract) {
        (*contract).set_experience(
            "exp name".to_string(),
            "exp description".to_string(),
            "url".to_string(),
            // 100.0,
            "moment".to_string(),
            100,
            150,
            3
        );
    }

    fn add_pov(contract: &mut Contract, wallet: &str, vid: u128) {
        (*contract).set_pov(vid, wallet.to_string() + " pov", 150);
    }

    #[test]
    fn create_users() {
        let mut context = get_context("test.tesnet", 0, 0);
        let mut contract = Contract::new("jane.testnet".parse().unwrap(), 20.0);
        context.signer_account_id = "pepe.testnet".parse().unwrap();
        testing_env!(context.clone());
        set_new_user(&mut contract, "pepe".to_string());
        context.signer_account_id = "bob.testnet".parse().unwrap();
        testing_env!(context.clone());
        set_new_user(&mut contract, "bob".to_string());
    }

    #[test]
    fn create_experience() {
        let mut context = get_context("test.tesnet", 0, 0);
        let mut contract = Contract::new("jane.testnet".parse().unwrap(), 1.0);
        context.signer_account_id = "pepe.testnet".parse().unwrap();
        testing_env!(context.clone());
        set_new_user(&mut contract, "pepe".to_string());
        add_exp(&mut contract);
        context.signer_account_id = "bob.testnet".parse().unwrap();
        testing_env!(context.clone());
        set_new_user(&mut contract, "bob".to_string());
        add_exp(&mut contract);
    }

    #[test]
    fn test_activate_exp() {
        let mut contract = Contract::new("jane.testnet".parse().unwrap(), 1.0);
        let context = get_context("pepe.testnet", 110 * YOCTO_NEAR, 0);
        testing_env!(context);
        set_new_user(&mut contract, "pepe".to_string());
        add_exp(&mut contract);
        // context.attached_deposit = 110 * YOCTO_NEAR;
        // testing_env!(context.clone());
        // contract.activate_experience(1);
        println!("{:?}", contract.get_experience(1));
    }

    #[test]
    fn create_pov() {
        let mut context = get_context("test.tesnet", 0, 0);
        let mut contract = Contract::new("jane.testnet".parse().unwrap(), 1.0);
        context.signer_account_id = "pepe.testnet".parse().unwrap();
        context.attached_deposit = (100.0 * contract.fee) as u128 * YOCTO_NEAR;
        testing_env!(context.clone());
        set_new_user(&mut contract, "pepe".to_string());
        for _n in 1..10 {
            add_exp(&mut contract);
        }
        context.signer_account_id = "bob.testnet".parse().unwrap();
        context.attached_deposit = (100.0 * contract.fee) as u128 * YOCTO_NEAR;
        testing_env!(context.clone());
        set_new_user(&mut contract, "bob".to_string());
        for n in 1..10 {
            add_exp(&mut contract);
            add_pov(&mut contract, "bob.testnet", n.clone());
        }
        context.signer_account_id = "pepe.testnet".parse().unwrap();
        testing_env!(context.clone());
        contract.get_number_of_experiences();
        contract.get_experience(11);
    }

    #[test]
    fn test_set_exp_expire_date() {
        let mut context = get_context("test.tesnet", 0, 0);
        let mut contract = Contract::new("jane.testnet".parse().unwrap(), 1.0);
        context.signer_account_id = "pepe.testnet".parse().unwrap();
        testing_env!(context.clone());
        set_new_user(&mut contract, "pepe".to_string());
        for _n in 1..10 {
            add_exp(&mut contract);
        }
        contract.set_experience_expire_date(1, 300);
        println!("{:?}", contract.get_exp_status(1));
        println!("{:?}", contract.get_exp_by_topic(3));
    }
}