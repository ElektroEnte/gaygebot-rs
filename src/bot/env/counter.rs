#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Counter {
    value: i64,
}

impl Default for Counter {
    fn default() -> Self {
        Counter { value: 0 }
    }
}

impl From<i64> for Counter {
    fn from(i: i64) -> Self {
        Counter { value: i }
    }
}

impl Counter {
    pub fn new(value: i64) -> Self {
        Counter { value }
    }

    pub fn get(&self) -> i64 {
        self.value
    }

    pub fn set(&mut self, value: i64) {
        self.value = value
    }

    pub fn add(&mut self, value: i64) {
        self.set(self.get() + value);
    }

    pub fn subtract(&mut self, value: i64) {
        self.set(self.get() - value)
    }
}