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
    print_help();

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
        let action = request_input("What to do?");

        let current_elapsed_time = now.elapsed().as_secs();
        let seconds = current_elapsed_time - elapsed_time;

        elapsed_time = current_elapsed_time;
        days = elapsed_time as i32 / 60;

        decrease_stats(&mut stats, seconds as f64);

        match action.trim() {
            "help" => print_help(),
            "sleep" => rest(&mut stats),
            "scavenge" => scavenge(&mut inventory, &mut stats),
            "inventory" => print_inventory(&inventory),
            "stats" => println!("Current stats: {:?}", stats),
            "days" => println!("Days survived so far: {}", days),
            "consume" => {
                let input = request_input("What do you want to eat/drink?");
                consume(&mut inventory, input.trim(), &mut stats);
            }
            "remove" => {
                let input = request_input("What do you want to remove?");
                remove_inventory(&mut inventory, input.trim());
            }
            "die" => {
                println!("You died after {} days", days);
                break;
            }
            _ => println!("Invalid input. Type 'help' for instructions."),
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
        println!("Your inventory is full. Remove at least one item to proceed.");
    } else {
        let mut rng = rand::thread_rng();
        sleep(Duration::new(2, 0));
        stats.energy.decrease(5.0);
        stats.water.decrease(5.0);
        stats.food.decrease(3.0);
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
    stats.energy.increase(35.0);
    println!("You wake up refreshed");
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
        None => println!("Item not in inventory. Type 'inventory' to list available items."),
    }
}

fn remove_inventory(inv: &mut std::vec::Vec<Item>, item_id: &str) {
    let item_idx = inv.iter().position(|item| item.id == item_id);

    match item_idx {
        Some(idx) => {
            inv.remove(idx);
            println!("Item removed");
        }
        None => println!("Item not in inventory. Type 'inventory' to list available items."),
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

fn request_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    input
}

fn print_help() {
    println!("\n*** L I V E ***");
    println!("How long can you survive in the wilderness?\n");
    println!("Actions:\n");
    println!("scavenge:\t Find useful items to survive");
    println!("\t\t -5 energy, -5 water, -3 food");
    println!("sleep:\t\t Rest to replenish your energy");
    println!("\t\t +35 energy");
    println!("hunt:\t\t COMING SOON"); //Hunt for food and fur to craft equipment
    println!("\t\t -10 energy, -10 water, -6 food");
    println!("\nremove:\t\t Remove an item from inventory");
    println!("consume:\t Consume an item to replenish food or water");
    println!("craft:\t\t COMING SOON");
    println!("inventory:\t List items on inventory");
    println!("stats:\t\t List your current stats");
    println!("days:\t\t List the days survived so far");
    println!("help:\t\t Print this instructions again");
    println!("\n");
}

fn print_inventory(inventory: &std::vec::Vec<Item>) {
    println!("Items in your backpack:");
    for item in inventory {
        println!("{:?}", item);
    }
}

// This is a really cool function but I prefer not to use it until I have a proper understanding of lifetimes
// fn get_inventory_item<'a, 'b>(
//     inv: &'a mut std::vec::Vec<Item<'a>>,
//     item_id: &'b str,
// ) -> Option<(&'a mut Item<'a>, usize)> {
//     let item_idx = inv.iter().position(|item| item.id == item_id);

//     match item_idx {
//         Some(idx) => Some((&mut inv[idx], idx)),
//         None => {
//             println!("Item not in inventory");
//             None
//         }
//     }
// }
