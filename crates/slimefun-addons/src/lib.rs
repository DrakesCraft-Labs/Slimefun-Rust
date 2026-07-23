pub mod all_addons;
pub mod fluffy_machines;
pub mod networks;
pub mod supreme;

pub use all_addons::{AllAddonsRegistry, SlimefunAddonDescriptor};
pub use fluffy_machines::{FluffyMachineSpec, FluffyMachinesAddonEngine};
pub use networks::{NetworksAddonEngine, NetworksItemSpec};
pub use supreme::{SupremeAddonEngine, SupremeCardSpec};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddonMetadata {
    pub name: &'static str,
    pub version: &'static str,
    pub author: &'static str,
    pub total_items: u32,
}

pub struct SlimefunAddonsEngine;

impl SlimefunAddonsEngine {
    pub fn get_active_addons() -> Vec<AddonMetadata> {
        AllAddonsRegistry::get_all_44_addons()
            .into_iter()
            .map(|a| AddonMetadata {
                name: a.name,
                version: "1.0.0-RUST",
                author: "DrakesCraft Labs",
                total_items: 25,
            })
            .collect()
    }
}
