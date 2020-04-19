#[derive(Default)]
pub struct StillAlive(bool);

impl StillAlive {
    pub fn new() -> StillAlive {
        StillAlive(true)
    }

    pub fn get(&self) -> bool {
        self.0
    }

    pub fn set(&mut self, new_value: bool) {
        self.0 = new_value;
    }
}
