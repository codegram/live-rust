extern crate rand;

use rand::seq::SliceRandom;
use rand::Rng;
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod items;

use crate::items::{Item, SCAVENGEABLE_ITEMS};

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
            "consume" => {
                println!("What do you want to eat/drink?");

                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .ok()
                    .expect("Failed to read line");

                consume(&mut inventory, input.trim(), &mut stats);
            }
            "remove" => {
                println!("What do you want to remove?");

                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .ok()
                    .expect("Failed to read line");

                remove_inventory(&mut inventory, input.trim());
            }
            _ => println!("Invalid input"),
        }

        if stats.energy.value <= 0.0 || stats.food.value <= 0.0 || stats.water.value <= 0.0 {
            println!("You died after {} days", days);
            break;
        }
    }
}

fn scavenge(inv: &mut std::vec::Vec<Item>, stats: &mut Stats) {
    let inv_max = 10;

    let slots_left = inv_max - inv.len();
    let number_of_items = if slots_left < 3 { slots_left } else { 3 };

    if number_of_items == 0 {
        println!("Your inventory is full");
    } else {
        let mut rng = rand::thread_rng();
        sleep(Duration::new(2, 0));
        stats.energy.decrease(5.0);
        for _number in 0..number_of_items {
            let item = SCAVENGEABLE_ITEMS.choose(&mut rng).unwrap().clone();
            println!("You found {:?}", item.name);
            inv.push(item);
        }
    }
}

fn rest(stats: &mut Stats) {
    println!("Sleeping…");
    sleep(Duration::new(2, 0));
    stats.energy.increase(10.0)
}

fn consume(inv: &mut std::vec::Vec<Item>, item_id: &str, stats: &mut Stats) {
    let item_idx = inv.iter().position(|item| item.id == item_id);

    match item_idx {
        Some(idx) => {
            let item = &inv[idx];
            if item.consumable {
                stats.water.increase(item.value.water);
                stats.food.increase(item.value.food);
                inv.remove(idx);
            } else {
                println!("Item is not consumable");
            }
        }
        None => println!("Item not in inventory"),
    }
}

fn decrease_stats(stats: &mut Stats, seconds: f64) {
    let ratio_energy = 25 as f64 / 60 as f64;
    let ratio_water = 25 as f64 / 60 as f64;
    let ratio_food = 15 as f64 / 60 as f64;

    stats.water.decrease(ratio_water * seconds);
    stats.food.decrease(ratio_food * seconds);
    stats.energy.decrease(ratio_energy * seconds);
}

fn remove_inventory(inv: &mut std::vec::Vec<Item>, item_id: &str) {
    let item_idx = inv.iter().position(|item| item.id == item_id);

    match item_idx {
        Some(idx) => {
            inv.remove(idx);
            println!("Item removed");
        }
        None => println!("Item not in inventory"),
    }
}
