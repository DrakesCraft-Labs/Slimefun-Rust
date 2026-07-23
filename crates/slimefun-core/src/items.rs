use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SlimefunCategory {
    Resources,
    Energy,
    Cargo,
    Machinery,
    Tools,
    Weapons,
    Armor,
    Magic,
    Agriculture,
    Storage,
    Endgame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlimefunItemSpec {
    pub id: String,
    pub name: String,
    pub category: SlimefunCategory,
    pub recipe_type: String,
    pub recipe_inputs: Vec<String>,
    pub recipe_output_count: u32,
    pub is_electric: bool,
    pub capacity_joules: u64,
    pub energy_consumption_per_tick: u64,
}

pub struct SlimefunItemRegistry {
    items: HashMap<String, SlimefunItemSpec>,
}

impl SlimefunItemRegistry {
    pub fn new() -> Self {
        let mut items = HashMap::new();

        // 1. Basic Slimefun Dusts & Ingots
        let dusts = [
            ("IRON_DUST", "Polvo de Hierro", "RESOURCE"),
            ("GOLD_DUST", "Polvo de Oro", "RESOURCE"),
            ("COPPER_DUST", "Polvo de Cobre", "RESOURCE"),
            ("TIN_DUST", "Polvo de Estaño", "RESOURCE"),
            ("LEAD_DUST", "Polvo de Plomo", "RESOURCE"),
            ("SILVER_DUST", "Polvo de Plata", "RESOURCE"),
            ("ALUMINUM_DUST", "Polvo de Aluminio", "RESOURCE"),
            ("ZINC_DUST", "Polvo de Zinc", "RESOURCE"),
            ("MAGNESIUM_DUST", "Polvo de Magnesio", "RESOURCE"),
        ];

        for (id, name, _cat) in dusts {
            items.insert(
                id.to_string(),
                SlimefunItemSpec {
                    id: id.to_string(),
                    name: name.to_string(),
                    category: SlimefunCategory::Resources,
                    recipe_type: "ORE_WASHER".to_string(),
                    recipe_inputs: vec!["SIFTED_ORE".to_string()],
                    recipe_output_count: 1,
                    is_electric: false,
                    capacity_joules: 0,
                    energy_consumption_per_tick: 0,
                },
            );
        }

        // 2. Electric Machinery & Energy
        items.insert(
            "SOLAR_GENERATOR".to_string(),
            SlimefunItemSpec {
                id: "SOLAR_GENERATOR".to_string(),
                name: "Generador Solar I".to_string(),
                category: SlimefunCategory::Energy,
                recipe_type: "ENHANCED_CRAFTING_TABLE".to_string(),
                recipe_inputs: vec!["SOLAR_PANEL".to_string(), "BASIC_CIRCUIT_BOARD".to_string()],
                recipe_output_count: 1,
                is_electric: true,
                capacity_joules: 1000,
                energy_consumption_per_tick: 0,
            },
        );

        items.insert(
            "ELECTRIC_SMELTER".to_string(),
            SlimefunItemSpec {
                id: "ELECTRIC_SMELTER".to_string(),
                name: "Fundidor Eléctrico I".to_string(),
                category: SlimefunCategory::Machinery,
                recipe_type: "ENHANCED_CRAFTING_TABLE".to_string(),
                recipe_inputs: vec!["HEATING_COIL".to_string(), "ELECTRIC_MOTOR".to_string()],
                recipe_output_count: 1,
                is_electric: true,
                capacity_joules: 2000,
                energy_consumption_per_tick: 24,
            },
        );

        Self { items }
    }

    pub fn get(&self, id: &str) -> Option<&SlimefunItemSpec> {
        self.items.get(id)
    }

    pub fn register_item(&mut self, spec: SlimefunItemSpec) {
        self.items.insert(spec.id.clone(), spec);
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}
