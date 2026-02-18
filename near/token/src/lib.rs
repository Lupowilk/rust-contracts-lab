use near_sdk::{near, AccountId, env};
use std::collections::HashMap;

#[near(contract_state)]
#[derive(Default)]
struct TokenContract {
    balances: HashMap<AccountId, u128>,
    token_supply: u128
}

#[near]
impl TokenContract {

    #[init]
    pub fn initTokenSupply(token_supply: u128) -> Self {
        let contract_owner = env::signer_account_id();
        let mut balances = HashMap::new();
        balances.insert(contract_owner, token_supply);

        Self {
            balances: balances,
            token_supply: token_supply
        }
    }

    pub fn balance_of(&self, account: AccountId) -> u128 {
        let realBalance = self.balances.get(&account);
        match realBalance {
            Some(value) => *value,
            None => 0
        }
    }

    pub fn total_supply(&self) -> u128 {
        self.token_supply
    }

    pub fn transfer(&mut self, receiver: AccountId, amount: u128 ) {

    }

}
