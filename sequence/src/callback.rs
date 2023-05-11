use near_sdk::{log, near_bindgen, env, PromiseError};
use crate::{Contract, ContractExt};

#[near_bindgen]
impl Contract {   
  // private macro is mandatory here for security concerns
  // https://docs.near.org/sdk/rust/contract-interface/private-methods
  #[private]
  // Return value indicates whether the action succeeded or not
  pub fn execute_some_actions_with_funds_callback(&mut self, #[callback_result] actions_result: Result<(), PromiseError>) -> bool {
    // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
    assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

    match actions_result {
      Err(_e) => {
          log!(format!("Something went wrong, all Cross-Contract calls got reverted"));

          false
      }
      Ok(_) => {
        log!(format!("Actions successfully executed!"));

        true
      }
    }
  }
}