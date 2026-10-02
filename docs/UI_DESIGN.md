# UI concept and tokens

Concept: `UI_CONCEPT.png`, generated with built-in ImageGen for the full primary desktop screen. Implementation follows its dark natural-history research station: charcoal #111a18, mint #b4e6bc, muted #99aaa2, borders #33483e. Segoe UI/Inter fallback; 13px controls, 14px body, 18px panels, 28px page headings. Four navigation destinations: World, Species, Evolution, History. Map and observation rails, followed by a three-chart telemetry band.

Intentional specification corrections: the concept image invented 64 DNA genes and a notes editor; the implementation uses the requested 14 actual regulatory loci and no notes editor. No simulated numbers are copied from the concept. Environment interventions, seed, world lifecycle and all series come from Rust. Map uses phenotype-derived ellipses/triangles as explicitly requested, not generated sprites. Resource and temperature layers use code-native authoritative fields. Added accessible individual selector and measured lifecycle counters support required acceptance flows.

Responsive design retains all controls: inspector moves below the map at tablet widths; rails and charts stack at mobile widths. Keyboard focus is visible. The New World dialog uses native modal focus trapping. Evolution pan/zoom is independent of world state.

## Visual and interaction verification

Compared concept and saved desktop screenshot through view_image: four destinations and three-column observation layout match; typography uses system fonts and the same modest hierarchy; charcoal/mint tokens match; code-native phenotype shapes replace decorative generated creatures; actual seed/traits/telemetry replace all concept sample numbers. Added lifecycle counters and accessible selector. Desktop map height was adjusted so the three real charts are visible together; observation rails scroll when needed. Mobile 390×844 and tablet 1024×768 have no horizontal overflow. World, organism selection, New World seed/population, play/pause/speed, keyboard temperature intervention, History, manual save/load, replay at tick537, species detail, tree zoom/reset and node navigation were exercised through the browser against the real Rust worker. Browser error/warning log was empty at the end. Production native page load and worker startup are checked separately; browser testing does not claim native IPC coverage.


Final v4 screenshots replace earlier captures. The actual seed11 world at107504 ticks contains200 organisms and two naturally formed species; the selected organism is generation124 with recorded parents. The second species has origin generation60 at tick48600. Temperature60C/no food caused two real extinctions and manual Load restored the saved living world. Latest390px QA has no horizontal overflow and no new error/warning logs after reconnect. Final screenshots were inspected with view_image.

