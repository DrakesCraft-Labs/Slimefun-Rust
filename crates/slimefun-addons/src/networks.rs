use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworksItemSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub capacity_joules: u64,
}

pub struct NetworksAddonEngine;

impl NetworksAddonEngine {
    pub fn get_item_specs() -> Vec<NetworksItemSpec> {
        vec![
            NetworksItemSpec { id: "NETWORKS_NETWORK_BRIDGE", name: "Puente de Red Networks", category: "NETWORKS", capacity_joules: 10000 },
            NetworksItemSpec { id: "NETWORKS_NETWORK_MONITOR", name: "Monitor de Red", category: "NETWORKS", capacity_joules: 0 },
            NetworksItemSpec { id: "NETWORKS_CRAFTING_UNIT", name: "Unidad de Autocrafteo", category: "NETWORKS", capacity_joules: 5000 },
            NetworksItemSpec { id: "NETWORKS_IMPORT_BUS", name: "Bus de Importación", category: "NETWORKS", capacity_joules: 1000 },
            NetworksItemSpec { id: "NETWORKS_EXPORT_BUS", name: "Bus de Exportación", category: "NETWORKS", capacity_joules: 1000 },
        ]
    }
}
