#[derive(Debug)]
pub struct Memory {
    pub limit: String,
    // reservation: String,
    pub swap: String,
    // swappiness: String,
}
impl Default for Memory {
    fn default() -> Self {
        Self {
            limit: String::from("max"),
            swap: String::from("0"),
        }
    }
}
impl Memory {
    pub fn set_limit(&mut self, limit: String) {
        self.limit = limit;
    }
    pub fn set_swap(&mut self, swap: String) {
        self.swap = swap;
    }
}
