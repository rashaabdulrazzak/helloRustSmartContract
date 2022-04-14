use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn say_hello(self) -> String {
        return "Hello".to_string();
    }
    pub fn say_hello_name(self, name: String) -> String {
        return "Hello".to_string() + " " + &name;
    }
}
