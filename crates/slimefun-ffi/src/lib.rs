use slimefun_core::{BlockStorageEngine, TickerEngine};
use slimefun_energy::EnergyNetGraphSolver;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::OnceLock;

static STORAGE_ENGINE: OnceLock<BlockStorageEngine> = OnceLock::new();
static ENERGY_SOLVER: OnceLock<EnergyNetGraphSolver> = OnceLock::new();
static TICKER_ENGINE: OnceLock<TickerEngine> = OnceLock::new();

fn get_storage() -> &'static BlockStorageEngine {
    STORAGE_ENGINE.get_or_init(BlockStorageEngine::new)
}

fn get_energy_solver() -> &'static EnergyNetGraphSolver {
    ENERGY_SOLVER.get_or_init(EnergyNetGraphSolver::new)
}

fn get_ticker() -> &'static TickerEngine {
    TICKER_ENGINE.get_or_init(TickerEngine::new)
}

/// Carga la base de datos SQLite actual de Slimefun (`stored-blocks.db`) sin resetear el servidor.
#[no_mangle]
pub extern "C" fn slimefun_load_sqlite_db(path_ptr: *const c_char) -> i64 {
    if path_ptr.is_null() {
        return -1;
    }
    let c_str = unsafe { CStr::from_ptr(path_ptr) };
    if let Ok(path_str) = c_str.to_str() {
        let storage = get_storage();
        if let Ok(count) = storage.load_sqlite_db(path_str) {
            return count as i64;
        }
    }
    -1
}

/// Guarda el estado actual en la base de datos SQLite de Slimefun (`stored-blocks.db`).
#[no_mangle]
pub extern "C" fn slimefun_save_sqlite_db(path_ptr: *const c_char) -> i64 {
    if path_ptr.is_null() {
        return -1;
    }
    let c_str = unsafe { CStr::from_ptr(path_ptr) };
    if let Ok(path_str) = c_str.to_str() {
        let storage = get_storage();
        if let Ok(count) = storage.save_sqlite_db(path_str) {
            return count as i64;
        }
    }
    -1
}

/// Resuelve el ciclo de energía y el tick de todas las máquinas en velocidad nativa C/Rust (nanosegundos).
#[no_mangle]
pub extern "C" fn slimefun_execute_tick_cycle() -> u64 {
    let ticker = get_ticker();
    let storage = get_storage();
    let solver = get_energy_solver();
    
    let _energy_generated = solver.solve_tick();
    ticker.tick_all_machines(storage)
}

/// Devuelve el número total de bloques registrados en BlockStorage.
#[no_mangle]
pub extern "C" fn slimefun_get_total_block_count() -> usize {
    get_storage().count()
}

/// Libera la memoria de cadenas C devueltas por Rust.
#[no_mangle]
pub extern "C" fn slimefun_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
