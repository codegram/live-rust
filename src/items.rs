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
        value: ItemStats,
        risk: f64,
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
pub struct ItemStats {
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
            ItemProperties::ToolItem {
                uses_until_breakdown,
            }
            | ItemProperties::WeaponItem {
                uses_until_breakdown,
            } => write!(
                f,
                "{}: {} ({} uses left)",
                self.id.bold(),
                self.description.dimmed(),
                uses_until_breakdown
            ),
            _ => write!(f, "{}: {}", self.id.bold(), self.description.dimmed()),
        }
    }
}

impl Item {
    pub fn decrease_use(&mut self) -> bool {
        match self.properties {
            ItemProperties::ToolItem {
                ref mut uses_until_breakdown,
            } => {
                *uses_until_breakdown -= 1;
                *uses_until_breakdown == 0
            }
            ItemProperties::WeaponItem {
                ref mut uses_until_breakdown,
            } => {
                *uses_until_breakdown -= 1;
                *uses_until_breakdown == 0
            }
            _ => {
                println!("{}", "Item does not degrade".red());
                false
            }
        }
    }
}
