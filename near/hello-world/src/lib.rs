use near_sdk::near;

#[near(contract_state)]
#[derive(Default)]
struct DataContract {
    greeting: String
}

#[near]
impl DataContract {
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    pub fn set_greeting(&mut self, new_greeting: String){
        self.greeting = new_greeting
    }
}
