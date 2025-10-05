# Cimetro Sprintplanung

## Hinweise zur Sprintplanung

- Sprintdauer: flexibel (1–3 Wochen), je nach verfügbarer Zeit
- Jeder Sprint liefert ein testbares, dokumentiertes Increment
- OKRs pro Sprint: klar definierte Ziele, messbare Fortschritte
- Dokumentation fortlaufend in `docs/` oder `manifest.md` ergänzen

---

## Sprint 1 — Core-Systeme: Scheduler, Eventbus, Config

**Ziel:** Basis für Zeitsteuerung, Ereignisverarbeitung und Konfiguration schaffen  
**Aufgaben:**

- `core_plugin` mit:
  - Scheduler (Tag/Nacht, Spielgeschwindigkeit, Timer)
  - Eventbus mit Event-Contracts (`PopulationChanged`, `BudgetUpdated`, etc.)
  - Deterministischer RNG (Seed-basiert)
- `config_loader` für TOML/JSON-basierte Parameter
- Debug-UI mit Bevy Inspector oder `bevy_egui`
- Erste Tests: Unit-Test für Events und Zeitsteuerung

---

## Sprint 2 — Weltgenerator & Save/Load

**Ziel:** Weltstruktur erzeugen und persistieren  
**Aufgaben:**

- `world_plugin` mit:
  - Grid-System (2D-Tiles)
  - Zoning: Residential, Commercial, Industrial, Agricultural
  - Tile-Komponenten und Platzierungslogik
- `persistence_plugin` mit:
  - Save/Load via `serde` + `ron` oder `toml`
  - Versionierung vorbereiten
- Visualisierung: einfache Tile-Darstellung mit Farben/Sprites

---

## Sprint 3 — Gebäude & Bevölkerung

**Ziel:** Erste Simulationseinheiten erzeugen  
**Aufgaben:**

- `buildings_plugin`:
  - Spawning-Logik
  - Upgrade-Stufen
  - Lot-Größen >4x4
- `population_plugin`:
  - Aggregierte Haushalte
  - Zufriedenheit, Jobzuweisung
  - Lebenszyklus (Basis)
- Event-Integration: `BuildingConstructed`, `PopulationChanged`

---

## Sprint 4 — Economy & Verkehr (Basis)

**Ziel:** Wirtschaftskreislauf und einfache Verkehrslogik  
**Aufgaben:**

- `economy_plugin`:
  - Steuern, Budget, laufende Kosten
  - Visualisierung im UI
- `traffic_plugin`:
  - Kapazitätsmodell für Straßen
  - einfache Routing-Logik (kein Pathfinding)
  - Transportmittel: PKW, Bus (Basis)
- Debug-Overlay für Verkehrsfluss

---

## Sprint 5 — Assets & Modding-Grundlagen

**Ziel:** Asset-Handling und Modding vorbereiten  
**Aufgaben:**

- `assets_plugin`:
  - Master Asset Collection laden
  - Profilbasiertes Streaming
- `modding_plugin`:
  - Lua-Sandbox
  - Hookpoints für Events
  - Beispiel-Mods (z. B. Gebäude-Skript)

---

## Sprint 6 — Dienste & Transporterweiterung

**Ziel:** Zivile Infrastruktur und Transportmittel ausbauen  
**Aufgaben:**

- `services_plugin`:
  - Polizei, Feuerwehr, Krankenhäuser
  - Coverage-Logik und Response-Zeiten
- `traffic_plugin`:
  - Erweiterung: Schienen, Häfen, Flughäfen
  - Tourismus-Transportmittel: Reisebus, Zug, Schiff, Flugzeug
- Event-Integration: `ServiceCoverageMissing`, `TouristArrived`

---

## Sprint 7+ — Erweiterung & Verfeinerung

**Ziel:** KI, ML, Regionen, Umwelt, Governance  
**Mögliche Themen:**

- KI-Hinweise zur Stadtentwicklung
- ONNX-Integration für Verkehrsfluss
- Regionale Interaktionen (SC4-Stil)
- Umweltmodell: Verschmutzung, Klima
- Bezirksverwaltung, Policies

---

## Abgeschlossene Sprints

## Sprint 0 — Projektstruktur & Setup

**Ziel:** Grundstruktur des Projekts erstellen, Plugin-Layout vorbereiten, Build & CI sicherstellen  
**Aufgaben:**

- Projekt initialisieren (`cargo init`)
- Plugin-Verzeichnisse und `mod.rs`-Dateien anlegen
- `main.rs` mit Bevy-Setup und Plugin-Platzhaltern erstellen
- `lib.rs` mit Modul-Exports
- `Cargo.toml` prüfen und optimieren
- Dokumentation: `README.md`, `architecture.md`, `manifest.md`, `guidelines.md`, `sprints.md`