use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluffyMachineSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub energy_per_tick: u64,
}

pub struct FluffyMachinesAddonEngine;

impl FluffyMachinesAddonEngine {
    pub fn get_machine_specs() -> Vec<FluffyMachineSpec> {
        vec![
            FluffyMachineSpec { id: "AUTO_ANVIL", name: "Yunque Automático", energy_per_tick: 64 },
            FluffyMachineSpec { id: "AUTO_ENCHANTER", name: "Encantador Automático", energy_per_tick: 128 },
            FluffyMachineSpec { id: "METAL_PRESS", name: "Prensa de Metal Fluffy", energy_per_tick: 32 },
            FluffyMachineSpec { id: "ELECTRIC_SMELTER", name: "Fundidor Eléctrico de Lingotes", energy_per_tick: 48 },
        ]
    }
}
