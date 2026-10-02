import { useState } from "react";
import { Chart } from "../components/Chart";
import type { Species, Telemetry } from "../types";
const traitNames = [
  "Body size",
  "Speed",
  "Vision",
  "Metabolism",
  "Thermal tolerance",
  "Aggression",
];
export function formatTrait(index: number, value: number): string {
  if (index === 4) return `${(value / 100).toFixed(2)} °C`;
  if (index === 5) return `${(value / 10).toFixed(1)}%`;
  return `${value} ${index === 3 ? "energy / tick" : index === 1 ? "units / tick" : "units"}`;
}
export function SpeciesView({
  species,
  telemetry,
  selected,
  select,
}: {
  species: Species[];
  telemetry: Telemetry[];
  selected: number | null;
  select: (id: number) => void;
}) {
  const [filter, setFilter] = useState("All");
  const detail = species.find((s) => s.id === selected) ?? species[0];
  const samples = telemetry.map((t) => ({
    tick: t.tick,
    data: t.species.find((s) => s.species === detail?.id),
  }));
  const latest = samples.filter((s) => s.data).at(-1)?.data;
  const first = samples.find((s) => s.data)?.data;
  return (
    <div className="species-workspace">
      <section className="species-list rail">
        <h2>Species registry</h2>
        <div className="segments">
          {["All", "Living", "Extinct"].map((f) => (
            <button
              key={f}
              onClick={() => setFilter(f)}
              className={filter === f ? "active" : ""}
            >
              {f}
            </button>
          ))}
        </div>
        {species
          .filter(
            (s) =>
              filter === "All" ||
              (filter === "Living"
                ? s.extinct_tick === null
                : s.extinct_tick !== null),
          )
          .map((s) => (
            <button
              className={`species-row ${detail?.id === s.id ? "selected" : ""}`}
              key={s.id}
              onClick={() => select(s.id)}
            >
              <span
                className={`species-dot ${s.extinct_tick !== null ? "extinct" : ""}`}
              />
              <span>
                <strong>{s.name}</strong>
                <small>Origin tick {s.origin_tick.toLocaleString()}</small>
              </span>
              <b>{s.population.toLocaleString()}</b>
            </button>
          ))}
        <p className="muted note">
          A new species requires sustained genetic divergence and restricted
          gene flow. Extinct records remain available.
        </p>
      </section>
      {detail ? (
        <section className="species-detail">
          <div className="detail-heading">
            <h1>{detail.name}</h1>
            <span
              className={
                detail.extinct_tick === null ? "living-label" : "muted"
              }
            >
              {detail.extinct_tick === null
                ? "Living"
                : `Extinct at tick ${detail.extinct_tick}`}
            </span>
          </div>
          <div className="facts">
            <div>
              <span>Population</span>
              <strong>{detail.population.toLocaleString()}</strong>
            </div>
            <div>
              <span>Origin</span>
              <strong>
                Generation {detail.origin_generation} · tick{" "}
                {detail.origin_tick.toLocaleString()}
              </strong>
            </div>
            <div>
              <span>Founder population</span>
              <strong>{detail.founder_population}</strong>
            </div>
            <div>
              <span>Genetic distance</span>
              <strong>{detail.genetic_distance} / 1000</strong>
            </div>
          </div>
          <p className="muted">
            Ancestor:{" "}
            {detail.ancestor ? (
              <button
                className="inline-link"
                onClick={() => select(detail.ancestor!)}
              >
                {species.find((s) => s.id === detail.ancestor)?.name ??
                  `#${detail.ancestor}`}
              </button>
            ) : (
              "Initial living population"
            )}
          </p>
          <div className="divider" />
          <h2>Measured trait averages</h2>
          <div className="trait-grid">
            {traitNames.map((name, i) => (
              <div className="trait-row" key={name}>
                <span>{name}</span>
                <strong>
                  {latest ? formatTrait(i, latest.means[i]) : "—"}
                </strong>
                <small>
                  {latest && first
                    ? `${latest.means[i] - first.means[i] >= 0 ? "+" : ""}${formatTrait(i, latest.means[i] - first.means[i])} since first sample`
                    : ""}
                </small>
              </div>
            ))}
          </div>
          <p className="muted note">
            Spatial distribution bounds:{" "}
            {latest
              ? `(${latest.bounds[0]}, ${latest.bounds[1]}) → (${latest.bounds[2]}, ${latest.bounds[3]})`
              : "No living distribution"}{" "}
            · means sampled every 100 ticks.
          </p>
          <div className="detail-charts">
            <Chart
              label="Population history"
              values={samples.map((s) => s.data?.population ?? 0)}
              ticks={samples.map((s) => s.tick)}
            />
            <Chart
              label="Mean body size"
              values={samples
                .filter((s) => s.data)
                .map((s) => s.data!.means[0])}
              ticks={samples.filter((s) => s.data).map((s) => s.tick)}
            />
            <Chart
              label="Mean speed"
              values={samples
                .filter((s) => s.data)
                .map((s) => s.data!.means[1])}
              ticks={samples.filter((s) => s.data).map((s) => s.tick)}
            />
          </div>
        </section>
      ) : (
        <p>No species recorded.</p>
      )}
    </div>
  );
}
