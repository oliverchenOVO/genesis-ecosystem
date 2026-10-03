export interface DisplayOrganism {
  id: number;
  x: number;
  y: number;
  dx: number;
  dy: number;
  species_id: number;
  body_size: number;
  speed: number;
  carnivory: number;
}
export interface Snapshot {
  files: FileStatus;
  simulation_version: number;
  seed: string;
  tick: number;
  generation: number;
  size: number;
  population: number;
  species_count: number;
  temperature: number;
  regeneration: number;
  running: boolean;
  speed: number;
  cells: number[][];
  organisms: DisplayOrganism[];
  counters: {
    births: number;
    deaths: number;
    mutations: number;
    predations: number;
    peak_population: number;
  };
  autosave_error: string | null;
}
export interface Phenotype {
  body_size: number;
  speed: number;
  vision: number;
  metabolism: number;
  food_efficiency: number;
  reproduction_threshold: number;
  offspring_count: number;
  mutation_rate: number;
  aggression: number;
  social_affinity: number;
  temperature_optimum: number;
  temperature_tolerance: number;
  energy_capacity: number;
  carnivory: number;
}
export interface Organism {
  id: number;
  birth_tick: number;
  generation: number;
  parents: [number, number] | null;
  genome: number[];
  phenotype: Phenotype;
  x: number;
  y: number;
  energy: number;
  health: number;
  species_id: number;
  lineage_id: number;
  offspring: number;
  behavior: string;
}
export interface Species {
  id: number;
  name: string;
  ancestor: number | null;
  origin_tick: number;
  origin_generation: number;
  extinct_tick: number | null;
  founder: number[];
  founder_population: number;
  genetic_distance: number;
  population: number;
}
export interface SpeciesSample {
  species: number;
  population: number;
  means: number[];
  bounds: number[];
}
export interface Telemetry {
  tick: number;
  population: number;
  species_count: number;
  food: number;
  temperature: number;
  births: number;
  deaths: number;
  mutations: number;
  diversity: number;
  species: SpeciesSample[];
}
export interface HistoryEvent {
  id: number;
  tick: number;
  species: number | null;
  kind: string | Record<string, Record<string, number>>;
}
export type Page = "World" | "Species" | "Evolution" | "History";
export type Action =
  | { op: "snapshot" | "species" | "history" | "telemetry" | "replay" }
  | { op: "control"; running: boolean; speed: number }
  | { op: "environment"; temperature: number; regeneration: number }
  | { op: "detail"; id: number }
  | { op: "new"; config: WorldConfig }
  | { op: "forget_recent"; path: string }
  | { op: "save" | "load"; path: string | null };
export interface WorldConfig {
  seed: number;
  size: number;
  starting_population: number;
  population_limit: number;
  mutation_multiplier: number;
}
export interface FileStatus {
  dirty: boolean;
  current_path: string | null;
  warning: string | null;
  recent: { path: string; last_opened_ms: number; available: boolean }[];
}
