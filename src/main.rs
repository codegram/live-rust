extern crate rand;

use rand::Rng;
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

const MAX: f64 = 100.0;

#[derive(Debug)]
struct Stats {
    water: Stat,
    food: Stat,
    energy: Stat,
}

#[derive(Debug)]
struct Stat {
    value: f64,
}

impl Stat {
    fn new(value: f64) -> Stat {
        Stat { value }
    }
    fn increase(&mut self, amount: f64) {
        self.value = self.value + amount;
        if self.value > MAX {
            self.value = MAX;
        }
    }
    fn decrease(&mut self, amount: f64) {
        self.value = self.value - amount;
    }
}

fn main() {
    println!("* LIVE *\n\n");

    let now = Instant::now();

    let mut days: i32;
    let mut elapsed_time = now.elapsed().as_secs();

    let mut inventory = Vec::new();

    let mut stats = Stats {
        water: Stat::new(50.0),
        food: Stat::new(50.0),
        energy: Stat::new(50.0),
    };

    loop {
        println!("What to do?");

        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .ok()
            .expect("Failed to read line");

        let current_elapsed_time = now.elapsed().as_secs();
        let seconds = current_elapsed_time - elapsed_time;

        elapsed_time = current_elapsed_time;
        days = elapsed_time as i32 / 60;

        decrease_stats(&mut stats, seconds as f64);

        match action.trim() {
            "help" => println!("Sorry. You are on your own."),
            "sleep" => rest(&mut stats),
            "scavenge" => scavenge(&mut inventory, &mut stats),
            "inventory" => println!("Items in your backpack: {:?}", inventory),
            "stats" => println!("Current stats: {:?}", stats),
            "days" => println!("Days survived so far: {}", days),
            "die" => {
                println!("You died after {} days", days);
                break;
            }
            _ => println!("Invalid input"),
        }

        if stats.energy.value <= 0.0 || stats.food.value <= 0.0 || stats.water.value <= 0.0 {
            println!("You died after {} days", days);
            break;
        }
    }
}

fn scavenge(inv: &mut std::vec::Vec<&str>, stats: &mut Stats) {
    let items = ["water", "berries", "wood", "flint", "string", "clams"];
    let inv_max = 10;

    // const numberOfItems = getters.slotsInInventoryLeft > 3 ? 3 : getters.slotsInInventoryLeft
    let slots_left = inv_max - inv.len();
    let number_of_items = if slots_left < 3 { slots_left } else { 3 };

    if number_of_items == 0 {
        println!("Your inventory is full");
    } else {
        let mut rng = rand::thread_rng();
        sleep(Duration::new(2, 0));
        stats.energy.decrease(5.0);
        for number in 0..number_of_items {
            let random_idx = rng.gen_range(0, items.len() - 1);
            let item = items[random_idx];
            println!("You found {}", item);
            inv.push(item);
        }
    }
}

fn rest(stats: &mut Stats) {
    println!("Sleeping…");
    sleep(Duration::new(2, 0));
    stats.energy.increase(10.0)
}

fn eat(stats: &mut Stats) {
    println!("Eating…");
    stats.food.increase(10.0)
}

fn drink(stats: &mut Stats) {
    println!("Drinking…");
    stats.water.increase(10.0)
}

fn decrease_stats(stats: &mut Stats, seconds: f64) {
    let ratio_energy = 25 as f64 / 60 as f64;
    let ratio_water = 25 as f64 / 60 as f64;
    let ratio_food = 15 as f64 / 60 as f64;

    stats.water.decrease(ratio_water * seconds);
    stats.food.decrease(ratio_food * seconds);
    stats.energy.decrease(ratio_energy * seconds);
}
