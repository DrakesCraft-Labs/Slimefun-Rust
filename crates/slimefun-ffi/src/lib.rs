use jni::objects::{JClass, JIntArray};
use jni::sys::{jdouble, jint, jlong};
use jni::JNIEnv;
use slimefun_core::{BlockStorageEngine, TickerEngine};
use slimefun_energy::EnergyNetGraphSolver;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::OnceLock;

static STORAGE_ENGINE: OnceLock<BlockStorageEngine> = OnceLock::new();
static ENERGY_SOLVER: OnceLock<EnergyNetGraphSolver> = OnceLock::new();
static TICKER_ENGINE: OnceLock<TickerEngine> = OnceLock::new();
const ABI_VERSION: i32 = 1;

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

/// Suma valores enteros manteniendo la misma saturación que EnergyNet en Java.
#[no_mangle]
pub extern "C" fn slimefun_sum_saturating(values: *const i32, length: usize) -> i32 {
    if values.is_null() || length == 0 {
        return 0;
    }

    let slice = unsafe { std::slice::from_raw_parts(values, length) };
    sum_saturating(slice)
}

#[no_mangle]
pub extern "system" fn Java_io_github_thebusybiscuit_slimefun4_core_services_nativeengine_RustNativeEngine_nativeAbiVersion(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    ABI_VERSION
}

#[no_mangle]
pub extern "system" fn Java_io_github_thebusybiscuit_slimefun4_core_services_nativeengine_RustNativeEngine_nativeSumSaturating(
    env: JNIEnv,
    _class: JClass,
    values: JIntArray,
) -> jint {
    let Ok(length) = env.get_array_length(&values) else {
        return 0;
    };
    if length == 0 {
        return 0;
    }

    let mut buffer = vec![0; length as usize];
    if env.get_int_array_region(&values, 0, &mut buffer).is_err() {
        return 0;
    }
    sum_saturating(&buffer)
}

#[no_mangle]
pub extern "system" fn Java_io_github_thebusybiscuit_slimefun4_core_services_nativeengine_RustNativeEngine_nativeCalculateMarketPrice(
    _env: JNIEnv,
    _class: JClass,
    base_price: jdouble,
    demand: jlong,
    total_wealth: jdouble,
    reference_wealth: jdouble,
    minimum_factor: jdouble,
    maximum_factor: jdouble,
    demand_step: jdouble,
    maximum_demand_factor: jdouble,
    pulse_factor: jdouble,
) -> jdouble {
    calculate_market_price(
        base_price,
        demand,
        total_wealth,
        reference_wealth,
        minimum_factor,
        maximum_factor,
        demand_step,
        maximum_demand_factor,
        pulse_factor,
    )
}

fn sum_saturating(values: &[i32]) -> i32 {
    values.iter().copied().fold(0_i32, i32::saturating_add)
}

#[allow(clippy::too_many_arguments)]
fn calculate_market_price(
    base_price: f64,
    demand: i64,
    total_wealth: f64,
    reference_wealth: f64,
    minimum_factor: f64,
    maximum_factor: f64,
    demand_step: f64,
    maximum_demand_factor: f64,
    pulse_factor: f64,
) -> f64 {
    let safe_reference = reference_wealth.max(1.0);
    let wealth_ratio = total_wealth.max(0.0) / safe_reference;
    let wealth_factor = 0.90 + wealth_ratio.ln_1p() / 11.0_f64.ln() * 0.65;
    let demand_factor = maximum_demand_factor.min(1.0 + demand.max(0) as f64 * demand_step);
    let combined =
        (wealth_factor * demand_factor * pulse_factor).clamp(minimum_factor, maximum_factor);
    (base_price * combined * 100.0).round().max(1.0) / 100.0
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

#[cfg(test)]
mod tests {
    use super::{calculate_market_price, sum_saturating};

    #[test]
    fn sums_regular_values() {
        assert_eq!(sum_saturating(&[10, 20, 30]), 60);
    }

    #[test]
    fn saturates_overflow_and_underflow() {
        assert_eq!(sum_saturating(&[i32::MAX, 1]), i32::MAX);
        assert_eq!(sum_saturating(&[i32::MIN, -1]), i32::MIN);
        assert_eq!(sum_saturating(&[i32::MAX, 1, -1]), i32::MAX - 1);
    }

    #[test]
    fn calculates_bounded_market_price() {
        assert_eq!(
            calculate_market_price(100.0, 0, 0.0, 100_000_000.0, 0.85, 1.85, 0.02, 1.45, 1.0),
            90.0
        );
        assert_eq!(
            calculate_market_price(
                100.0,
                1_000,
                1_000_000_000.0,
                100_000_000.0,
                0.85,
                1.85,
                0.02,
                1.45,
                1.0,
            ),
            185.0
        );
    }
}
