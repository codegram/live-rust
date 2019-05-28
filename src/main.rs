extern crate rand;

use rand::Rng;
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

const MAX: i32 = 100;

#[derive(Debug)]
struct Stats {
    water: Stat,
    food: Stat,
    energy: Stat,
}

#[derive(Debug)]
struct Stat {
    value: i32,
}

impl Stat {
    fn new(value: i32) -> Stat {
        Stat { value }
    }
    fn increase(&mut self, amount: i32) {
        self.value = self.value + amount;
        if self.value > MAX {
            self.value = MAX;
        }
    }
    fn decrease(&mut self, amount: i32) {
        self.value = self.value - amount;
    }
}

fn main() {
    println!("* LIVE *\n\n");

    let now = Instant::now();

    let mut days: i32;
    let mut elapsed_time: f64;

    let mut inventory = Vec::new();

    let mut stats = Stats {
        water: Stat::new(50),
        food: Stat::new(50),
        energy: Stat::new(50),
    };

    loop {
        println!("What to do?");

        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .ok()
            .expect("Failed to read line");

        elapsed_time = now.elapsed().as_secs() as f64;
        days = (elapsed_time / 60.0).floor() as i32;

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

        if stats.energy.value <= 0 || stats.food.value <= 0 || stats.water.value <= 0 {
            println!("You died!");
            break;
        }
    }
}

fn scavenge(inv: &mut std::vec::Vec<&str>, stats: &mut Stats) {
    let items = ["water", "berries", "wood", "flint", "string", "clams"];
    let mut rng = rand::thread_rng();
    let random_idx = rng.gen_range(0, items.len() - 1);
    let item = items[random_idx];
    sleep(Duration::new(2, 0));
    println!("You found {}", item);
    inv.push(item);
    stats.energy.decrease(5)
}

fn rest(stats: &mut Stats) {
    println!("Sleeping…");
    sleep(Duration::new(2, 0));
    stats.energy.increase(10)
}

fn eat(stats: &mut Stats) {
    println!("Eating…");
    stats.food.increase(10)
}

fn drink(stats: &mut Stats) {
    println!("Drinking…");
    stats.water.increase(10)
}
