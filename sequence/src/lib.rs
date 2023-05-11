use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, Promise, PanicOnDefault};

mod callback;

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(money_contract)]
trait MoneyContract {
  fn deposit_funds(&self, amount: u128);
  fn withdraw_funds(&self, amount: u128);
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
  pub fn execute_some_actions_with_funds(&mut self) -> Promise {
    let cross_contract_gas = Gas(5 * TGAS);

    let deposit_promise = money_contract::ext(self.contract_address.clone())
      .with_static_gas(cross_contract_gas)
      .deposit_funds(10);
    
    let withdraw_promise = money_contract::ext(self.contract_address.clone())
    .with_static_gas(cross_contract_gas)
    .deposit_funds(50);

    let callback_promise =
      Self::ext(env::current_account_id())
      .with_static_gas(cross_contract_gas)
      .execute_some_actions_with_funds_callback();

    // the order of calls is important for us
    // so we need to call promises sequentially
    deposit_promise.then(withdraw_promise).then(callback_promise)
  }
}