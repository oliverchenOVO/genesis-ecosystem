import { describe, it, expect } from "vitest";
import { validateConfig } from "./components/NewWorld";
import { nearestOrganism } from "./ecosystem/WorldCanvas";
import { treePositions } from "./evolution/EvolutionView";
import { eventDescription } from "./history/HistoryView";
import { seriesPath } from "./components/Chart";
import type { Species, DisplayOrganism } from "./types";
describe("World configuration", () => {
  const config = {
    seed: 42,
    size: 512,
    starting_population: 200,
    population_limit: 2000,
    mutation_multiplier: 100,
  };
  it("accepts valid configuration", () =>
    expect(validateConfig(config)).toBeNull());
  it("rejects unsafe seeds and excess population", () => {
    expect(validateConfig({ ...config, seed: Infinity })).toMatch(/Seed/);
    expect(validateConfig({ ...config, starting_population: 3000 })).toMatch(
      /population/,
    );
    expect(validateConfig({ ...config, size: 0 })).toMatch(/size/);
  });
});
describe("Observation tools", () => {
  it("selects closest organism with stable ID ties", () => {
    const a = { id: 2, x: 10, y: 10 } as DisplayOrganism;
    const b = { id: 1, x: 10, y: 10 } as DisplayOrganism;
    expect(nearestOrganism([a, b], 10, 10, 5)?.id).toBe(1);
    expect(nearestOrganism([a], 100, 100, 5)).toBeUndefined();
  });
  it("places descendants after ancestors without mutating records", () => {
    const species = [
      { id: 2, ancestor: 1 },
      { id: 1, ancestor: null },
    ] as Species[];
    const positions = treePositions(species);
    expect(positions.get(2)!.x).toBeGreaterThan(positions.get(1)!.x);
    expect(species[0].id).toBe(2);
  });
  it("formats measured environmental evidence", () => {
    expect(
      eventDescription({
        id: 1,
        tick: 4,
        species: null,
        kind: { Environment: { temperature: 1250, regeneration: 7 } },
      }),
    ).toContain("12.5 °C");
  });
  it("handles empty and extinct chart series", () => {
    expect(seriesPath([])).toBe("");
    expect(seriesPath([0, 0])).not.toContain("NaN");
    expect(seriesPath([1, 2])).toContain("L400.00,0.00");
  });
});

import { formatTrait } from "./species/SpeciesView";
it("formats phenotype units without presenting centidegrees as degrees", () => {
  expect(formatTrait(4, 894)).toBe("8.94 °C");
  expect(formatTrait(5, 499)).toBe("49.9%");
  expect(formatTrait(3, 4)).toBe("4 energy / tick");
});

import { eventCategory } from "./history/HistoryView";
it("keeps population milestones distinct from environmental interventions", () => {
  expect(
    eventCategory({
      id: 1,
      tick: 1000,
      species: null,
      kind: { PopulationMilestone: { population: 200 } },
    }),
  ).toBe("Population");
  expect(
    eventCategory({ id: 2, tick: 1001, species: 1, kind: "Extinction" }),
  ).toBe("Evolution");
});
