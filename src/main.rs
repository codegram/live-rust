extern crate colored;
extern crate rand;
extern crate regex;

use colored::*;
use rand::seq::SliceRandom;
use regex::Regex;
use std::fmt;
use std::io;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod camp;
mod crafting;
mod items;

use crate::items::{Item, ItemProperties, CRAFTABLE_ITEMS, SCAVENGEABLE_ITEMS};

use crate::crafting::{print_recipes, recipes, RecipeCategory};

use crate::camp::{CollectorStatus, Fire, FireStatus, WaterCollector};

const MAX: f64 = 100.0;
const INV_MAX: usize = 10;

type Inventory = Vec<Item>;

#[derive(Debug)]
struct Stats {
    water: Stat,
    food: Stat,
    energy: Stat,
}

struct Stat {
    value: f64,
}

impl fmt::Debug for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*}", 2, self.value)
    }
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

    let mut inventory: Inventory = Vec::new();

    let fire = Arc::new(Mutex::new(Fire::new()));
    let water_collector = Arc::new(Mutex::new(WaterCollector::new()));

    let stats = Arc::new(Mutex::new(Stats {
        water: Stat::new(100.0),
        food: Stat::new(100.0),
        energy: Stat::new(100.0),
    }));

    let days = Arc::new(Mutex::new(0));

    control_time(&days, &stats, &fire, &water_collector);

    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || loop {
        let _ = rx2.recv();

        let action = request_input("\nWhat to do?");

        let send = action.clone();

        tx.send(send).ok();
    });

    trigger_input(&tx2);

    loop {
        if let Ok(action) = rx.try_recv() {
            match action.trim() {
                "help" => print_help(),
                "sleep" => rest(&mut stats.lock().unwrap()),
                "scavenge" => scavenge(&mut inventory, &mut stats.lock().unwrap()),
                "inventory" => print_inventory(&inventory),
                "stats" => {
                    let print_stats = stats.lock().unwrap();
                    println!("Current {:#?}", *print_stats);
                }
                "days" => {
                    let print_days = days.lock().unwrap();

                    println!("Days survived so far: {:?}", *print_days)
                }
                "recipes" => print_recipes(),
                "consume" => {
                    let input = request_input("What do you want to eat/drink?");
                    consume(&mut inventory, input.trim(), &mut stats.lock().unwrap());
                }
                "stoke" => {
                    stoke_fire(&mut inventory, &mut fire.lock().unwrap());
                }
                "collect" => {
                    collect(&mut inventory, &mut water_collector.lock().unwrap());
                }
                "remove" => {
                    let input = request_input("What do you want to remove?");
                    remove_inventory(&mut inventory, input.trim());
                }
                "die" => {
                    println!("You died after {} days", days.lock().unwrap());
                    break;
                }
                _ => {
                    let re = Regex::new(r"(consume|remove|craft)(.+)").unwrap();

                    let capture_groups = re.captures_iter(action.trim());

                    let mut matched = false;

                    for cap in capture_groups {
                        let action = &cap[1];
                        let target = &cap[2].trim();

                        matched = match action {
                            "remove" => {
                                remove_inventory(&mut inventory, target);
                                true
                            }
                            "consume" => {
                                consume(&mut inventory, target, &mut stats.lock().unwrap());
                                true
                            }
                            "craft" => {
                                craft_item(
                                    &mut inventory,
                                    target,
                                    &mut fire.lock().unwrap(),
                                    &mut water_collector.lock().unwrap(),
                                );
                                true
                            }
                            _ => false,
                        };
                    }
                    if matched == false {
                        println!("Invalid input. Type 'help' for instructions.")
                    }
                }
            }
            trigger_input(&tx2);
        }

        let days_game_over = days.lock().unwrap();

        if is_game_over(&stats.lock().unwrap(), *days_game_over) {
            break;
        }
    }
}

fn control_time(
    days: &Arc<Mutex<i32>>,
    stats: &Arc<Mutex<Stats>>,
    fire: &Arc<Mutex<Fire>>,
    water_collector: &Arc<Mutex<WaterCollector>>,
) {
    let now = Instant::now();
    let days = Arc::clone(&days);
    let stats = Arc::clone(&stats);
    let fire = Arc::clone(&fire);
    let water_collector = Arc::clone(&water_collector);

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));

        let mut fire_lock = fire.lock().unwrap();
        fire_lock.pass_time();

        let mut collector_lock = water_collector.lock().unwrap();
        collector_lock.pass_time();

        let elapsed_time = now.elapsed().as_secs();
        let mut elapsed_days = days.lock().unwrap();

        *elapsed_days = elapsed_time as i32 / 60;

        let mut stats_lock = stats.lock().unwrap();
        decrease_stats(&mut stats_lock, 10.0);
    });
}

fn trigger_input(tx: &mpsc::Sender<String>) {
    tx.send(String::from("do something")).ok();
}

fn is_game_over(stats: &Stats, days: i32) -> bool {
    let water_death = stats.water.value <= 0.0;
    let food_death = stats.food.value <= 0.0;
    let energy_death = stats.energy.value <= 0.0;

    match (water_death, food_death, energy_death) {
        (true, _, _) => {
            print_death(
                "You died of thirst. A water collector could have saved your life",
                days,
            );
            true
        }
        (_, true, _) => {
            print_death("You died of hunger. A sturdy weapon would have provided you with a steady food supply, if only someone programmed the hunting feature", days);
            true
        }
        (_, _, true) => {
            print_death(
                "You died from exhaustion. Remember, sleeping is important, even in the wild.",
                days,
            );
            true
        }
        (false, false, false) => false,
    }
}

fn print_death(cause_of_death: &str, days: i32) {
    println!("*** {} ***", "G A M E  O V E R".red().bold());
    println!("You survived {} days", days.to_string().bold());
    println!("{}", cause_of_death)
}

fn scavenge(inv: &mut Inventory, stats: &mut Stats) {
    let slots_left = INV_MAX - inv.len();
    let number_of_items = if slots_left < 3 { slots_left } else { 3 };

    if number_of_items == 0 {
        println!("Your inventory is full. Remove at least one item to proceed.");
    } else {
        let mut rng = rand::thread_rng();
        println!("{}", "Scavenging…".italic().dimmed());
        sleep(Duration::new(2, 0));
        stats.energy.decrease(5.0);
        stats.water.decrease(5.0);
        stats.food.decrease(3.0);
        for _number in 0..number_of_items {
            let item = SCAVENGEABLE_ITEMS.choose(&mut rng).unwrap().clone();
            println!("You found {}", item.name.bold());
            inv.push(item);
        }
    }
}

fn rest(stats: &mut Stats) {
    println!("{}", "Sleeping…".italic().dimmed());
    sleep(Duration::new(2, 0));
    stats.energy.increase(35.0);
    println!("You wake up refreshed");
}

fn consume(inv: &mut Inventory, item_id: &str, stats: &mut Stats) {
    let item_idx = inv.iter().position(|item| item.id == item_id);

    match item_idx {
        Some(idx) => {
            let item = &inv[idx];
            match &item.properties {
                ItemProperties::ConsumeableItem { value, .. } => {
                    stats.water.increase(value.water);
                    stats.food.increase(value.food);
                    inv.remove(idx);
                    println!("Yummy!");
                }
                _ => println!("{}", "Item is not consumable".red()),
            }
        }
        None => println!(
            "{} Type '{}' to list available items.",
            "Item not in inventory.".red(),
            "inventory".bold()
        ),
    }
}

fn craft_item(
    inv: &mut Inventory,
    recipe_id: &str,
    fire: &mut Fire,
    collector: &mut WaterCollector,
) {
    let recipes = recipes();
    let recipe = recipes.iter().find(|&recipe| recipe.id == recipe_id);

    match recipe {
        Some(recipe) => {
            let items_needed = &recipe.items_needed;
            let items_supplied = &recipe.result;
            let mut can_be_crafted = true;

            for item in items_needed {
                let amount_in_inventory = inv.iter().filter(|i| i.id == item.0).count();
                can_be_crafted = can_be_crafted && amount_in_inventory >= item.1;
            }

            for upgrade in &recipe.upgrades_needed {
                match upgrade as &str {
                    "fire" => can_be_crafted = can_be_crafted && fire.status != FireStatus::Out,
                    _ => println!("Unable to craft"),
                }
            }

            if can_be_crafted {
                for item in items_needed {
                    for _ in 0..item.1 {
                        remove_inventory(inv, item.0);
                    }
                }

                for item in items_supplied {
                    match recipe.category {
                        RecipeCategory::CampUpgrade => {
                            println!("craft upgrade {}", item);

                            match item as &str {
                                "fire" => {
                                    fire.craft();
                                    println!("Fire is burning {:?}", fire.status)
                                }
                                "water collector" => {
                                    collector.craft();
                                }
                                _ => println!("Unable to craft {}", item),
                            }
                        }
                        _ => {
                            let result = CRAFTABLE_ITEMS
                                .iter()
                                .find(|craftable| craftable.id == *item)
                                .unwrap()
                                .clone();
                            println!("You got {}", result.name.bold());
                            inv.push(result);
                        }
                    }
                }
            } else {
                println!(
                    "{} You don't have enough items or upgrades",
                    "Failed to craft.".red().bold()
                );
            }
        }
        None => println!(
            "{} Type '{}' to list existing recipies.",
            "Invalid recipe.".red(),
            "crafting".bold()
        ),
    }
}

fn remove_inventory(inv: &mut Inventory, item_id: &str) -> bool {
    let item_idx = inv.iter().position(|item| item.id == item_id);

    match item_idx {
        Some(idx) => {
            inv.remove(idx);
            println!("{}", "Item removed".green());
            true
        }
        None => {
            println!(
                "{} Type '{}' to list available items.",
                "Item not in inventory.".red(),
                "inventory".bold()
            );
            false
        }
    }
}

fn stoke_fire(inv: &mut Inventory, fire: &mut Fire) {
    if fire.status != FireStatus::Out {
        if remove_inventory(inv, "wood") {
            fire.increase_status();
        } else {
            println!("{}", "You don't have wood in your inventory".red());
        }
    } else {
        println!("{}", "You don't have a fire in your camp".red());
    }
}

fn collect(inv: &mut Inventory, collector: &mut WaterCollector) {
    if inv.len() == INV_MAX {
        println!("{}", "Your inventory is full".red());
        return;
    }

    if collector.status == CollectorStatus::Waiting {
        let result = CRAFTABLE_ITEMS
            .iter()
            .find(|craftable| craftable.id == "clean water")
            .unwrap()
            .clone();
        println!("You got {}", result.name.bold());
        inv.push(result);
        collector.collect();
    } else {
        println!("{}", "There is nothing to collect".red());
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
    println!("{}", prompt.bold());

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    input
}

fn print_help() {
    println!("\n*** {} ***", "L I V E".bold());
    println!("How long can you survive in the wilderness?\n");
    println!("Actions:\n");
    println!("{}:\t Find useful items to survive", "scavenge".bold());
    println!("\t\t {}", "-5 energy, -5 water, -3 food".italic().dimmed());
    println!("{}:\t\t Rest to replenish your energy", "sleep".bold());
    println!("\t\t {}", "+35 energy".italic().dimmed());
    println!("{}:\t\t COMING SOON", "hunt".bold()); //Hunt for food and fur to craft equipment
    println!(
        "\t\t {}",
        "-10 energy, -10 water, -6 food".italic().dimmed()
    );
    println!("\n{}:\t\t Remove an item from inventory", "remove".bold());
    println!(
        "{}:\t Consume an item to replenish food or water",
        "consume".bold()
    );
    println!(
        "{}:\t List all the available recipes to craft items",
        "recipes".bold()
    );
    println!(
        "{} <item>:\t Combine items to create other items",
        "craft".bold()
    );
    println!("{}:\t\t Add wood to stoke the fire", "stoke".bold());
    println!("{}:\t Add clean water to your inventory", "collect".bold());
    println!("{}:\t List items on inventory", "inventory".bold());
    println!("{}:\t\t List your current stats", "stats".bold());
    println!("{}:\t\t List the days survived so far", "days".bold());
    println!("{}:\t\t Print this instructions again", "help".bold());
    println!("\n");
}

fn print_inventory(inventory: &Inventory) {
    println!("Items in your backpack:");
    for item in inventory {
        println!("{}", item);
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
