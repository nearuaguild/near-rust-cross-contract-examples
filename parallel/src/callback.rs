use near_sdk::{log, near_bindgen, env, PromiseResult};
use crate::{Contract, ContractExt};

#[near_bindgen]
impl Contract {   
  // private macro is mandatory here for security concerns
  // https://docs.near.org/sdk/rust/contract-interface/private-methods
  #[private]
  // Return array of bools indicates whether the action succeeded or not
  pub fn update_bio_callback(&mut self) -> [bool; 3] {
    // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
    assert_eq!(env::promise_results_count(), 3, "ERR_TOO_MANY_RESULTS");

    (0..3).map(|index| {
      // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_result.html
      let result = env::promise_result(index);

      match result {
        PromiseResult::Failed => {
            log!(format!("Promise num {index} failed"));
            false
        }
        PromiseResult::NotReady => {
            log!(format!("Promise num {index} is not ready yet"));
            false
        }
        PromiseResult::Successful(_) => {
            true
        }
    }
    }).collect::<Vec<bool>>().try_into().unwrap()
  }
}