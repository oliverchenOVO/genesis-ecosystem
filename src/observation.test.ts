import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import { Chart, seriesMaximum, seriesPath } from "./components/Chart";
import { filterSpecies, SpeciesView } from "./species/SpeciesView";
import type { Species } from "./types";

describe("Measured chart time", () => {
  it("positions irregular observations at their actual ticks", () => {
    expect(seriesPath([2, 4, 8], 400, 100, [100, 200, 1000])).toBe(
      "M0.00,75.00 L44.44,50.00 L400.00,0.00",
    );
  });
  it("handles long measured histories without overflowing argument limits", () => {
    const values = Array.from({ length: 200000 }, (_, i) => i % 1000);
    expect(seriesMaximum(values)).toBe(999);
    expect(seriesPath(values).endsWith("L400.00,0.00")).toBe(true);
  });
  it("renders the only measured point and labels its sample tick", () => {
    const html = renderToStaticMarkup(
      createElement(Chart, {
        label: "Population",
        values: [8],
        ticks: [4000],
      }),
    );
    expect(html).toContain("<circle");
    expect(html).toContain("Latest sample · tick 4,000");
    expect(html).not.toContain("Waiting for the next sample");
  });
  it("does not present empty data as a measurement at tick zero", () => {
    const html = renderToStaticMarkup(
      createElement(Chart, {
        label: "Food",
        values: [],
        ticks: [],
      }),
    );
    expect(html).toContain("Waiting for the first sample");
    expect(html).not.toContain("Latest sample");
    expect(html).not.toContain("<circle");
  });
});

describe("Species discovery", () => {
  const species = [
    { id: 1, name: "Alga", extinct_tick: null },
    { id: 11, name: "Hunter", extinct_tick: 0 },
  ] as Species[];
  it("combines living status with case-insensitive name search", () => {
    expect(filterSpecies(species, "Living", " ALg ").map((s) => s.id)).toEqual([
      1,
    ]);
    expect(filterSpecies(species, "Living", "hunter")).toEqual([]);
    expect(filterSpecies(species, "Extinct", "").map((s) => s.id)).toEqual([
      11,
    ]);
  });
  it("matches exact hash-prefixed IDs and preserves the input records", () => {
    const original = structuredClone(species);
    expect(filterSpecies(species, "All", "#1").map((s) => s.id)).toEqual([1]);
    expect(filterSpecies(species, "All", "#11").map((s) => s.id)).toEqual([11]);
    expect(filterSpecies(species, "All", "#999")).toEqual([]);
    expect(species).toEqual(original);
  });
  it("renders a genuinely empty registry without stale species details", () => {
    const html = renderToStaticMarkup(
      createElement(SpeciesView, {
        species: [],
        telemetry: [],
        selected: 11,
        select: () => {},
      }),
    );
    expect(html).toContain("No species recorded.");
    expect(html).toContain("0 of 0 species");
    expect(html).not.toContain("species-detail");
  });
});
