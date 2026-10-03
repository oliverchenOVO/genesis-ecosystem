use crate::{
    genetics::{Genome, Phenotype},
    rng::Streams,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SIMULATION_VERSION: u32 = 5;
/// Unreleased v5 revisions are isolated; published v4 remains immutable.
pub const SIMULATION_RULES_REVISION: u32 = 2;
pub const TELEMETRY_INTERVAL: u64 = 100;
macro_rules! id {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
        )]
        pub struct $name(pub u64);
    };
}
id!(OrganismId);
id!(SpeciesId);
id!(LineageId);
id!(EventId);
id!(GenomeId);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub seed: u64,
    pub size: i32,
    pub starting_population: usize,
    pub population_limit: usize,
    pub mutation_multiplier: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            seed: 42,
            size: 512,
            starting_population: 200,
            population_limit: 2000,
            mutation_multiplier: 100,
        }
    }
}
impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if ![256, 512, 1024].contains(&self.size) {
            return Err("World size must be 256, 512 or 1024".into());
        }
        if self.starting_population == 0
            || self.starting_population > self.population_limit
            || self.population_limit > 5000
        {
            return Err("Population must be 1–5000, within the population limit".into());
        }
        if self.mutation_multiplier > 500 {
            return Err("Mutation multiplier must be 0–500".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Cell {
    pub food: i32,
    pub fertility: i32,
    pub temperature_offset: i32,
    pub elevation: i32,
    pub moisture: i32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Environment {
    pub temperature: i32,
    pub regeneration: i32,
    pub cells: Vec<Cell>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Behavior {
    SeekFood,
    SeekMate,
    Flee,
    Hunt,
    Explore,
    Rest,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeathCause {
    Starvation,
    Age,
    Predation,
    Environment,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Organism {
    pub id: OrganismId,
    pub genome_id: GenomeId,
    pub birth_tick: u64,
    pub generation: u64,
    pub parents: Option<[OrganismId; 2]>,
    pub genome: Genome,
    pub phenotype: Phenotype,
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub energy: i32,
    pub health: i32,
    pub species_id: SpeciesId,
    pub lineage_id: LineageId,
    pub offspring: u32,
    pub last_mating: u64,
    pub last_attack: u64,
    pub behavior: Behavior,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ancestry {
    pub birth_tick: u64,
    pub parents: Option<[OrganismId; 2]>,
    pub death: Option<(u64, DeathCause)>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Lineage {
    pub id: LineageId,
    pub parent: Option<LineageId>,
    pub origin_tick: u64,
    pub founder: Genome,
    pub species_id: SpeciesId,
    pub candidate_since: Option<u64>,
    pub births: u64,
    pub cross_births: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Species {
    pub representative_genome: Genome,
    pub representative_morphology: crate::morphology::BodyMorphology,
    pub morphology_summary: [i32; 9],
    pub origin_environment: [i32; 3],
    pub niche: [i32; 6],
    pub feeding_observations: [u64; 4],
    pub innovation_streaks: [u32; 5],
    pub innovations: [bool; 5],
    pub id: SpeciesId,
    pub name: String,
    pub ancestor: Option<SpeciesId>,
    pub origin_tick: u64,
    pub origin_generation: u64,
    pub extinct_tick: Option<u64>,
    pub founder: Genome,
    pub founder_population: usize,
    pub genetic_distance: u32,
    pub population: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum HistoryKind {
    MorphologicalInnovation {
        index: usize,
    },
    SafetyPopulationCeiling {
        population: usize,
    },
    Origin,
    SpeciesCandidate {
        lineage: LineageId,
    },
    Speciation {
        ancestor: SpeciesId,
        distance: u32,
        founders: usize,
    },
    Extinction,
    Environment {
        temperature: i32,
        regeneration: i32,
    },
    PopulationMilestone {
        population: usize,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoricalEvent {
    pub id: EventId,
    pub tick: u64,
    pub species: Option<SpeciesId>,
    pub kind: HistoryKind,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpeciesTelemetry {
    pub morphology: [i32; 9],
    pub niche: [i32; 6],
    pub species: SpeciesId,
    pub population: usize,
    pub means: [i32; 6],
    pub bounds: [i32; 4],
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Telemetry {
    pub tick: u64,
    pub population: usize,
    pub species_count: usize,
    pub food: i64,
    pub temperature: i32,
    pub births: u64,
    pub deaths: u64,
    pub mutations: u64,
    pub diversity: u32,
    pub species: Vec<SpeciesTelemetry>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Command {
    pub tick: u64,
    pub temperature: i32,
    pub regeneration: i32,
}
impl Command {
    pub fn validate(&self) -> Result<(), String> {
        if !(-2000..=6000).contains(&self.temperature) || !(0..=100).contains(&self.regeneration) {
            return Err("Temperature must be -20–60°C; regeneration must be 0–100".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Counters {
    pub predator_starvation_deaths: u64,
    pub coexistence_ticks: u64,
    pub safety_ceiling_ticks: u64,
    pub births: u64,
    pub deaths: u64,
    pub mutations: u64,
    pub predations: u64,
    pub peak_population: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub config: Config,
    pub tick: u64,
    pub rng: Streams,
    pub environment: Environment,
    pub organisms: Vec<Organism>,
    pub ancestry: BTreeMap<OrganismId, Ancestry>,
    pub lineages: BTreeMap<LineageId, Lineage>,
    pub species: BTreeMap<SpeciesId, Species>,
    pub history: Vec<HistoricalEvent>,
    pub telemetry: Vec<Telemetry>,
    pub commands: Vec<Command>,
    pub next_organism: u64,
    pub next_lineage: u64,
    pub next_species: u64,
    pub counters: Counters,
}
