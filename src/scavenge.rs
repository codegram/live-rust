use colored::*;
use rand::seq::SliceRandom;
use std::thread::sleep;
use std::time::Duration;

use crate::inventory::{Inventory, INV_MAX};
use crate::items::{Item, ItemProperties, ItemStats};
use crate::stats::Stats;

const SCAVENGEABLE_ITEMS: [Item; 9] = [
  Item {
    id: "berries",
    name: "Berries",
    description: "Small amount of vitamins and water",
    properties: ItemProperties::ConsumeableItem {
      value: ItemStats {
        health: 0.0,
        food: 3.0,
        water: 3.0,
        energy: 0.0,
      },
      risk: 0.0,
      days_to_perish: 5,
    },
  },
  Item {
    id: "dirty water",
    name: "Dirty Water",
    description: "It will calm your thirst, but might make you sick",
    properties: ItemProperties::ConsumeableItem {
      value: ItemStats {
        health: 0.0,
        food: 0.0,
        water: 20.0,
        energy: 0.0,
      },
      risk: 3.0,
      days_to_perish: 0,
    },
  },
  Item {
    id: "salt",
    name: "Salt",
    description: "Useful to preserve food",
    properties: ItemProperties::StandardItem,
  },
  Item {
    id: "string",
    name: "String",
    description: "Useful for crafting",
    properties: ItemProperties::StandardItem,
  },
  Item {
    id: "wood",
    name: "Wood",
    description: "Useful for crafting",
    properties: ItemProperties::StandardItem,
  },
  Item {
    id: "plastic",
    name: "Plastic",
    description: "Useful for crafting",
    properties: ItemProperties::StandardItem,
  },
  Item {
    id: "bottle",
    name: "Empty bottle",
    description: "Useful for crafting",
    properties: ItemProperties::StandardItem,
  },
  Item {
    id: "flint",
    name: "Flint",
    description: "Useful for crafting",
    properties: ItemProperties::StandardItem,
  },
  Item {
    id: "medicinal herbs",
    name: "Medicinal herbs",
    description: "Healing properties when brewed",
    properties: ItemProperties::StandardItem,
  },
];

pub fn scavenge(inv: &mut Inventory, stats: &mut Stats) {
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
