use near_sdk::near;

#[near(contract_state)]
#[derive(Default)]
struct DataContract {
    greeting: String
};

#[near]
impl DataContract {};
