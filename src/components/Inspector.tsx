import type { Organism, Snapshot, Species } from "../types";
export function Inspector({
  organism,
  snapshot,
  species,
  selectSpecies,
  missing,
}: {
  organism: Organism | null;
  snapshot: Snapshot;
  species: Species[];
  selectSpecies: (id: number) => void;
  missing: string;
}) {
  if (!organism)
    return (
      <aside className="rail inspector">
        <h2>Individual inspector</h2>
        <div className="empty-inspector">
          <span className="cell-mark" />
          <h3>Follow a life</h3>
          <p>
            {missing ||
              "Select an organism on the map or use the individual selector to see its inherited traits and ancestry."}
          </p>
        </div>
        <div className="divider" />
        <p className="muted note">
          Every visible trait comes from the Rust simulation. No narrative model
          controls this world.
        </p>
      </aside>
    );
  const sp = species.find((s) => s.id === organism.species_id);
  const p = organism.phenotype;
  const m = p.morphology;
  const traits: [string, number, string][] = [
    ["Speed", p.speed, "units / tick"],
    ["Body size", p.body_size, "units"],
    ["Vision", p.vision, "units"],
    ["Metabolism", p.metabolism, "energy / tick"],
    ["Aggression", p.aggression / 10, "%"],
    ["Thermal tolerance", p.temperature_tolerance / 100, "°C"],
  ];
  return (
    <aside className="rail inspector">
      <h2>Individual #{organism.id}</h2>
      <button
        className="species-link"
        onClick={() => selectSpecies(organism.species_id)}
      >
        {sp?.name ?? `Species #${organism.species_id}`}
      </button>
      <p className="muted">
        Generation {organism.generation} · {organism.behavior}
      </p>
      <dl>
        <dt>Age</dt>
        <dd>{snapshot.tick - organism.birth_tick} ticks</dd>
        <dt>Energy</dt>
        <dd>
          {organism.energy} / {p.energy_capacity}
        </dd>
        <dt>Health</dt>
        <dd>{organism.health}%</dd>
        <dt>Parents</dt>
        <dd>
          {organism.parents?.map((id) => `#${id}`).join(" × ") ?? "Founder"}
        </dd>
        <dt>Lineage</dt>
        <dd>#{organism.lineage_id}</dd>
        <dt>Offspring</dt>
        <dd>{organism.offspring}</dd>
      </dl>
      <div className="divider" />
      <h3>
        Body structure · {m.segment_count > 1 ? "Multi-unit" : "Single-unit"}
      </h3>
      <dl>
        <dt>Segments / appendages</dt>
        <dd>
          {m.segment_count} / {m.appendage_count}
        </dd>
        <dt>Body mass</dt>
        <dd>{m.mass} mass units</dd>
        <dt>Armor investment</dt>
        <dd>{(m.armor / 10).toFixed(1)}%</dd>
        <dt>Feeding structure</dt>
        <dd>
          {m.mouth} · bite {m.bite_capacity}
        </dd>
        <dt>Locomotion / sensory</dt>
        <dd>
          {m.locomotion_efficiency} / {m.sensory_investment}
        </dd>
        <dt>Movement coefficient</dt>
        <dd>{(m.movement_cost / 100).toFixed(2)}</dd>
        <dt>Construction investment</dt>
        <dd>{m.reproduction_cost} energy</dd>
        <dt>Complexity / maintenance</dt>
        <dd>
          {m.complexity} / {m.maintenance_cost} energy per tick
        </dd>
      </dl>
      <div className="divider" />
      <h3>Trait values</h3>
      {traits.map(([label, value, unit]) => (
        <div key={label} className="trait-row">
          <span>{label}</span>
          <strong>
            {value} <small>{unit}</small>
          </strong>
        </div>
      ))}
      <div className="divider" />
      <h3>Genome · {organism.genome.length} loci</h3>
      <div
        className="genome"
        aria-label={`Alleles: ${organism.genome.join(", ")}`}
      >
        {organism.genome.map((value, i) => (
          <span
            key={i}
            title={`Locus ${i + 1}: ${value}`}
            style={{
              height: `${8 + value / 100}px`,
              opacity: 0.3 + value / 1400,
            }}
          />
        ))}
      </div>
      <p className="muted note">Haploid regulatory alleles · bounded 0–1000</p>
    </aside>
  );
}
