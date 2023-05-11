use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, Promise, PanicOnDefault};

mod callback;

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(profile_contract)]
trait ProfileContract {
  fn set_name(&self, name: String);
  fn set_age(&self, age: u8);
  fn set_title(&self, title: String);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  pub contract_address: AccountId
}

#[near_bindgen]
impl Contract {
  #[init]
  #[private]
  pub fn new(contract_address: AccountId) -> Self {
    Self {
      contract_address,
    }
  }

  // return here is set to Promise, because actual value will be returned in callback
  pub fn update_bio(&mut self, name: String, age: u8, title: String) -> Promise {
    let cross_contract_gas = Gas(5 * TGAS);

    let set_name_promise = profile_contract::ext(self.contract_address.clone())
      .with_static_gas(cross_contract_gas)
      .set_name(name.clone());

    let set_age_promise = profile_contract::ext(self.contract_address.clone())
      .with_static_gas(cross_contract_gas)
      .set_age(age.clone());

    let set_title_promise = profile_contract::ext(self.contract_address.clone())
      .with_static_gas(cross_contract_gas)
      .set_title(title.clone());

    // we don't care about order of calls
    // so we'd call them in parallel by using AND 
    let set_union = set_name_promise.and(set_age_promise).and(set_title_promise);

    let callback_promise =
      Self::ext(env::current_account_id())
      .with_static_gas(cross_contract_gas)
      .update_bio_callback();

    set_union.then(callback_promise)
  }
}