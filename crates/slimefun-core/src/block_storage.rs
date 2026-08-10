use dashmap::DashMap;
use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlimefunBlockData {
    pub world: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub item_id: String,
    pub extra_data: String,
    pub owner_uuid: Option<String>,
    pub last_tick_timestamp: u64,
}

pub struct BlockStorageEngine {
    blocks: DashMap<(String, i32, i32, i32), SlimefunBlockData>,
    /// Los bloques tocados desde el ultimo guardado. Sin esto, save_sqlite_db reescribia la
    /// tabla entera en cada pasada: en un mundo con decenas de miles de maquinas eso es un
    /// pico de disco por guardado, y el 99 % de las filas se reescriben identicas.
    dirty: DashMap<(String, i32, i32, i32), ()>,
    total_queries: AtomicU64,
}

impl BlockStorageEngine {
    pub fn new() -> Self {
        Self {
            blocks: DashMap::new(),
            dirty: DashMap::new(),
            total_queries: AtomicU64::new(0),
        }
    }

    /// Carga la base de datos SQLite actual de Slimefun4 (`stored-blocks.db`) sin resetear el servidor.
    pub fn load_sqlite_db<P: AsRef<Path>>(&self, db_path: P) -> SqlResult<usize> {
        let conn = Connection::open(db_path)?;
        
        // Crear tabla si no existe para compatibilidad total
        conn.execute(
            "CREATE TABLE IF NOT EXISTS slimefun_blocks (
                world TEXT NOT NULL,
                x INTEGER NOT NULL,
                y INTEGER NOT NULL,
                z INTEGER NOT NULL,
                id TEXT NOT NULL,
                data TEXT,
                PRIMARY KEY (world, x, y, z)
            )",
            [],
        )?;

        let mut stmt = conn.prepare("SELECT world, x, y, z, id, data FROM slimefun_blocks")?;
        let rows = stmt.query_map([], |row| {
            Ok(SlimefunBlockData {
                world: row.get(0)?,
                x: row.get(1)?,
                y: row.get(2)?,
                z: row.get(3)?,
                item_id: row.get(4)?,
                extra_data: row.get(5).unwrap_or_default(),
                owner_uuid: None,
                last_tick_timestamp: 0,
            })
        })?;

        let mut count = 0;
        for block_res in rows {
            if let Ok(block) = block_res {
                let key = (block.world.clone(), block.x, block.y, block.z);
                // Cargar no ensucia: lo que viene del disco ya esta guardado.
                self.blocks.insert(key, block);
                count += 1;
            }
        }

        Ok(count)
    }

    /// Guarda en SQLite **solo los bloques tocados** desde el guardado anterior.
    ///
    /// La marca de sucio se limpia despues del commit, no antes: si la transaccion falla, los
    /// bloques siguen pendientes y entran en la siguiente pasada. Limpiarla antes perderia
    /// cambios de forma silenciosa, que es la peor manera de perderlos.
    pub fn save_sqlite_db<P: AsRef<Path>>(&self, db_path: P) -> SqlResult<usize> {
        let pendientes: Vec<(String, i32, i32, i32)> =
            self.dirty.iter().map(|r| r.key().clone()).collect();

        if pendientes.is_empty() {
            return Ok(0);
        }

        let mut conn = Connection::open(db_path)?;
        let tx = conn.transaction()?;

        let mut count = 0;
        {
            let mut insert = tx.prepare(
                "INSERT OR REPLACE INTO slimefun_blocks (world, x, y, z, id, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
            )?;
            let mut delete = tx.prepare(
                "DELETE FROM slimefun_blocks WHERE world = ?1 AND x = ?2 AND y = ?3 AND z = ?4"
            )?;

            for key in &pendientes {
                match self.blocks.get(key) {
                    Some(r) => {
                        let b = r.value();
                        insert.execute(params![b.world, b.x, b.y, b.z, b.item_id, b.extra_data])?;
                    }
                    // Se marco sucio y ya no esta: lo rompieron. Hay que borrarlo de la tabla o
                    // reaparece en el siguiente arranque.
                    None => {
                        delete.execute(params![key.0, key.1, key.2, key.3])?;
                    }
                }
                count += 1;
            }
        }

        tx.commit()?;

        for key in pendientes {
            self.dirty.remove(&key);
        }

        Ok(count)
    }

    /// Cuantos bloques hay pendientes de guardar.
    pub fn dirty_count(&self) -> usize {
        self.dirty.len()
    }

    /// Fuerza el guardado completo marcando todo como pendiente. Para migraciones y para el
    /// volcado de apagado, donde interesa la copia entera y no el delta.
    pub fn mark_all_dirty(&self) {
        for r in self.blocks.iter() {
            self.dirty.insert(r.key().clone(), ());
        }
    }

    pub fn get_block(&self, world: &str, x: i32, y: i32, z: i32) -> Option<SlimefunBlockData> {
        self.total_queries.fetch_add(1, Ordering::Relaxed);
        let key = (world.to_string(), x, y, z);
        self.blocks.get(&key).map(|r| r.value().clone())
    }

    pub fn set_block(&self, block: SlimefunBlockData) {
        let key = (block.world.clone(), block.x, block.y, block.z);
        self.dirty.insert(key.clone(), ());
        self.blocks.insert(key, block);
    }

    pub fn remove_block(&self, world: &str, x: i32, y: i32, z: i32) -> Option<SlimefunBlockData> {
        let key = (world.to_string(), x, y, z);
        let quitado = self.blocks.remove(&key).map(|(_, b)| b);

        // Se marca aunque no existiera: es barato y evita depender de que el llamante acierte.
        if quitado.is_some() {
            self.dirty.insert(key, ());
        }

        quitado
    }

    pub fn count(&self) -> usize {
        self.blocks.len()
    }

    pub fn total_queries(&self) -> u64 {
        self.total_queries.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bloque(x: i32, id: &str) -> SlimefunBlockData {
        SlimefunBlockData {
            world: "world".into(),
            x,
            y: 64,
            z: 0,
            item_id: id.into(),
            extra_data: String::new(),
            owner_uuid: None,
            last_tick_timestamp: 0,
        }
    }

    fn ruta(nombre: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("sf-test-{}-{}.db", nombre, std::process::id()));
        let _ = std::fs::remove_file(&p);
        p
    }

    fn ids_en_disco(db: &std::path::Path) -> Vec<(i32, String)> {
        let conn = Connection::open(db).unwrap();
        let mut stmt = conn
            .prepare("SELECT x, id FROM slimefun_blocks ORDER BY x")
            .unwrap();
        let filas = stmt
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .map(|f| f.unwrap())
            .collect();
        filas
    }

    #[test]
    fn poner_un_bloque_lo_deja_pendiente() {
        let e = BlockStorageEngine::new();
        assert_eq!(e.dirty_count(), 0);
        e.set_block(bloque(1, "SOLAR_PANEL"));
        assert_eq!(e.dirty_count(), 1);
    }

    #[test]
    fn guardar_solo_escribe_lo_pendiente_y_limpia_la_marca() {
        let db = ruta("delta");
        let e = BlockStorageEngine::new();
        e.load_sqlite_db(&db).unwrap(); // crea la tabla

        e.set_block(bloque(1, "SOLAR_PANEL"));
        e.set_block(bloque(2, "COAL_GENERATOR"));
        assert_eq!(e.save_sqlite_db(&db).unwrap(), 2);
        assert_eq!(e.dirty_count(), 0);

        // Sin tocar nada, la siguiente pasada no escribe una sola fila.
        assert_eq!(e.save_sqlite_db(&db).unwrap(), 0);

        // Y si se toca uno solo, se escribe uno solo.
        e.set_block(bloque(2, "LAVA_GENERATOR"));
        assert_eq!(e.save_sqlite_db(&db).unwrap(), 1);

        assert_eq!(
            ids_en_disco(&db),
            vec![(1, "SOLAR_PANEL".to_string()), (2, "LAVA_GENERATOR".to_string())]
        );
        let _ = std::fs::remove_file(&db);
    }

    #[test]
    fn romper_un_bloque_lo_borra_del_disco() {
        // Sin esto el bloque reaparecia al arrancar: se quitaba de memoria pero la fila seguia
        // en la tabla, y el guardado por delta ya no reescribe todo como para taparlo.
        let db = ruta("borrado");
        let e = BlockStorageEngine::new();
        e.load_sqlite_db(&db).unwrap();

        e.set_block(bloque(1, "SOLAR_PANEL"));
        e.set_block(bloque(2, "COAL_GENERATOR"));
        e.save_sqlite_db(&db).unwrap();

        e.remove_block("world", 2, 64, 0);
        assert_eq!(e.dirty_count(), 1);
        e.save_sqlite_db(&db).unwrap();

        assert_eq!(ids_en_disco(&db), vec![(1, "SOLAR_PANEL".to_string())]);
        let _ = std::fs::remove_file(&db);
    }

    #[test]
    fn cargar_del_disco_no_ensucia() {
        let db = ruta("carga");
        let e = BlockStorageEngine::new();
        e.load_sqlite_db(&db).unwrap();
        e.set_block(bloque(1, "SOLAR_PANEL"));
        e.save_sqlite_db(&db).unwrap();

        let otro = BlockStorageEngine::new();
        assert_eq!(otro.load_sqlite_db(&db).unwrap(), 1);
        assert_eq!(otro.count(), 1);
        assert_eq!(
            otro.dirty_count(),
            0,
            "lo recien leido ya esta en disco, no hay nada que guardar"
        );
        assert_eq!(otro.save_sqlite_db(&db).unwrap(), 0);
        let _ = std::fs::remove_file(&db);
    }

    #[test]
    fn quitar_algo_que_no_existe_no_deja_marca() {
        let e = BlockStorageEngine::new();
        assert!(e.remove_block("world", 9, 9, 9).is_none());
        assert_eq!(e.dirty_count(), 0);
    }

    #[test]
    fn marcar_todo_fuerza_la_copia_entera() {
        let db = ruta("todo");
        let e = BlockStorageEngine::new();
        e.load_sqlite_db(&db).unwrap();
        e.set_block(bloque(1, "A"));
        e.set_block(bloque(2, "B"));
        e.save_sqlite_db(&db).unwrap();
        assert_eq!(e.save_sqlite_db(&db).unwrap(), 0);

        e.mark_all_dirty();
        assert_eq!(e.save_sqlite_db(&db).unwrap(), 2);
        let _ = std::fs::remove_file(&db);
    }
}
