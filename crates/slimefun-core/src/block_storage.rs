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
    total_queries: AtomicU64,
}

impl BlockStorageEngine {
    pub fn new() -> Self {
        Self {
            blocks: DashMap::new(),
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
                self.blocks.insert(key, block);
                count += 1;
            }
        }

        Ok(count)
    }

    /// Guarda todos los bloques modificados directamente en la base de datos SQLite de Slimefun.
    pub fn save_sqlite_db<P: AsRef<Path>>(&self, db_path: P) -> SqlResult<usize> {
        let mut conn = Connection::open(db_path)?;
        let tx = conn.transaction()?;

        let mut count = 0;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO slimefun_blocks (world, x, y, z, id, data) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
            )?;

            for r in self.blocks.iter() {
                let b = r.value();
                stmt.execute(params![b.world, b.x, b.y, b.z, b.item_id, b.extra_data])?;
                count += 1;
            }
        }

        tx.commit()?;
        Ok(count)
    }

    pub fn get_block(&self, world: &str, x: i32, y: i32, z: i32) -> Option<SlimefunBlockData> {
        self.total_queries.fetch_add(1, Ordering::Relaxed);
        let key = (world.to_string(), x, y, z);
        self.blocks.get(&key).map(|r| r.value().clone())
    }

    pub fn set_block(&self, block: SlimefunBlockData) {
        let key = (block.world.clone(), block.x, block.y, block.z);
        self.blocks.insert(key, block);
    }

    pub fn remove_block(&self, world: &str, x: i32, y: i32, z: i32) -> Option<SlimefunBlockData> {
        let key = (world.to_string(), x, y, z);
        self.blocks.remove(&key).map(|(_, b)| b)
    }

    pub fn count(&self) -> usize {
        self.blocks.len()
    }

    pub fn total_queries(&self) -> u64 {
        self.total_queries.load(Ordering::Relaxed)
    }
}
