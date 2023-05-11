use near_sdk::serde_json::json;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, Balance, Promise, PanicOnDefault};

mod callback;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: Balance = 0;

#[ext_contract(message_contract)]
trait MessageContract {
  fn get_message(&self) -> String;
  fn set_message(&self, message: String);
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
  pub fn get_external_message(&self) -> Promise {
    let cross_contract_gas = Gas(5 * TGAS);

    let get_promise = message_contract::ext(self.contract_address.clone())
      .with_static_gas(cross_contract_gas)
      .get_message();
    
    let callback_promise =
      Self::ext(env::current_account_id())
      .with_static_gas(cross_contract_gas)
      .get_external_message_callback();

    get_promise.then(callback_promise)
  }

  // return here is set to Promise, because actual value will be returned in callback
  pub fn set_external_message(&mut self, message: String) -> Promise {
    let cross_contract_gas = Gas(5 * TGAS);

    let set_promise = message_contract::ext(self.contract_address.clone())
      .with_static_gas(cross_contract_gas)
      .set_message(message);

    let callback_promise =
      Self::ext(env::current_account_id())
      .with_static_gas(cross_contract_gas)
      .set_external_message_callback();

    set_promise.then(callback_promise)
  }

  // return here is set to Promise, because actual value will be returned in callback
  pub fn set_external_message_without_trait(&mut self, message: String) -> Promise {
    let cross_contract_gas = Gas(5 * TGAS);

    let args = json!({"message": message}).to_string().into_bytes();
    let set_promise = Promise::new(self.contract_address.clone()).function_call("set_message".to_owned(), args, NO_DEPOSIT, cross_contract_gas);

    let callback_promise =
      Self::ext(env::current_account_id())
      .with_static_gas(cross_contract_gas)
      .set_external_message_callback();

    set_promise.then(callback_promise)
  }
}