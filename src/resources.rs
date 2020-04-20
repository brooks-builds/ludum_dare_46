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

#[derive(Default)]
pub struct BulletSize(f32);

impl BulletSize {
    pub fn new(size: f32) -> BulletSize {
        BulletSize(size)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

#[derive(Default)]
pub struct DelayFiringUntilAfter(u128);

impl DelayFiringUntilAfter {
    pub fn new() -> DelayFiringUntilAfter {
        DelayFiringUntilAfter(0)
    }

    pub fn get(&self) -> u128 {
        self.0
    }

    pub fn set(&mut self, new_time: u128) {
        self.0 = new_time;
    }
}

#[derive(Default)]
pub struct Score(usize);

impl Score {
    pub fn new() -> Score {
        Score(0)
    }

    pub fn get(&self) -> usize {
        self.0
    }

    pub fn increase(&mut self, amount: usize) {
        self.0 += amount;
    }
}
