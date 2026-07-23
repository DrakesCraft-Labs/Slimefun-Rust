<div align="center">

<img src="https://raw.githubusercontent.com/DrakesCraft-Labs/Slimefun-Rust/main/slimefun_rust_banner.svg" alt="Slimefun-Rust Engine Banner" width="920" />

# 🧪 Slimefun-Rust Engine

**Motor Unificado de Alto Rendimiento en Rust 2021 para Slimefun4 Core y los 44 Addons de DrakesCraft**

<p>
  <a href="https://github.com/DrakesCraft-Labs/Slimefun-Rust"><img src="https://img.shields.io/badge/GitHub-Slimefun--Rust-181717?style=for-the-badge&logo=github" alt="GitHub"/></a>
  <img src="https://img.shields.io/badge/Rust-2021_Workspace-FF4500?style=for-the-badge&logo=rust&logoColor=white" alt="Rust 2021"/>
  <img src="https://img.shields.io/badge/Java-21_FFM_Panama-F89820?style=for-the-badge&logo=openjdk&logoColor=white" alt="Java 21 FFM"/>
  <img src="https://img.shields.io/badge/Purpur-1.21.11-00FF66?style=for-the-badge&logo=minecraft&logoColor=white" alt="Purpur 1.21.11"/>
  <img src="https://img.shields.io/badge/Addons-44_Integrated-00F2FE?style=for-the-badge" alt="44 Addons"/>
</p>

</div>

---

## 📖 Descripción General

`Slimefun-Rust` es una arquitectura de **Monorepo en Rust (Workspace 2021)** diseñada para reemplazar el cuello de botella de rendimiento de Slimefun4 y sus 44 Addons en servidores Minecraft de alta demanda (Purpur 1.21.11).

Procesa las redes eléctricas (**EnergyNet**), las redes de transporte de ítems (**CargoNet**), la persistencia espacial SQLite y los tickers de máquinas en sub-hilos de CPU a velocidad nativa (nanosegundos) **sin pausas de Garbage Collector (GC)**.

---

## 🏛️ Estructura del Monorepo

```
Slimefun-Rust/
 ├── Cargo.toml                       <-- Workspace Configuration
 ├── slimefun_rust_banner.svg         <-- Animated SVG Banner
 └── crates/
      ├── slimefun-core/              <-- BlockStorage (SQLite/JSON), Ítems y Ticker Paralelo
      ├── slimefun-energy/            <-- EnergyNet Graph Solver (Rayon + Petgraph)
      ├── slimefun-cargo/             <-- CargoNet Item Router & Node Logistics
      ├── slimefun-addons/            <-- Motor unificado para los 44 Addons de Slimefun
      ├── slimefun-ffi/               <-- C-ABI Shared Library (.dll / .so) para Java 21 Panama FFM
      └── slimefun-server/            <-- Microservicio Standalone en Axum/Tokio (Puerto 8085)
```

---

## 🛡️ Cero Reset de Servidor (`BlockStorageEngine`)

El crate `slimefun-core` implementa una lectura/escritura bidireccional sobre la base de datos SQLite `stored-blocks.db` nativa de Slimefun4:

```rust
use slimefun_core::BlockStorageEngine;

let storage = BlockStorageEngine::new();
// Carga transaccional directa de stored-blocks.db
let total_loaded = storage.load_sqlite_db("data-storage/Slimefun/stored-blocks.db")?;
println!("Bloques Slimefun cargados en memoria nativa: {}", total_loaded);
```

---

## 📋 Tabla de los 44 Addons Integrados

| # | Addon de Slimefun | Categoría | Módulo en Rust |
| :-: | :--- | :--- | :--- |
| **1** | **NetworksV6-Drake** | `CARGO_ENERGY` | `networks.rs` |
| **2** | **FluffyMachines** | `AUTOMATION` | `fluffy_machines.rs` |
| **3** | **Supreme-Drake** | `ADVANCED_TECH` | `supreme.rs` |
| **4** | **DynaTech** | `AUTOMATION` | `dynatech.rs` |
| **5** | **InfinityExpansion** | `ENDGAME` | `infinity_expansion.rs` |
| **6** | **AdvancedTech** | `MACHINERY` | `all_addons.rs` |
| **7** | **AlchimiaVitae** | `MAGIC` | `all_addons.rs` |
| **8** | **BetterChests** | `STORAGE` | `all_addons.rs` |
| **9** | **ChestTerminal** | `STORAGE` | `all_addons.rs` |
| **10** | **ColoredEnderChests** | `STORAGE` | `all_addons.rs` |
| **11** | **Cultivation** | `AGRICULTURE` | `all_addons.rs` |
| **12** | **DankTech2** | `STORAGE` | `all_addons.rs` |
| **13** | **DyeBench** | `CRAFTING` | `all_addons.rs` |
| **14** | **DyedBackpacks** | `STORAGE` | `all_addons.rs` |
| **15** | **EcoPower** | `ENERGY` | `all_addons.rs` |
| **16** | **ElectricSpawners** | `MOBS` | `all_addons.rs` |
| **17** | **ElementManipulation** | `MAGIC` | `all_addons.rs` |
| **18** | **ExoticGarden** | `AGRICULTURE` | `all_addons.rs` |
| **19** | **ExtraGear** | `WEAPONS_ARMOR` | `all_addons.rs` |
| **20** | **ExtraTools** | `TOOLS` | `all_addons.rs` |
| **21** | **FastMachines** | `AUTOMATION` | `all_addons.rs` |
| **22** | **FlowerPower** | `AGRICULTURE` | `all_addons.rs` |
| **23** | **FNAmplifications** | `MACHINERY` | `all_addons.rs` |
| **24** | **FoxyMachines** | `AUTOMATION` | `all_addons.rs` |
| **25** | **Galaxyfun** | `SPACE_ENDGAME` | `all_addons.rs` |
| **26** | **GeneticChickengineering** | `MOBS` | `all_addons.rs` |
| **27** | **HotbarPets** | `ITEMS` | `all_addons.rs` |
| **28** | **LiteXpansion** | `MACHINERY` | `all_addons.rs` |
| **29** | **MobCapturer** | `MOBS` | `all_addons.rs` |
| **30** | **MoreResearches** | `RESEARCH` | `all_addons.rs` |
| **31** | **PotionExpansion** | `MAGIC` | `all_addons.rs` |
| **32** | **RelicsOfCthonia** | `MAGIC_WEAPONS` | `all_addons.rs` |
| **33** | **SensibleToolbox** | `AUTOMATION` | `all_addons.rs` |
| **34** | **SFCalc** | `UTILITY` | `all_addons.rs` |
| **35** | **SimpleMaterialGenerators** | `GENERATORS` | `all_addons.rs` |
| **36** | **SlimeFrame** | `UTILITY` | `all_addons.rs` |
| **37** | **SlimefunLuckyBlocks** | `FUN` | `all_addons.rs` |
| **38** | **SlimeHUD** | `UI` | `all_addons.rs` |
| **39** | **SlimeTinker** | `WEAPONS` | `all_addons.rs` |
| **40** | **SlimyTreeTaps** | `AGRICULTURE` | `all_addons.rs` |
| **41** | **SoulJars** | `MOBS` | `all_addons.rs` |
| **42** | **TranscEndence** | `ENDGAME` | `all_addons.rs` |
| **43** | **WorldEditSlimefun** | `WORLD_BUILDING` | `all_addons.rs` |
| **44** | **DrakesSlimeMarket** | `ECONOMY` | `all_addons.rs` |

---

## 🛠️ Compilación y Construcción

```bash
# Comprobar la compilación de todos los crates del Monorepo
cargo check --workspace

# Compilar binarios de producción (slimefun-server.exe y slimefun_ffi.dll)
cargo build --release --workspace
```

---

<div align="center">

**DrakesCraft Labs** · Desarrollado por [**JackStar6677-1**](https://github.com/JackStar6677-1)

</div>
