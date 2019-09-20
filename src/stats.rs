use std::fmt;

const MAX: f64 = 100.0;

#[derive(Debug)]
pub struct Stats {
    pub water: Stat,
    pub food: Stat,
    pub energy: Stat,
    pub health: Stat,
    pub is_sick: bool,
}

pub struct Stat {
    pub value: f64,
}

impl fmt::Debug for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*}", 2, self.value)
    }
}

impl Stat {
    pub fn new(value: f64) -> Stat {
        Stat { value }
    }
    pub fn increase(&mut self, amount: f64) {
        self.value = self.value + amount;
        if self.value > MAX {
            self.value = MAX;
        }
    }
    pub fn decrease(&mut self, amount: f64) {
        self.value = self.value - amount;
    }
}

pub fn decrease_stats(stats: &mut Stats, seconds: f64) {
    let ratio_energy = 25 as f64 / 60 as f64;
    let ratio_water = 25 as f64 / 60 as f64;
    let ratio_food = 15 as f64 / 60 as f64;
    let ratio_health = 30 as f64 / 60 as f64;

    stats.water.decrease(ratio_water * seconds);
    stats.food.decrease(ratio_food * seconds);
    stats.energy.decrease(ratio_energy * seconds);

    if stats.is_sick {
        stats.health.decrease(ratio_health * seconds);
    }
}