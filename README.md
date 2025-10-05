# Cimetro

**Cimetro** is a modular, extensible city-building simulation written in Rust using the [Bevy](https://bevyengine.org/) game engine. It is designed for long-term scalability, data-driven architecture, and community modding support.

It's name stands for Carsten's simulation of a metropolis.

## Vision

Cimetro aims to become a feature-rich, open-source simulation platform inspired by classics like SimCity and Cities: Skylines — but built from the ground up with modern software engineering practices, plugin-based architecture, and AI-assisted development.

## Key Features

- 🧱 Plugin-based architecture (`core`, `world`, `population`, `economy`, etc.)
- 🧠 API-first design with clear module boundaries
- 🕒 Event-driven simulation with scheduler and time control
- 🌍 Procedural world generation with zoning and climate support
- 🚗 Traffic systems with multimodal transport and congestion modeling
- 🏢 Building lifecycle, upgrades, and placement logic
- 🧑‍🤝‍🧑 Population simulation with needs, jobs, and satisfaction
- 💰 Budget, taxation, and economic systems
- 🧰 Modding API with Lua/Python scripting
- 🧠 Optional ML integration (ONNX, tract) and GPU acceleration (CUDA/OpenCL)

## Project Structure

See [`docs/manifest.md`](docs/manifest.md) for a full breakdown of modules and architecture.

## Getting Started

```bash
git clone https://github.com/yourusername/cimetro.git
cd cimetro
cargo run
```

Requires Rust 1.74+ and Bevy 0.17.2.

## Contributing

Cimetro is built in iterative sprints. See docs/sprints.md for current goals. Contributions are welcome — especially in simulation logic, UI, modding, and asset design.

## License

MIT — free to use, modify, and distribute.
