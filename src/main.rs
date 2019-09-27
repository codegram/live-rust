extern crate colored;
extern crate rand;
extern crate regex;

use colored::*;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::io;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod camp;
mod crafting;
mod hunt;
mod inventory;
mod items;
mod scavenge;
mod stats;

use crate::camp::{collect, stoke_fire, Fire, WaterCollector};
use crate::crafting::{craft_item, print_recipes};
use crate::hunt::hunt;
use crate::inventory::{print_inventory, remove_inventory, Inventory};
use crate::items::ItemProperties;
use crate::scavenge::scavenge;
use crate::stats::{decrease_stats, Stat, Stats};

fn main() {
    #[cfg(windows)]
    control::set_virtual_terminal(true);

    print_help();

    let mut inventory: Inventory = Vec::new();

    let fire = Arc::new(Mutex::new(Fire::new()));
    let water_collector = Arc::new(Mutex::new(WaterCollector::new()));

    let stats = Arc::new(Mutex::new(Stats {
        water: Stat::new(100.0),
        food: Stat::new(100.0),
        energy: Stat::new(100.0),
        health: Stat::new(100.0),
        is_sick: false,
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
                "hunt" => hunt(&mut inventory, &mut stats.lock().unwrap()),
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
    let health_death = stats.health.value <= 0.0;

    match (water_death, food_death, energy_death, health_death) {
        (true, _, _, _) => {
            print_death(
                "You died of thirst. A water collector could have saved your life",
                days,
            );
            true
        }
        (_, true, _, _) => {
            print_death("You died of hunger. A sturdy weapon would have provided you with a steady food supply.", days);
            true
        }
        (_, _, true, _) => {
            print_death(
                "You died from exhaustion. Remember, sleeping is important, even in the wild.",
                days,
            );
            true
        }
        (_, _, _, true) => {
            print_death(
                "You died of sickness. Skip the paleo diet, cooking your food is important.",
                days,
            );
            true
        }
        (false, false, false, false) => false,
    }
}

fn print_death(cause_of_death: &str, days: i32) {
    println!("*** {} ***", "G A M E  O V E R".red().bold());
    println!("You survived {} days", days.to_string().bold());
    println!("{}", cause_of_death)
}

fn rest(stats: &mut Stats) {
    println!("{}", "Sleeping…".italic().dimmed());
    sleep(Duration::new(2, 0));
    stats.energy.increase(50.0);
    println!("You wake up refreshed");
}

fn consume(inv: &mut Inventory, item_id: &str, stats: &mut Stats) {
    let item_idx = inv.iter().position(|item| item.id == item_id);

    match item_idx {
        Some(idx) => {
            let item = &inv[idx];
            match &item.properties {
                ItemProperties::ConsumeableItem { value, risk, .. } => {
                    stats.water.increase(value.water);
                    stats.food.increase(value.food);
                    stats.food.increase(value.health);

                    let mut get_sick = false;

                    if risk > &0.0 {
                        let mut rng = thread_rng();
                        get_sick = rng.gen_bool(1.0 / risk);
                    }

                    if item_id == "medicinal tea" && stats.is_sick {
                        stats.is_sick = false;
                        println!("{}", "You are feeling better now!".green());
                    } else if get_sick {
                        stats.is_sick = true;
                        stats.health.decrease(10.0);
                        println!("{}", "You got sick!".red());
                    } else {
                        println!("Yummy!");
                    }

                    inv.remove(idx);
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
    println!(
        "{}:\t\t Hunt for food and fur to craft equipment",
        "hunt".bold()
    );
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
