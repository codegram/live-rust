use colored::*;
use std::fmt;

use crate::camp::{CollectorStatus, Fire, FireStatus, WaterCollector};
use crate::inventory::{remove_inventory, Inventory};
use crate::items::{Item, ItemProperties, ItemStats};

#[derive(Debug)]
pub struct Recipe {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub items_needed: Vec<(&'static str, usize)>,
    pub tools_needed: Vec<&'static str>,
    pub upgrades_needed: Vec<&'static str>,
    pub result: Vec<&'static str>,
    pub category: RecipeCategory,
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name.bold())
    }
}

#[derive(Debug, PartialEq)]
pub enum RecipeCategory {
    Consumable,
    Tool,
    Weapon,
    CampUpgrade,
    Other,
}

pub const CRAFTABLE_ITEMS: [Item; 9] = [
    Item {
        id: "raw meat",
        name: "Raw meat",
        description: "Careful, might have parasites!",
        properties: ItemProperties::ConsumeableItem {
            value: ItemStats {
                health: 0.0,
                food: 25.0,
                water: 0.0,
                energy: 0.0,
            },
            risk: 3.0,
            days_to_perish: 3,
        },
    },
    Item {
        id: "jerky",
        name: "Jerky",
        description: "Long lasting nourishment",
        properties: ItemProperties::ConsumeableItem {
            value: ItemStats {
                health: 0.0,
                food: 25.0,
                water: 0.0,
                energy: 0.0,
            },
            risk: 0.0,
            days_to_perish: 0,
        },
    },
    Item {
        id: "medicinal tea",
        name: "Medicinal tea",
        description: "Cures you from sickness and restores health",
        properties: ItemProperties::ConsumeableItem {
            value: ItemStats {
                health: 35.0,
                food: 0.0,
                water: 20.0,
                energy: 0.0,
            },
            risk: 0.0,
            days_to_perish: 0,
        },
    },
    Item {
        id: "cooked meat",
        name: "Cooked meat",
        description: "Tasty nourishment",
        properties: ItemProperties::ConsumeableItem {
            value: ItemStats {
                health: 0.0,
                food: 25.0,
                water: 0.0,
                energy: 0.0,
            },
            risk: 0.0,
            days_to_perish: 10,
        },
    },
    Item {
        id: "clean water",
        name: "Clean water",
        description: "Safe for drink",
        properties: ItemProperties::ConsumeableItem {
            value: ItemStats {
                health: 0.0,
                food: 0.0,
                water: 20.0,
                energy: 0.0,
            },
            risk: 0.0,
            days_to_perish: 0,
        },
    },
    Item {
        id: "rope",
        name: "Rope",
        description: "Useful for crafting",
        properties: ItemProperties::StandardItem,
    },
    Item {
        id: "bow",
        name: "Bow",
        description: "Lets you hunt and defend yourself:",
        properties: ItemProperties::WeaponItem {
            uses_until_breakdown: 5,
        },
    },
    Item {
        id: "knife",
        name: "Knife",
        description: "Useful tool",
        properties: ItemProperties::ToolItem {
            uses_until_breakdown: 5,
        },
    },
    Item {
        id: "rabbit pelt",
        name: "Rabbit pelt",
        description: "It's not gonna be of much use until you can craft more stuff…:",
        properties: ItemProperties::StandardItem,
    },
];

pub fn recipes() -> [Recipe; 10] {
    return [
        Recipe {
            id: "fire",
            name: "Fire",
            description: "Will allow you to cook items",
            items_needed: vec![("wood", 1), ("flint", 1)],
            tools_needed: vec![],
            upgrades_needed: vec![],
            result: vec!["fire"],
            category: RecipeCategory::CampUpgrade,
        },
        Recipe {
            id: "water collector",
            name: "Water collector",
            description: "Collects rain water",
            items_needed: vec![("plastic", 1), ("rope", 1), ("bottle", 1)],
            tools_needed: vec![],
            upgrades_needed: vec![],
            result: vec!["water collector"],
            category: RecipeCategory::CampUpgrade,
        },
        Recipe {
            id: "rope",
            name: "Rope",
            description: "",
            items_needed: vec![("string", 2)],
            tools_needed: vec![],
            upgrades_needed: vec![],
            result: vec!["rope"],
            category: RecipeCategory::Tool,
        },
        Recipe {
            id: "bow",
            name: "Bow",
            description: "",
            items_needed: vec![("string", 1), ("wood", 1)],
            tools_needed: vec![],
            upgrades_needed: vec![],
            result: vec!["bow"],
            category: RecipeCategory::Weapon,
        },
        Recipe {
            id: "knife",
            name: "Knife",
            description: "",
            items_needed: vec![("flint", 1), ("wood", 1), ("rope", 1)],
            tools_needed: vec![],
            upgrades_needed: vec![],
            result: vec!["knife"],
            category: RecipeCategory::Tool,
        },
        Recipe {
            id: "jerky",
            name: "Jerky",
            description: "",
            items_needed: vec![("raw meat", 1), ("salt", 1)],
            tools_needed: vec![],
            upgrades_needed: vec![],
            result: vec!["jerky"],
            category: RecipeCategory::Consumable,
        },
        Recipe {
            id: "medicinal tea",
            name: "Medicinal tea",
            description: "",
            items_needed: vec![("clean water", 1), ("herbs", 1)],
            tools_needed: vec![],
            upgrades_needed: vec![],
            result: vec!["medicinal tea"],
            category: RecipeCategory::Consumable,
        },
        Recipe {
            id: "cooked meat",
            name: "Cooked meat",
            description: "",
            items_needed: vec![("raw meat", 1)],
            tools_needed: vec![],
            upgrades_needed: vec!["fire"],
            result: vec!["cooked meat"],
            category: RecipeCategory::Consumable,
        },
        Recipe {
            id: "clean water",
            name: "Clean water",
            description: "",
            items_needed: vec![("dirty water", 1)],
            tools_needed: vec![],
            upgrades_needed: vec!["fire"],
            result: vec!["clean water"],
            category: RecipeCategory::Consumable,
        },
        Recipe {
            id: "skinned rabbit",
            name: "Skinned rabbit",
            description: "Obtain meat and pelt",
            items_needed: vec![("rabbit", 1)],
            tools_needed: vec!["knife"],
            upgrades_needed: vec![],
            result: vec!["raw meat", "rabbit pelt"],
            category: RecipeCategory::Other,
        },
    ];
}

pub fn print_recipes() {
    let recipes = recipes();

    for recipe in &recipes {
        println!("{} - Items needed:", recipe);
        for item in &recipe.items_needed {
            println!(
                "\t{} {}",
                item.1.to_string().dimmed(),
                item.0.bold().dimmed(),
            )
        }
        for upgrade in &recipe.upgrades_needed {
            if upgrade as &str == "fire" {
                println!("\tNeeds fire");
            }
        }
        for tool in &recipe.tools_needed {
            println!("\tNeeds {}", tool);
        }
    }
}

pub fn craft_item(
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
            let mut error_msg = "You don't have enough items or upgrades";

            if recipe.category == RecipeCategory::CampUpgrade {
                match recipe_id {
                    "fire" => {
                        can_be_crafted = fire.status == FireStatus::Out;
                        if !can_be_crafted {
                            error_msg = "Fire is already burning";
                        }
                    }
                    "water collector" => {
                        can_be_crafted = collector.status == CollectorStatus::Out;
                        if !can_be_crafted {
                            error_msg = "Water collector already crafted";
                        }
                    }
                    _ => {}
                }
            }

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

            for tool in &recipe.tools_needed {
                let is_in_inventory = inv.iter().find(|i| i.id == *tool);

                if let None = is_in_inventory {
                    can_be_crafted = false;
                }
            }

            if can_be_crafted {
                for item in items_needed {
                    for _ in 0..item.1 {
                        remove_inventory(inv, item.0);
                    }
                }

                for tool in &recipe.tools_needed {
                    let tool_inv = inv.iter_mut().find(|i| i.id == *tool).unwrap();

                    let broke_down = tool_inv.decrease_use();

                    if broke_down {
                        println!("{} {}", tool.red(), "broke down".red());
                        remove_inventory(inv, tool);
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
                println!("{} {}", "Failed to craft.".red().bold(), error_msg);
            }
        }
        None => println!(
            "{} Type '{}' to list existing recipies.",
            "Invalid recipe.".red(),
            "crafting".bold()
        ),
    }
}
