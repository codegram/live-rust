extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("* LIVE *\n\n");

    let mut inventory = Vec::new();

    loop {
        println!("What to do?");

        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .ok()
            .expect("Failed to read line");

        match action.trim() {
            "help" => println!("Sorry. You are on your own."),
            "sleep" => println!("Sleeping…"),
            "scavenge" => scavenge(&mut inventory),
            "inventory" => println!("Items in your backpack: {:?}", inventory),
            "die" => {
                println!("You died!");
                break;
            }
            _ => println!("Invalid input"),
        }
    }
}

fn scavenge(inv: &mut std::vec::Vec<&str>) {
    let items = ["water", "berries", "wood", "flint", "string", "clams"];
    let mut rng = rand::thread_rng();
    let random_idx = rng.gen_range(0, items.len() - 1);
    let item = items[random_idx];
    println!("You found {}", item);
    inv.push(item);
}
