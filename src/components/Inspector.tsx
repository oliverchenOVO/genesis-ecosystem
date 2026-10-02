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
      <h3>Genome · 14 loci</h3>
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
