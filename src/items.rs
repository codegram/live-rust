use colored::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub properties: ItemProperties,
}

#[derive(Debug, Clone)]
pub enum ItemProperties {
    StandardItem,
    ConsumeableItem {
        value: Stats,
        risk: i32,
        days_to_perish: i32, // 0 for non perishable items
    },
    ToolItem {
        uses_until_breakdown: i32,
    },
    WeaponItem {
        uses_until_breakdown: i32,
    },
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub health: f64,
    pub water: f64,
    pub food: f64,
    pub energy: f64,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.properties {
            ItemProperties::ConsumeableItem { value, .. } => write!(
                f,
                "{}: {} (+{} water, +{} food)",
                self.id.bold(),
                self.description.dimmed(),
                value.water,
                value.food
            ),
            _ => write!(f, "{}: {}", self.id.bold(), self.description.dimmed()),
        }
    }
}

pub const SCAVENGEABLE_ITEMS: [Item; 9] = [
    Item {
        id: "berries",
        name: "Berries",
        description: "Small amount of vitamins and water",
        properties: ItemProperties::ConsumeableItem {
            value: Stats {
                health: 0.0,
                food: 3.0,
                water: 3.0,
                energy: 0.0,
            },
            risk: 0,
            days_to_perish: 5,
        },
    },
    Item {
        id: "dirty water",
        name: "Dirty Water",
        description: "It will calm your thirst, but might make you sick",
        properties: ItemProperties::ConsumeableItem {
            value: Stats {
                health: 0.0,
                food: 0.0,
                water: 20.0,
                energy: 0.0,
            },
            risk: 6,
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

pub const CRAFTABLE_ITEMS: [Item; 9] = [
    Item {
        id: "raw meat",
        name: "Raw meat",
        description: "Careful, might have parasites!",
        properties: ItemProperties::ConsumeableItem {
            value: Stats {
                health: 0.0,
                food: 20.0,
                water: 0.0,
                energy: 0.0,
            },
            risk: 6,
            days_to_perish: 3,
        },
    },
    Item {
        id: "jerky",
        name: "Jerky",
        description: "Long lasting nourishment",
        properties: ItemProperties::ConsumeableItem {
            value: Stats {
                health: 0.0,
                food: 20.0,
                water: 0.0,
                energy: 0.0,
            },
            risk: 0,
            days_to_perish: 0,
        },
    },
    Item {
        id: "medicinal tea",
        name: "Medicinal tea",
        description: "Cures you from sickness and restores health",
        properties: ItemProperties::ConsumeableItem {
            value: Stats {
                health: 50.0,
                food: 0.0,
                water: 20.0,
                energy: 0.0,
            },
            risk: 0,
            days_to_perish: 0,
        },
    },
    Item {
        id: "cooked meat",
        name: "Cooked meat",
        description: "Tasty nourishment",
        properties: ItemProperties::ConsumeableItem {
            value: Stats {
                health: 0.0,
                food: 20.0,
                water: 0.0,
                energy: 0.0,
            },
            risk: 0,
            days_to_perish: 10,
        },
    },
    Item {
        id: "clean water",
        name: "Clean water",
        description: "Safe for drink",
        properties: ItemProperties::ConsumeableItem {
            value: Stats {
                health: 0.0,
                food: 0.0,
                water: 20.0,
                energy: 0.0,
            },
            risk: 0,
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
        id: "rabbit pelt:",
        name: "Rabbit pelt:",
        description: "It's not gonna be of much use until you can craft more stuff…:",
        properties: ItemProperties::StandardItem,
    },
];

const HUNTABLE_ITEMS: [Item; 1] = [Item {
    id: "rabbit",
    name: "Dead rabbit",
    description: "Poor little thing",
    properties: ItemProperties::StandardItem,
}];
