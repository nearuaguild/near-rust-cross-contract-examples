use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupSet;
use near_sdk::{env, near_bindgen, AccountId, Gas, Balance, Promise, PanicOnDefault};

mod callback;

pub const TGAS: u64 = 1_000_000_000_000;
pub const GIFT_AMOUNT: Balance = 10_000_000_000_000_000_000_000; // 0.01 Near

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pub accounts_claimed_gift: LookupSet<AccountId>
}

#[near_bindgen]
impl Contract {
  #[init]
  #[private]
  pub fn new() -> Self {
    Self {
      accounts_claimed_gift: LookupSet::new(b"a"),
    }
  }

  pub fn claim_gift(&mut self) -> Promise {
    let receiver_id = env::predecessor_account_id();

    // check if the user claimed a gift before
    if self.accounts_claimed_gift.contains(&receiver_id) {
      env::panic_str("This account already claimed a gift");
    }

    // adding account to the set to prevent users from claiming gift twice
    self.accounts_claimed_gift.insert(&receiver_id);

    let transfer_promise = Promise::new(receiver_id.clone()).transfer(GIFT_AMOUNT);

    let cross_contract_gas = Gas(5 * TGAS); 
    let callback_promise = Self::ext(env::current_account_id()).with_static_gas(cross_contract_gas).claim_gift_callback(
      receiver_id.clone(),
      GIFT_AMOUNT      
  );

    transfer_promise.then(callback_promise)
  }
}