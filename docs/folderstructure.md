# folders

```Plaintext
📁 cimetro/
├── Cargo.toml               # Project manifest: dependencies, profiles, metadata
├── README.md                # Project overview, goals, architecture, contribution guide
├── docs/
│   ├── guidelines.md        # Architectural principles, coding standards, API-first rules
│   ├── manifest.md          # Full architecture manifest and module breakdown
│   ├── sprints.md           # Sprint planning and progress tracking
├── src/
│   ├── lib.rs               # Module exports for plugin structure
│   ├── main.rs              # Entry point: initializes Bevy App and registers plugins
│   ├── buildings/           # Building logic: spawning, upgrades, lots, placement
│   │   └── mod.rs
│   ├── core/                # Core systems: scheduler, event bus, config loader, RNG
│   │   └── mod.rs
│   ├── economy/             # Taxes, budget, production chains, trade, resource flow
│   │   └── mod.rs
│   ├── environment/         # Pollution, climate zones, weather, environmental effects
│   │   └── mod.rs
│   ├── governance/          # Districts, policies, regulations, political systems
│   │   └── mod.rs
│   ├── infra/               # Infrastructure: power, water, waste, networks, maintenance
│   │   └── mod.rs
│   ├── modding/             # Modding API: Lua/Python scripting, asset hooks, sandboxing
│   │   └── mod.rs
│   ├── persistence/         # Save/load system, serialization, versioning
│   │   └── mod.rs
│   ├── population/          # Citizens: households, demographics, satisfaction, lifecycle
│   │   └── mod.rs
│   ├── services/            # Civil services: police, fire, health, education, coverage
│   │   └── mod.rs
│   ├── traffic/             # Transport systems: roads, routing, vehicles, congestion
│   │   └── mod.rs
│   ├── ui/                  # User interface: panels, overlays, debug tools
│   │   └── mod.rs
│   └── world/               # World generation: grid, zoning, terrain, climate, agricultural zones
│       └── mod.rs
```Plaintext
