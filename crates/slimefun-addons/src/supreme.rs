use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupremeCardSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub multiplier: f64,
}

pub struct SupremeAddonEngine;

impl SupremeAddonEngine {
    pub fn get_card_specs() -> Vec<SupremeCardSpec> {
        vec![
            SupremeCardSpec { id: "SUPREME_SPEED_CARD_1", name: "Tarjeta de Velocidad I", multiplier: 1.5 },
            SupremeCardSpec { id: "SUPREME_SPEED_CARD_2", name: "Tarjeta de Velocidad II", multiplier: 2.0 },
            SupremeCardSpec { id: "SUPREME_ENERGY_CARD_1", name: "Tarjeta de Eficiencia Energética I", multiplier: 0.75 },
            SupremeCardSpec { id: "SUPREME_GENERATOR_QUANTUM", name: "Generador Cuántico Supremo", multiplier: 5.0 },
        ]
    }
}
