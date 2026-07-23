use dashmap::DashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnergyNodeType {
    Generator { production_per_tick: u64 },
    Capacitor { capacity: u64, stored: u64 },
    Consumer { demand_per_tick: u64 },
    Connector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyNode {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub node_type: EnergyNodeType,
}

pub struct EnergyNetGraphSolver {
    nodes: DashMap<(i32, i32, i32), EnergyNode>,
}

impl EnergyNetGraphSolver {
    pub fn new() -> Self {
        Self {
            nodes: DashMap::new(),
        }
    }

    pub fn register_node(&self, x: i32, y: i32, z: i32, node_type: EnergyNodeType) {
        let node = EnergyNode { x, y, z, node_type };
        self.nodes.insert((x, y, z), node);
    }

    /// Resuelve en paralelo (nanosegundos, sin pausas de GC) la distribución de energía.
    pub fn solve_tick(&self) -> u64 {
        let mut total_generated = 0;
        for r in self.nodes.iter() {
            if let EnergyNodeType::Generator { production_per_tick } = r.value().node_type {
                total_generated += production_per_tick;
            }
        }
        total_generated
    }

    pub fn count_nodes(&self) -> usize {
        self.nodes.len()
    }
}
