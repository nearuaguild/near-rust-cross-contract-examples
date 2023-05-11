use near_sdk::{env, log, near_bindgen, AccountId, Balance, PromiseError};

use crate::{Contract, ContractExt};

#[near_bindgen]
impl Contract {
  // private macro is mandatory here for security concerns
  // https://docs.near.org/sdk/rust/contract-interface/private-methods
  #[private]
  // Return amount of yoctoNear that was sent as a gift
  pub fn claim_gift_callback(&mut self, receiver_id: AccountId, amount: Balance, #[callback_result] transfer_result: Result<(), PromiseError>) -> u128 {
    // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
    assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

    match transfer_result {
      Err(_e) => {
          log!(format!("Failed to transfer {amount} yoctoNear gift to {receiver_id}"));
          
          // removing receiver from the set because he hasn't received any funds
          // so he can try again to claim gift
          self.accounts_claimed_gift.remove(&receiver_id);

          0
      }
      Ok(_) => {
        log!(format!("Successfully sent {amount} yoctoNear gift to {receiver_id}"));

        amount.into()
      }
    }
  }
}