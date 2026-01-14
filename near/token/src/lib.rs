use near_sdk::{near, AccountId};
use std::collections::HashMap;

#[near(contract_state)]
#[derive(Default)]
struct TokenContract {
    balances: HashMap<AccountId, u128>,
    token_supply: u128
}

#[near]
impl TokenContract {

}
