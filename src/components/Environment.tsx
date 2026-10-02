import { useState } from "react";
import type { Snapshot } from "../types";
export function Environment({
  snapshot,
  apply,
  busy,
}: {
  snapshot: Snapshot;
  apply: (temperature: number, regeneration: number) => void;
  busy: boolean;
}) {
  const [temperature, setTemperature] = useState(snapshot.temperature);
  const [regeneration, setRegeneration] = useState(snapshot.regeneration);
  return (
    <aside className="environment rail">
      <h2>Environment</h2>
      <label className="range-label">
        Temperature<strong>{(temperature / 100).toFixed(1)} °C</strong>
        <input
          type="range"
          min="-2000"
          max="6000"
          step="100"
          value={temperature}
          onChange={(e) => setTemperature(Number(e.target.value))}
        />
        <span className="range-ends">
          <span>−20 °C</span>
          <span>60 °C</span>
        </span>
      </label>
      <label className="range-label">
        Food regeneration<strong>{regeneration}</strong>
        <input
          type="range"
          min="0"
          max="100"
          value={regeneration}
          onChange={(e) => setRegeneration(Number(e.target.value))}
        />
        <span className="range-ends">
          <span>0</span>
          <span>100 / tick</span>
        </span>
      </label>
      <button
        className="primary full"
        disabled={busy}
        onClick={() => apply(temperature, regeneration)}
      >
        Apply environment
      </button>
      <div className="divider" />
      <h3>Recorded pressure</h3>
      <dl>
        <dt>World temperature</dt>
        <dd>{(snapshot.temperature / 100).toFixed(1)} °C</dd>
        <dt>Regeneration</dt>
        <dd>{snapshot.regeneration} / tick</dd>
        <dt>Generation</dt>
        <dd>{snapshot.generation}</dd>
      </dl>
      <p className="muted note">
        Change the habitat, then observe inherited traits. Generations follow
        parentage; ticks measure simulation time.
      </p>
      <div className="divider" />
      <h3>Life cycle</h3>
      <dl>
        <dt>Births</dt>
        <dd>{snapshot.counters.births.toLocaleString()}</dd>
        <dt>Deaths</dt>
        <dd>{snapshot.counters.deaths.toLocaleString()}</dd>
        <dt>Mutated loci</dt>
        <dd>{snapshot.counters.mutations.toLocaleString()}</dd>
        <dt>Predation</dt>
        <dd>{snapshot.counters.predations.toLocaleString()}</dd>
      </dl>
    </aside>
  );
}
