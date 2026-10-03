import { useState } from "react";
import type { HistoryEvent, Species } from "../types";
export function eventCategory(event: HistoryEvent): string {
  if (typeof event.kind === "string") return "Evolution";
  if (
    "PopulationMilestone" in event.kind ||
    "SafetyPopulationCeiling" in event.kind
  )
    return "Population";
  if ("Environment" in event.kind) return "Environment";
  return "Evolution";
}
export function eventDescription(event: HistoryEvent): string {
  if (typeof event.kind === "string")
    return event.kind === "Origin"
      ? "Initial life established"
      : event.kind === "Extinction"
        ? "Species became extinct"
        : event.kind;
  const [type, payload] = Object.entries(event.kind)[0];
  switch (type) {
    case "Environment":
      return `Habitat changed to ${(payload.temperature / 100).toFixed(1)} °C · food regeneration ${payload.regeneration}`;
    case "Speciation":
      return `Species diverged from #${payload.ancestor} · distance ${payload.distance}/1000 · ${payload.founders} founders`;
    case "SpeciesCandidate":
      return `Lineage #${payload.lineage} met the divergence criteria; persistence under observation`;
    case "PopulationMilestone":
      return `${payload.population.toLocaleString()} living organisms recorded`;
    case "SafetyPopulationCeiling":
      return `Technical safety ceiling reached at ${payload.population.toLocaleString()} organisms`;
    case "MorphologicalInnovation":
      return (
        [
          "Multi-unit morphology persisted",
          "Armor investment persisted",
          "Piercer morphology persisted with observed kills",
          "Large-body morphology persisted",
          "Sensory expansion persisted",
        ][payload.index] ?? "Unknown morphology observation"
      );
    default:
      return type;
  }
}
export function HistoryView({
  history,
  species,
  select,
}: {
  history: HistoryEvent[];
  species: Species[];
  select: (id: number) => void;
}) {
  const [filter, setFilter] = useState("All");
  const names = new Map(species.map((s) => [s.id, s.name]));
  return (
    <section className="history-workspace">
      <div className="view-heading">
        <div>
          <h1>Natural history</h1>
          <p className="muted">Chronological evidence from this world</p>
        </div>
        <label>
          Show
          <select value={filter} onChange={(e) => setFilter(e.target.value)}>
            {["All", "Evolution", "Environment", "Population"].map((f) => (
              <option key={f}>{f}</option>
            ))}
          </select>
        </label>
      </div>
      <ol className="timeline">
        {history
          .filter(
            (event) => filter === "All" || eventCategory(event) === filter,
          )
          .map((event) => (
            <li key={event.id}>
              <time>Tick {event.tick.toLocaleString()}</time>
              <span className="timeline-point" />
              <div>
                <h3>{eventDescription(event)}</h3>
                {event.species !== null ? (
                  <button
                    className="inline-link"
                    onClick={() => select(event.species!)}
                  >
                    {names.get(event.species) ?? `Species #${event.species}`}
                  </button>
                ) : (
                  <span className="muted">{eventCategory(event)} record</span>
                )}
              </div>
            </li>
          ))}
      </ol>
    </section>
  );
}
