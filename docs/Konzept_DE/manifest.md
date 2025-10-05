
# Logic

column0 | column1
------- | -------
Modul | Aufgabe
Engine | Bevy-Setup, Fenster, Input, Kamera, Zeitsteuerung
Simulation | Zonen, Gebäude, Bevölkerung, Wirtschaft, Ereignisse
AI | Verkehrsverhalten, Bürgerlogik, Stadtplanung, Pathfinding
Assets | Laden, Caching, Modding, LOD-Streaming
Rendering | 2D/3D Darstellung, Shader, Raytracing, DLSS/XeSS/FSR
UI/UX | Menüs, Tooltips, Platzierungs-UI, Accessibility
Config | UserSettings, GameMode, Profile-Handling
Events | Interne Kommunikation, z. B. Gebäude platziert, Budget geändert
Save/Load | Spielstände, Szenen, Serialisierung
Audio | Soundeffekte, Musik, Audio-Feedback
Map/World | Kartenlogik, Regionen, Flüsse, Gelände
Systems | Zeitsteuerung, Ticks, Scheduler, SystemSets
Debug/DevTools | Sandbox-Modus, Logging, Performance-Monitoring

```Plaintext
src/
├── main.rs
├── engine/                  # Bevy Setup, Kamera, Input
│   ├── mod.rs
│   ├── window.rs
│   └── input.rs
├── simulation/             # Stadtlogik
│   ├── mod.rs
│   ├── zoning.rs
│   ├── buildings.rs
│   ├── economy.rs
│   ├── population.rs
│   └── events.rs
├── ai/                     # KI & Verhalten
│   ├── mod.rs
│   ├── traffic.rs
│   ├── citizens.rs
│   └── planning.rs
├── assets/                 # Asset-Handling
│   ├── mod.rs
│   ├── loader.rs
│   ├── cache.rs
│   └── modding.rs
├── render/                 # Rendering & Grafik
│   ├── mod.rs
│   ├── pipeline.rs
│   ├── effects.rs
│   └── raytracing.rs
├── ui/                     # Benutzeroberfläche
│   ├── mod.rs
│   ├── layout.rs
│   ├── interaction.rs
│   └── accessibility.rs
├── config/                 # Einstellungen & Profile
│   ├── mod.rs
│   └── user_settings.rs
├── events/                 # Globale Events
│   ├── mod.rs
│   └── building_events.rs
├── save_load/              # Spielstände
│   ├── mod.rs
│   └── serializer.rs
├── audio/                  # Sound & Musik
│   ├── mod.rs
│   └── sound.rs
├── map/                    # Karten & Regionen
│   ├── mod.rs
│   ├── terrain.rs
│   └── regions.rs
├── systems/                # Zeitsteuerung & Scheduler
│   ├── mod.rs
│   └── tick.rs
├── debug/                  # Sandbox, Logging, DevTools
│   ├── mod.rs
│   └── sandbox.rs
```

---

## Cimetro Architekturmanifest

## 1. Zielsetzung

Modulare, erweiterbare Städtebausimulation mit iterativem Sprint‑Workflow (SCRUM / OKR‑freundlich). Beginn mit einem Minimal Viable Simulation (MVS), die in Genauigkeit und Umfang wächst. Divide et impera: klare Plugin-Grenzen. Stabile APIs, data‑driven Parameter, niedrige Kopplung und klar definierte Event‑Contracts, um spätere Refactors zu minimieren, API-first Design.

---

## 2. Kernübersicht

* Welt  
* Bevölkerung  
* Gebäude  
* Verkehr und Transportmittel  
* Zivile Dienste  
* Infrastruktur und Versorgung  
* Economy und Logistik  
* Governance und Verwaltung  
* Regionen und Intercity‑Features  
* Gameplay‑Schichten: UI, Story, Rewards, Events, Scheduler  
* Tools / Technik: Assets, Modding, Load/Save, Debug, Config

---

## 3. Komponentenbaum — Detailliert

### Core‑Systeme

* **Scheduler**  
  * Zeitsteuerung: Tag/Nacht, Jahreszeiten, Spielgeschwindigkeit  
  * Wiederkehrende Jobs, Timers, Cron‑ähnliche Aufgaben  
* **Event Bus**  
  * Zentrale Event API und Event‑Contracts (z. B. PopulationChanged, PowerOutage, TrafficJam)  
* **Statistik**  
  * Laufende Metriken: Bevölkerung, Budget, Umwelt, Verkehr, Zufriedenheit  
* **Deterministischer RNG**  
  * Reproduzierbare Simulationen für Tests  
* **Logiksteuerung**  
  * Spielregeln und Validierungen (z. B. kein Verkehr ohne Bevölkerung)

#### Welt & Regionen

* **Weltgenerator**  
  * Grid/Tile oder Voxel‑Basis; Placement/Snap‑Logik; realistische Karten  
  * Zoning: Residential, Commercial, Industrial, Mixed, Special, Agricultural  
* **Regionen**  
  * Rural/Urban‑Klassifikation; SC4‑ähnliche Regionen; Intercity‑Features (Regionenflughafen, Hafen, gemeinsame Energie/Handel)  
* **Umwelt & Klima**  
  * Klimazonen, Wetter, Umweltverschmutzung, Bodenversiegelung, Mikroklima  
* **Rendering**  
  * Day/Night, Wettervisualisierung, Licht & Schatten

#### Bevölkerung & Gesellschaft

* **Demografie**  
  * Haushalte, Altersstruktur, Migration, Geburten/Sterbefälle  
* **Bürger**  
  * Bedürfnisse, Beschäftigung, Bildung, Gesundheit, Lebenszyklus  
* **Soziale Dynamiken**  
  * Kriminalität, Segregation, Zufriedenheitsverteilung  
* **Zivile Dienste**  
  * Schulen, Krankenhäuser, Polizei, Feuerwehr, Freizeit  
* **Stadtviertel / Bezirke**  
  * Player definierbare Districts; Regeln, notwendige Einrichtungen

#### Infrastruktur & Versorgung

* **Basisinfrastruktur**  
  * Strom, Wasser, Abwasser, Müll, Internet/Netzwerke  
* **Kanalisation (grundlegend)**  
  * Wasserpumpen, Klärwerke, Anschlusslogik für Gebiete  
* **Wartung & Alterung**  
  * Infrastrukturzustand, Wartungskosten, Ausfallraten  
* **Master Asset Sammlung & Loading**  
  * Effizientes Streaming, City‑Profile, Memory Management

#### Gebäude & Bau

* **Gebäude Lifecycle**  
  * Spawning, Entwicklung / Upgrade‑Stufen, Baukosten  
* **Lots & Parzellen**  
  * Konfigurierbare Lots, Standard >4x4 möglich  
* **Platzierungstools**  
  * Freie Prop‑Platzierung, Straßenkurven, Brücken, Höhenhandling, gute Bedienbarkeit

#### Verkehr & Transport

* **Infrastrukturtypen**  
  * Straßen, Schienen, Häfen, Flughäfen, ÖPNV‑Infrastruktur  
* **Transportmittel — Güter**  
  * LKW, Güterzug, Frachtschiff  
* **Transportmittel — Personen / Tourismus**  
  * PKW, Reisebusse, Personenzüge, Fähren/Schiffe, Flugzeuge  
* **Erweiterbarkeit**  
  * NAM‑style Hookpoints; nachrüstbare Verkehrssysteme  
* **Verkehrsmodell**  
  * Routing, Kapazitätsmodelle, Stau, ÖPNV‑Takte, multimodale Logistikketten  
* **Tourismus**  
  * Besucherströme, Unterkunfts‑ & Transportnachfrage

#### Wirtschaft & Logistik

* **Economy**  
  * Steuern, Haushalt, Einnahmen/Ausgaben, Budgetverwaltung  
  * Produktionsketten, Preise, Angebot/Nachfrage, Arbeitsmarkt  
* **Ressourcen & Handel**  
  * Lager, Nahrungsimport, Güterversorgung, interregionale Logistik  
* **Finanzinstrumente**  
  * Kredite, Investoren, Projektfinanzierung

#### Governance & Verwaltung

* **Politik & Programme**  
  * Verordnungen, Subventionen, kommunale Programme  
* **Bezirksverwaltung**  
  * District Management, Policies, Regionale Regeln

#### Gameplay, UI & Audio

* **UI / Dialoge**  
  * Panels, Kartenansichten, Tooltips, Debug‑Panels  
* **Story & Events**  
  * Szenarien, Missionen, Katastrophen, Sonderereignisse  
* **Belohnungssystem**  
  * Achievements, Progression, Boni  
* **User Interaction**  
  * Kamera, Selektion, Bauen/Abreißen, Edit‑Modus  
* **Audio**  
  * User MP3 Playlist, Soundsteuerung, Effekte

#### Tools, Modding & Technik

* **Modding API**  
  * Script‑Hooks (Lua, Python), Event‑Subscriptions, Asset‑Hooks, Sandbox  
* **Assets**  
  * Master Asset Collection; Integration vorhandener 3D‑Modelle; profilbasiertes Laden  
* **Performance & Multithreading**  
  * Optimales Multithreading, Ressourcenschonung, Profiling, LOD, Streaming  
* **ML & GPU Optionen**  
  * ONNX / tract (Inference in Rust), Python‑Bridges; optional CUDA/OpenCL Bindings als Prüfpfad  
* **Persistence**  
  * Save/Load, Versioning, Migration Scripts  
* **Debugging**  
  * Debug‑UI, Visualizer, Profiling Overlays, Deterministic Replay

---

## 4. Architekturprinzipien zur Stabilität

* API‑first: frühe, stabile Contracts zwischen Plugins  
* Data‑driven: Balancing & Regeln extern konfigurierbar (JSON/TOML/CSV)  
* Layered fidelity: mit aggregierten Modellen starten, selektiv granularer werden  
* Event‑contracts: zentrales Event‑Bus statt direkter Cross‑Module‑Calls  
* Profilbasiertes Asset Loading: City‑Profile, Memory Limits, Streaming

---

## 5. Minimal Viable Simulation (MVS)

Startumfang zum schnellen, sicheren Prototypenbau:

* Weltgenerator (Grid + einfache Zonen)
* Gebäude‑Spawning mit 2–3 Entwicklungsstufen  
* Aggregierte Haushalte (Population) mit Jobzuweisung und Zufriedenheit  
* Simplifizierte Verkehrssimulation (Kapazitätschecks, kein Full Pathfinding)  
* Einfache Economy: Steuern, laufende Kosten, Budgetanzeige  
* Core: Scheduler, Eventbus, Deterministic RNG, Debug‑UI, Save/Load Basis

---

## 6. Plugin‑Matrix (Bevy‑orientiert, Empfehlung)

* core_plugin: scheduler, event_bus, config, stats  
* world_plugin: generator, grid, terrain, climate_base  
* population_plugin: households, demography, life_cycle  
* economy_plugin: tax_system, budget, trade_imports  
* buildings_plugin: spawner, upgrade_rules, parcel_system  
* infra_plugin: power, water, waste, networks, maintenance  
* traffic_plugin: roads, rails, ports, airports, routing, congestion  
* services_plugin: health, education, police, fire, service_coverage  
* environment_plugin: pollution_model, climate_events  
* governance_plugin: districts, policies, regulations  
* ui_plugin: core_ui, debug_ui, panels  
* persistence_plugin: save_load, serialization, migration  
* modding_plugin: script_api, asset_api, hooks

---

## 7. Sprint‑Vorschlag (flexibel)

Sprintdauer: 1–3 Wochen je nach Verfügbarkeit. Jeder Sprint liefert ein shippable Increment.

* **Sprint 0 — Setup**  
  Projekttemplate, Bevy + Rust, Folder/Plugin Skeleton, CI, Config Schema

* **Sprint 1 — Core MVS**  
  Scheduler, Eventbus, Deterministic RNG, Debug‑UI, Config Loader

* **Sprint 2 — World & Persistence**  
  Grid generator, einfache Zonen, Placement API; Save/Load

* **Sprint 3 — Buildings & Population**  
  Spawner, Upgrade‑Rules; Haushalte, Jobs, Zufriedenheit

* **Sprint 4 — Economy & Basic Traffic**  
  Taxes, Budget; Capacity Checks, Simple Routing; UI Panels

* **Sprint 5 — Assets & Modding Foundations**  
  Master Collection ingest, Streaming Profiles; Lua Sandbox + Hooks

* **Sprint 6 — Transport Expansion & Services**  
  Rails, Ports, Airports; Police
  
## 9. Risiken und Gegenmaßnahmen

* Zu frühe Detailtiefe → Starte MVS + klare Abstraktionen
* Enge Kopplung → Eventbus + Query APIs
* Asset Memory Blowup → Profiling Streaming City Profiles
* Modding Sicherheitsrisiken → Sandbox Lua restriktive API
