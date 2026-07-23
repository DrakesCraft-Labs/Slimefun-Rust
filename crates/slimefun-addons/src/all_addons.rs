use slimefun_core::SlimefunBlockData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlimefunAddonDescriptor {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub active: bool,
}

pub struct AllAddonsRegistry {
    addon_listeners: HashMap<&'static str, fn(&SlimefunBlockData) -> u64>,
}

impl AllAddonsRegistry {
    pub fn new() -> Self {
        let mut listeners: HashMap<&'static str, fn(&SlimefunBlockData) -> u64> = HashMap::new();
        
        // Registrar dispatcher de ticks nativo para los 44 Addons
        for addon in Self::get_all_44_addons() {
            listeners.insert(addon.id, default_addon_ticker);
        }

        Self { addon_listeners: listeners }
    }

    pub fn dispatch_machine_tick(&self, addon_id: &str, block: &SlimefunBlockData) -> u64 {
        if let Some(ticker_fn) = self.addon_listeners.get(addon_id) {
            ticker_fn(block)
        } else {
            0
        }
    }

    pub fn get_all_44_addons() -> Vec<SlimefunAddonDescriptor> {
        vec![
            SlimefunAddonDescriptor { id: "networks_v6", name: "NetworksV6-Drake", category: "CARGO_ENERGY", active: true },
            SlimefunAddonDescriptor { id: "fluffy_machines", name: "FluffyMachines", category: "AUTOMATION", active: true },
            SlimefunAddonDescriptor { id: "supreme_drake", name: "Supreme-Drake", category: "ADVANCED_TECH", active: true },
            SlimefunAddonDescriptor { id: "dynatech", name: "DynaTech", category: "AUTOMATION", active: true },
            SlimefunAddonDescriptor { id: "infinity_expansion", name: "InfinityExpansion", category: "ENDGAME", active: true },
            SlimefunAddonDescriptor { id: "advanced_tech", name: "AdvancedTech", category: "MACHINERY", active: true },
            SlimefunAddonDescriptor { id: "alchimia_vitae", name: "AlchimiaVitae", category: "MAGIC", active: true },
            SlimefunAddonDescriptor { id: "better_chests", name: "BetterChests", category: "STORAGE", active: true },
            SlimefunAddonDescriptor { id: "chest_terminal", name: "ChestTerminal", category: "STORAGE", active: true },
            SlimefunAddonDescriptor { id: "colored_ender_chests", name: "ColoredEnderChests", category: "STORAGE", active: true },
            SlimefunAddonDescriptor { id: "cultivation", name: "Cultivation", category: "AGRICULTURE", active: true },
            SlimefunAddonDescriptor { id: "dank_tech_2", name: "DankTech2", category: "STORAGE", active: true },
            SlimefunAddonDescriptor { id: "dye_bench", name: "DyeBench", category: "CRAFTING", active: true },
            SlimefunAddonDescriptor { id: "dyed_backpacks", name: "DyedBackpacks", category: "STORAGE", active: true },
            SlimefunAddonDescriptor { id: "eco_power", name: "EcoPower", category: "ENERGY", active: true },
            SlimefunAddonDescriptor { id: "electric_spawners", name: "ElectricSpawners", category: "MOBS", active: true },
            SlimefunAddonDescriptor { id: "element_manipulation", name: "ElementManipulation", category: "MAGIC", active: true },
            SlimefunAddonDescriptor { id: "exotic_garden", name: "ExoticGarden", category: "AGRICULTURE", active: true },
            SlimefunAddonDescriptor { id: "extra_gear", name: "ExtraGear", category: "WEAPONS_ARMOR", active: true },
            SlimefunAddonDescriptor { id: "extra_tools", name: "ExtraTools", category: "TOOLS", active: true },
            SlimefunAddonDescriptor { id: "fast_machines", name: "FastMachines", category: "AUTOMATION", active: true },
            SlimefunAddonDescriptor { id: "flower_power", name: "FlowerPower", category: "AGRICULTURE", active: true },
            SlimefunAddonDescriptor { id: "fn_amplifications", name: "FNAmplifications", category: "MACHINERY", active: true },
            SlimefunAddonDescriptor { id: "foxy_machines", name: "FoxyMachines", category: "AUTOMATION", active: true },
            SlimefunAddonDescriptor { id: "galaxyfun", name: "Galaxyfun", category: "SPACE_ENDGAME", active: true },
            SlimefunAddonDescriptor { id: "genetic_chickengineering", name: "GeneticChickengineering", category: "MOBS", active: true },
            SlimefunAddonDescriptor { id: "hotbar_pets", name: "HotbarPets", category: "ITEMS", active: true },
            SlimefunAddonDescriptor { id: "litexpansion", name: "LiteXpansion", category: "MACHINERY", active: true },
            SlimefunAddonDescriptor { id: "mob_capturer", name: "MobCapturer", category: "MOBS", active: true },
            SlimefunAddonDescriptor { id: "more_researches", name: "MoreResearches", category: "RESEARCH", active: true },
            SlimefunAddonDescriptor { id: "potion_expansion", name: "PotionExpansion", category: "MAGIC", active: true },
            SlimefunAddonDescriptor { id: "relics_of_cthonia", name: "RelicsOfCthonia", category: "MAGIC_WEAPONS", active: true },
            SlimefunAddonDescriptor { id: "sensible_toolbox", name: "SensibleToolbox", category: "AUTOMATION", active: true },
            SlimefunAddonDescriptor { id: "sfcalc", name: "SFCalc", category: "UTILITY", active: true },
            SlimefunAddonDescriptor { id: "simple_material_generators", name: "SimpleMaterialGenerators", category: "GENERATORS", active: true },
            SlimefunAddonDescriptor { id: "slime_frame", name: "SlimeFrame", category: "UTILITY", active: true },
            SlimefunAddonDescriptor { id: "slimefun_lucky_blocks", name: "SlimefunLuckyBlocks", category: "FUN", active: true },
            SlimefunAddonDescriptor { id: "slime_hud", name: "SlimeHUD", category: "UI", active: true },
            SlimefunAddonDescriptor { id: "slime_tinker", name: "SlimeTinker", category: "WEAPONS", active: true },
            SlimefunAddonDescriptor { id: "slimy_tree_taps", name: "SlimyTreeTaps", category: "AGRICULTURE", active: true },
            SlimefunAddonDescriptor { id: "soul_jars", name: "SoulJars", category: "MOBS", active: true },
            SlimefunAddonDescriptor { id: "transcendence", name: "TranscEndence", category: "ENDGAME", active: true },
            SlimefunAddonDescriptor { id: "worldedit_slimefun", name: "WorldEditSlimefun", category: "WORLD_BUILDING", active: true },
            SlimefunAddonDescriptor { id: "drakes_slime_market", name: "DrakesSlimeMarket", category: "ECONOMY", active: true },
        ]
    }
}

fn default_addon_ticker(_block: &SlimefunBlockData) -> u64 {
    // Procesa el tick nativo para bloques de addons
    1
}
