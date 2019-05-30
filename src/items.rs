use colored::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Stats {
    pub health: f64,
    pub water: f64,
    pub food: f64,
    pub energy: f64,
}

#[derive(Clone)]
pub struct Item<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub value: Stats,
    risk: i32,
    pub consumable: bool,
    days_to_perish: i32, // 0 for non perishable items
    uses_until_breakdown: i32,
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.consumable {
            write!(
                f,
                "{}: {} (+{} water, +{} food)",
                self.id.bold(),
                self.description.dimmed(),
                self.value.water,
                self.value.food
            )
        } else {
            write!(f, "{}: {}", self.id.bold(), self.description.dimmed())
        }
    }
}

pub const SCAVENGEABLE_ITEMS: [Item; 9] = [
    Item {
        id: "berries",
        name: "Berries",
        description: "Small amount of vitamins and water",
        value: Stats {
            health: 0.0,
            food: 3.0,
            water: 3.0,
            energy: 0.0,
        },
        risk: 0,
        days_to_perish: 5,
        uses_until_breakdown: 0,
        consumable: true,
    },
    Item {
        id: "water-dirty",
        name: "Dirty Water",
        description: "It will calm your thirst, but might make you sick",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 20.0,
            energy: 0.0,
        },
        risk: 6,
        days_to_perish: 0,
        uses_until_breakdown: 0,
        consumable: true,
    },
    Item {
        id: "salt",
        name: "Salt",
        description: "Useful to preserve food",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 0.0,
            energy: 0.0,
        },
        uses_until_breakdown: 0,
        consumable: false,
        days_to_perish: 0,
        risk: 0,
    },
    Item {
        id: "string",
        name: "String",
        description: "Useful for crafting",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 0.0,
            energy: 0.0,
        },
        uses_until_breakdown: 0,
        consumable: false,
        days_to_perish: 0,
        risk: 0,
    },
    Item {
        id: "wood",
        name: "Wood",
        description: "Useful for crafting",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 0.0,
            energy: 0.0,
        },
        uses_until_breakdown: 0,
        consumable: false,
        days_to_perish: 0,
        risk: 0,
    },
    Item {
        id: "plastic",
        name: "Plastic",
        description: "Useful for crafting",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 0.0,
            energy: 0.0,
        },
        uses_until_breakdown: 0,
        consumable: false,
        days_to_perish: 0,
        risk: 0,
    },
    Item {
        id: "bottle",
        name: "Empty bottle",
        description: "Useful for crafting",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 0.0,
            energy: 0.0,
        },
        uses_until_breakdown: 0,
        consumable: false,
        days_to_perish: 0,
        risk: 0,
    },
    Item {
        id: "flint",
        name: "Flint",
        description: "Useful for crafting",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 0.0,
            energy: 0.0,
        },
        uses_until_breakdown: 0,
        consumable: false,
        days_to_perish: 0,
        risk: 0,
    },
    Item {
        id: "medicinal-herbs",
        name: "Medicinal herbs",
        description: "Healing properties when brewed",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 0.0,
            energy: 0.0,
        },
        uses_until_breakdown: 0,
        consumable: false,
        days_to_perish: 0,
        risk: 0,
    },
];

pub const CRAFTABLE_ITEMS: [Item; 9] = [
    Item {
        id: "meat",
        name: "Raw meat",
        description: "Careful, might have parasites!",
        value: Stats {
            health: 0.0,
            food: 20.0,
            water: 0.0,
            energy: 0.0,
        },
        risk: 6,
        days_to_perish: 3,
        uses_until_breakdown: 0,
        consumable: true,
    },
    Item {
        id: "jerky",
        name: "Jerky",
        description: "Long lasting nourishment",
        value: Stats {
            health: 0.0,
            food: 20.0,
            water: 0.0,
            energy: 0.0,
        },
        risk: 0,
        days_to_perish: 0,
        uses_until_breakdown: 0,
        consumable: true,
    },
    Item {
        id: "medicinal-tea",
        name: "Medicinal tea",
        description: "Cures you from sickness and restores health",
        value: Stats {
            health: 50.0,
            food: 0.0,
            water: 20.0,
            energy: 0.0,
        },
        risk: 0,
        days_to_perish: 0,
        uses_until_breakdown: 0,
        consumable: true,
    },
    Item {
        id: "meat-cooked",
        name: "Cooked meat",
        description: "Tasty nourishment",
        value: Stats {
            health: 0.0,
            food: 20.0,
            water: 0.0,
            energy: 0.0,
        },
        risk: 0,
        days_to_perish: 10,
        uses_until_breakdown: 0,
        consumable: true,
    },
    Item {
        id: "water-clean",
        name: "Clean water",
        description: "Safe for drink",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 20.0,
            energy: 0.0,
        },
        risk: 0,
        days_to_perish: 0,
        uses_until_breakdown: 0,
        consumable: true,
    },
    Item {
        id: "rope",
        name: "Rope",
        description: "Useful for crafting",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 0.0,
            energy: 0.0,
        },
        uses_until_breakdown: 0,
        consumable: false,
        days_to_perish: 0,
        risk: 0,
    },
    Item {
        id: "bow:",
        name: "Bow:",
        description: "Lets you hunt and defend yourself:",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 0.0,
            energy: 0.0,
        },
        uses_until_breakdown: 5,
        consumable: false,
        days_to_perish: 0,
        risk: 0,
    },
    Item {
        id: "knife:",
        name: "Knife:",
        description: "Useful tool:",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 0.0,
            energy: 0.0,
        },
        uses_until_breakdown: 5,
        consumable: false,
        days_to_perish: 0,
        risk: 0,
    },
    Item {
        id: "rabbit-pelt:",
        name: "Rabbit pelt:",
        description: "It's not gonna be of much use until you can craft more stuff…:",
        value: Stats {
            health: 0.0,
            food: 0.0,
            water: 0.0,
            energy: 0.0,
        },
        uses_until_breakdown: 5,
        consumable: false,
        days_to_perish: 0,
        risk: 0,
    },
];

const HUNTABLE_ITEMS: [Item; 1] = [Item {
    id: "rabbit",
    name: "Dead rabbit",
    description: "Poor little thing",
    value: Stats {
        health: 0.0,
        food: 0.0,
        water: 0.0,
        energy: 0.0,
    },
    uses_until_breakdown: 0,
    consumable: false,
    days_to_perish: 0,
    risk: 0,
}];
