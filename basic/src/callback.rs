use near_sdk::{log, near_bindgen, PromiseError};

use crate::{Contract, ContractExt};

#[near_bindgen]
impl Contract {
  // private macro is mandatory here for security concerns
  // https://docs.near.org/sdk/rust/contract-interface/private-methods
  #[private]
  pub fn get_external_message_callback(&self, #[callback_result] view_result: Result<String, PromiseError>) -> Option<String> {
    match view_result {
        Err(_e) => {
            log!("Something went wrong while trying to get_external_message");
            None
        }
        Ok(msg) => {
            Some(msg)
        }
    }
  }
    
  // private macro is mandatory here for security concerns
  // https://docs.near.org/sdk/rust/contract-interface/private-methods
  #[private]
  // Return value indicates whether the action succeeded or not
  pub fn set_external_message_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>) -> bool {
    match call_result {
        Err(_e) => {
            log!("Failed to set_message");
            false
        }
        Ok(_) => {
            log!("set_message suceeded!");
            true
        }
    }
  }
}