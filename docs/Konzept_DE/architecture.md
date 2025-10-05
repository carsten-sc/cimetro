# Problems of CSII

Jede einzelne Person, jedes Fahrzeug, jede Route wird individuell berechnet
Verkehrs-, Strom-, Wasser-, Müll-, Bildungs- und Gesundheitsnetzwerke sind miteinander verknüpft
Pathfinding-Algorithmen laufen in Echtzeit für Tausende Entitäten
Modding-Schnittstellen und dynamische UI-Elemente erzeugen zusätzliche Last

Virtual Texturing verursacht Grafikfehler und Nachladezeiten
Volumetrics, Depth of Field, Motion Blur sind schlecht implementiert und kosten unnötig FPS
Dynamic Resolution funktioniert nicht zuverlässig
Multithreading wird nicht optimal genutzt – viele Aufgaben laufen auf einem Hauptthread.

## Lösungen bei CSII

1. Bessere Thread-Verteilung
Aktuell nutzt CSII nicht alle Kerne effizient – viele Tasks hängen am Hauptthread Mit echter Multithreading-Optimierung könnten auch Ryzen 5 5600X oder i5-12400 mithalten

2. Granularitätsreduktion
Die Simulation berechnet jede Person, jedes Fahrzeug einzeln.
Eine gröbere Simulation (z. B. Gruppen statt Individuen) würde die CPU-Last massiv senken.

3. GPU-Entlastung
Viele Berechnungen (z. B. Verkehr, Sichtlinien) könnten teilweise auf die GPU ausgelagert werden
Das würde die CPU entlasten und auch Mittelklasse-Systeme stabiler machen

4. Asset-Streaming & Caching
Mods und Assets werden aktuell nicht effizient geladen
Mit besserem Asset-Management könnten auch Systeme mit weniger RAM und Cache mithalten

## Ziele

1. NAM/traffic Manager integrieren
2. Modding Schnittstellen
3. Möglichst vorhandene 3D Modelle integrieren
4. Scripting via Lua und Python
5. Optimales Multithreading
6. Resourcenschondend
7. Master Asset Sammlung, die in allen Städten geladen werden
8. Effizientes Asset Loading. Profile für unterschiedliche Städte, so das nicht alle geladen werden
9. Realistische Karten
10. In späteren Versionen regionsbasiert
11. Stadtübergreifende Features wie (Regionsflughafen, Hafen, Energie, Synergien zwischen Städten)
12. Stadtgröße wie bei SC4
13. Seeverkehr via Fluss, Meer
14. Eisenbahn, Autobahn, Flugverkehr übergreifend (siehe 9)
15. Nachrüstbare Verkehrssysteme, wie in NAM
16. KI Verkehrplanung Bürgerverhalten etc.
17. KI gibt Hinweise auf Entwicklung von Stadtvierteln unter Berücksichtigung von Maßnahmen wie Straßenbau oder Zivile Gebäude
18. Gute Bedienbarkeit und Freiheit beim Platzieren von Props
19. Freie Platzierung von Straßen auch Kurven.
20. Ähnlich auch Brücken und Autobahnen, wie bei CS
21. Zoning von Industrie, Wohnung, Gewerbe und Erholungsgebieten.
22. Polizei, Feuerwehrr, Krnakenwagen Friedhöfe Müll etc. realistisch umgesetzt.
23. Kriminalität steigt, wenn keine/zu wenig Polizei.
24. Dito Feuerwehr
25. Ohne Müllabfuhr wächst Verschmutzung
26. Kanalisation nur grundlegend, also Wasserpumpen, Klärwerke etc. Von diesen müssen Gebiete angeschlossen werden.
27. Stadtviertel, welche je nach Art (Industrie, Wohn Gewerbe etc) bestimmte zivilgebäude haben müssen.
28. Stadtviertel können vom Spieler eingegrenzt, erweitert werden und sind Grundbedingung.
29. Lots sollen größer als 4x4 sein, ähnlich wie bei SC4.
30. Prüfe ob Nvidia Karte vorhanden:CUDA/OpenCL (über Bindings): Wenn du NVIDIA-GPUs gezielt einsetzen willst, kannst du Rust-Bindings für CUDA nutzen (z. B. rust-cuda). Dies ist jedoch komplexer und weniger portabel.
31. ONNX oder TensorFlow: Für maschinelles Lernen kannst du ONNX-Modelle in Rust (z. B. mit tract) oder Python-basierte Frameworks wie PyTorch integrieren, wobei letzteres die Integration in Bevy erschwert.
32. Tag/Nacht Modus

## 🧠 Ziel: Eine KI-gestützte Städtebausimulation mit modularer Architektur

* Modularität: Trennung von Simulation, Rendering, UI und KI
* Skalierbarkeit: Läuft auf Mittelklasse-Hardware, skaliert aber bis High-End
* KI-Integration: Für Verhalten, Optimierung, Vorhersage und Assistenz
* GPU-Entlastung: Nutzung von Compute-Shaders für Pathfinding, Verkehr, etc.

```Plaintext
+-----------------------------+
|        Game Engine          | ← Bevy
+-----------------------------+
|  Simulation Core (CPU)      |
|  ├── Zonen & Gebäude        |
|  ├── Wirtschaft & Budget    |
|  ├── Bevölkerung & Demografie|
|  └── Ereignisse & Policies  |
+-----------------------------+
|  AI Layer (GPU + CPU)       |
|  ├── Pathfinding (GPU)      |
|  ├── Verkehrsverhalten (ML) |
|  ├── Bürgerlogik (RL)       |
|  └── Stadtplanungshilfe     |
+-----------------------------+
|  Asset Manager              |
|  ├── LOD-Streaming           |
|  ├── Modding API            |
|  └── Caching & Preloading   |
+-----------------------------+
|  Rendering Engine (GPU)     |
|  ├── Vulkan/DirectX 12      |
|  ├── DLSS/XeSS/FSR Support  |
|  └── Raytracing optional    |
+-----------------------------+
|  UI & UX Layer              |
|  ├── Responsive UI          |
|  ├── KI-gestützte Tipps     |
|  └── Accessibility Features |
+-----------------------------+
```

## 🧠 KI-Einsatzbereiche

### 🧭 Verkehrssteuerung

* Reinforcement Learning (RL) für dynamische Ampelschaltung, Stauvermeidung
* GPU-gestützte Pathfinding via Compute-Shaders → Echtzeit-Routenberechnung

### 🧑‍🤝‍🧑 Bürgerverhalten

* Agentenbasierte Simulation mit ML-Modellen für realistische Entscheidungen z. B. „Gehe ich zur Arbeit, zur Schule oder bleibe ich zu Hause?“

### 🏗️ Stadtplanungshilfe

* KI schlägt optimale Zonenverteilung vor basierend auf Nachfrage, Verkehrsfluss, Umwelt z. B. „Hier wäre ein Park sinnvoll, um Luftqualität zu verbessern“

### 📈 Vorhersagemodelle

* KI prognostiziert wirtschaftliche Entwicklung, Bevölkerungswachstum, Verkehrsaufkommen. z. B. „In 5 Jahren wird dieses Viertel überlastet sein – baue jetzt eine Umgehungsstraße“

### 🔧 Technische Highlights

* Multithreading mit Task-Scheduler → Simulation läuft parallel zur UI
* GPU-Compute für Pathfinding & Crowd Simulation → entlastet CPU
* Modding-Sandbox mit Scriptable Objects → Lua, Python oder C#-API
* Asset-Streaming mit LODs & Caching → keine Ladezeiten trotz großer Städte.
