use crate::items::Item;
use colored::*;

pub const INV_MAX: usize = 10;
pub type Inventory = Vec<Item>;

pub fn print_inventory(inventory: &Inventory) {
  println!("Items in your backpack:");
  for item in inventory {
    println!("{}", item);
  }
}

pub fn remove_inventory(inv: &mut Inventory, item_id: &str) -> bool {
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
