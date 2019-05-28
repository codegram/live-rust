extern crate rand;

use rand::Rng;
use std::io;

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
            self.value = 100;
        }
    }
    fn decrease(&mut self, amount: i32) {
        self.value = self.value - amount;
    }
}

fn main() {
    println!("* LIVE *\n\n");

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

        match action.trim() {
            "help" => println!("Sorry. You are on your own."),
            "sleep" => sleep(&mut stats),
            "scavenge" => scavenge(&mut inventory, &mut stats),
            "inventory" => println!("Items in your backpack: {:?}", inventory),
            "stats" => println!("Current stats: {:?}", stats),
            "die" => {
                println!("You died!");
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
    println!("You found {}", item);
    inv.push(item);
    stats.energy.decrease(5)
}

fn sleep(stats: &mut Stats) {
    println!("Sleeping…");
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
