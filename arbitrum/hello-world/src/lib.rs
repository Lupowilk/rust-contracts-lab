#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;
use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}

#[public]
impl Counter {

    fn number(&self) -> U256 {
        self.number.get()
    }

    fn set_number(&mut self, new_number: U256){
        self.number.set(new_number)
    }

    fn increment(&mut self) {
        let get_number = self.number.get();
        let new_increment = get_number + U256::from(1);
        self.number.set(new_increment)
    }
}

#[cfg(test)]
mod test {
    use stylus_sdk::{testing::TestVM};
    use super::*;

    #[test] // check that the initial value of number is zero.
    fn test_counter() {
        let mock_blockchain = TestVM::default();
        let mock_contract = Counter::from(&mock_blockchain);
        assert_eq!(mock_contract.number(), U256::ZERO);
    }

    #[test]
    fn test_set_number() {
        let mock_blockchain = TestVM::default();
        let mut test_contract = Counter::from(&mock_blockchain);
        test_contract.set_number(U256::from(5));
        assert_eq!(test_contract.number(), U256::from(5));
    }

    #[test]
    fn test_increment() {
        let mock_blockchain = TestVM::default();
        let mut test_contract = Counter::from(&mock_blockchain);
        test_contract.increment();
        assert_eq!(test_contract.number(), U256::from(1));
    }



}
