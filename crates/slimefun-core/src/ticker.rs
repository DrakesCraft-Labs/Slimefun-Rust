use crate::block_storage::BlockStorageEngine;
use std::sync::atomic::{AtomicU64, Ordering};

/// Resultado de un tick. Lleva los bloques pendientes para que el llamante sepa que el motor
/// todavia no los procesa, en vez de recibir un numero suelto que parece un exito.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickReport {
    pub tick: u64,
    pub pending_blocks: usize,
}

pub struct TickerEngine {
    tick_count: AtomicU64,
}

impl TickerEngine {
    pub fn new() -> Self {
        Self {
            tick_count: AtomicU64::new(0),
        }
    }

    /// Avanza el contador de ticks y devuelve cuantos bloques habria que procesar.
    ///
    /// **Todavia no ejecuta la logica de las maquinas.** Antes esta funcion decia hacer un tick
    /// paralelo con rayon a velocidad de nanosegundos; lo unico que hacia era contar bloques y
    /// tirar el resultado. Se deja dicho aqui para que nadie construya encima creyendo que el
    /// tick ya funciona: falta el modelo de comportamiento de cada maquina, que es el trabajo de
    /// verdad, y sin el no hay nada que paralelizar.
    pub fn tick_all_machines(&self, storage: &BlockStorageEngine) -> TickReport {
        let tick = self.tick_count.fetch_add(1, Ordering::Relaxed);

        TickReport {
            tick,
            pending_blocks: storage.count(),
        }
    }

    pub fn current_tick(&self) -> u64 {
        self.tick_count.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn el_contador_avanza_uno_por_tick() {
        let t = TickerEngine::new();
        let s = BlockStorageEngine::new();

        assert_eq!(t.tick_all_machines(&s).tick, 0);
        assert_eq!(t.tick_all_machines(&s).tick, 1);
        assert_eq!(t.current_tick(), 2);
    }

    #[test]
    fn informa_de_los_bloques_que_aun_no_procesa() {
        let t = TickerEngine::new();
        let s = BlockStorageEngine::new();
        s.set_block(crate::block_storage::SlimefunBlockData {
            world: "world".into(),
            x: 0,
            y: 64,
            z: 0,
            item_id: "SOLAR_PANEL".into(),
            extra_data: String::new(),
            owner_uuid: None,
            last_tick_timestamp: 0,
        });

        assert_eq!(t.tick_all_machines(&s).pending_blocks, 1);
    }
}
