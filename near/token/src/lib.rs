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
    pub fn init_token_supply(token_supply: u128) -> Self {
        let contract_owner = env::signer_account_id();
        let mut balances = HashMap::new();
        balances.insert(contract_owner, token_supply);
        Self {
            balances: balances,
            token_supply: token_supply
        }
    }

    pub fn balance_of(&self, account: &AccountId) -> u128 {
        let real_balance = self.balances.get(account);
        match real_balance {
            Some(value) => *value,
            None => 0
        }
    }

    pub fn total_supply(&self) -> u128 {
        self.token_supply
    }

    pub fn transfer(&mut self, receiver: AccountId, amount: u128) {
        let sender = env::signer_account_id();
        let sender_balance = self.balance_of(&sender);
        let net_balance = sender_balance - amount;
        assert!(sender_balance >= amount, "Not enough balance");
        self.balances.insert(sender, net_balance);
        let receiver_balance = self.balances.get(&receiver);
        let receiver_current_balance = match receiver_balance {
            Some(value) => *value,
            None => 0
        };
        let receiver_final_balance = receiver_current_balance + amount;
        self.balances.insert(receiver, receiver_final_balance);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    #[test]
    fn test_initial_supply() {
        let mut blockchain = VMContextBuilder::new();
        blockchain.signer_account_id("bobo.near".parse().unwrap());
        testing_env!(blockchain.build());
        let token_contract = TokenContract::init_token_supply(1917);
        assert_eq!(token_contract.total_supply(), 1917);
    }

    #[test]
    fn test_sender_balance_decreased() {
        let mut blockchain = VMContextBuilder::new();
        blockchain.signer_account_id("bobo.near".parse().unwrap());
        testing_env!(blockchain.build());
        let mut token_contract = TokenContract::init_token_supply(1917);
        assert_eq!(token_contract.balance_of(&"bobo.near".parse().unwrap()), 1917);
        token_contract.transfer("alice.near".parse().unwrap(), 500);
        assert_eq!(token_contract.balance_of(&"bobo.near".parse().unwrap()), 1917-500);
        assert_eq!(token_contract.balance_of(&"alice.near".parse().unwrap()), 500);
        assert_eq!(token_contract.total_supply(), 1917);
    }

}
