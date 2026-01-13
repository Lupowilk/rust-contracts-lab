use near_sdk::near;

#[near(contract_state)]
#[derive(Default)]
struct Counter {
    count: u64
}

#[near]
impl Counter {
    pub fn increment(&mut self) {
        self.count = self.count +1
    }

    pub fn decrement(&mut self) {
        self.count = self.count -1
    }


    pub fn get_count(&self) -> u64 {
        self.count
    }
}
