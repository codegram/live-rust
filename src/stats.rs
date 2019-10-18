use std::fmt;
use wasm_bindgen::prelude::*;

const MAX: f64 = 100.0;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Stats {
    pub water: Stat,
    pub food: Stat,
    pub energy: Stat,
    pub health: Stat,
    pub is_sick: bool,
}
#[wasm_bindgen]
impl Stats {
    pub fn new() -> Stats {
        Stats {
            water: Stat::new(100.0),
            food: Stat::new(100.0),
            energy: Stat::new(100.0),
            health: Stat::new(100.0),
            is_sick: false,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Stat {
    pub value: f64,
}

impl fmt::Debug for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*}", 2, self.value)
    }
}

#[wasm_bindgen]
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

#[wasm_bindgen]
pub fn decrease_stats(stats: &mut Stats, seconds: f64) {
    let ratio_energy = 20 as f64 / 60 as f64;
    let ratio_water = 20 as f64 / 60 as f64;
    let ratio_food = 10 as f64 / 60 as f64;
    let ratio_health = 30 as f64 / 60 as f64;

    stats.water.decrease(ratio_water * seconds);
    stats.food.decrease(ratio_food * seconds);
    stats.energy.decrease(ratio_energy * seconds);

    if stats.is_sick {
        stats.health.decrease(ratio_health * seconds);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrease_stats() {
        let mut stats = Stats {
            water: Stat::new(100.0),
            food: Stat::new(100.0),
            energy: Stat::new(100.0),
            health: Stat::new(100.0),
            is_sick: false,
        };
        decrease_stats(&mut stats, 60.0);
        assert_eq!(stats.water.value, 80.0);
        assert_eq!(stats.food.value, 90.0);
        assert_eq!(stats.energy.value, 80.0);
        assert_eq!(stats.health.value, 100.0);
    }
}
