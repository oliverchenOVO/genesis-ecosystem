use crate::{
    genetics::{Genome, LOCI},
    model::*,
    rng::Streams,
    spatial::{distance_squared, Spatial, CELL_SIZE},
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub struct World {
    pub state: State,
}
#[derive(Clone, Copy)]
struct Intent {
    behavior: Behavior,
    x: i32,
    y: i32,
    target: Option<usize>,
}

impl World {
    pub fn new(config: Config) -> Result<Self, String> {
        config.validate()?;
        let mut rng = Streams::new(config.seed);
        let side = config.size / CELL_SIZE;
        let cells: Vec<Cell> = (0..side * side)
            .map(|index| Cell {
                food: 300 + rng.world.below(700) as i32,
                hard_food: 0,
                resource_remainders: [0; 2],
                fertility: 60 + rng.world.below(81) as i32,
                temperature_offset: ((index / side / 4 + (config.seed % 7) as i32) % 5 - 2) * 250
                    + rng.world.below(201) as i32
                    - 100,
                elevation: ((index % side / 4 + (config.seed % 7) as i32) % 5) * 180
                    + rng.world.below(201) as i32,
                moisture: ((index / side / 4 + index % side / 4) % 5) * 180
                    + rng.world.below(201) as i32,
            })
            .map(|mut cell| {
                cell.initialize_channels();
                cell
            })
            .collect();
        let mut base = Genome([500; LOCI]);
        base.0[14..].copy_from_slice(&[200, 500, 125, 200, 100, 200, 300, 200]);
        let mut organisms = Vec::new();
        let mut ancestry = BTreeMap::new();
        for i in 1..=config.starting_population as u64 {
            let mut genome = base.clone();
            for g in &mut genome.0[..14] {
                *g = (450 + rng.world.below(101)) as u16;
            }
            // Dietary tendency is heritable. Founders include natural standing variation.
            genome.0[13] = if rng.world.below(10) == 0 { 800 } else { 200 };
            for g in &mut genome.0[14..] {
                *g = (i32::from(*g) + rng.world.below(51) as i32 - 25).clamp(0, 1000) as u16;
            }
            genome.0[17] = if genome.0[13] > 650 {
                800
            } else if rng.world.below(4) == 0 {
                500
            } else {
                200
            };
            let phenotype = genome.phenotype();
            organisms.push(Organism {
                id: OrganismId(i),
                genome_id: GenomeId(i),
                birth_tick: 0,
                generation: 0,
                parents: None,
                x: rng.world.below(config.size as u64) as i32,
                y: rng.world.below(config.size as u64) as i32,
                dx: 0,
                dy: 0,
                energy: phenotype.energy_capacity * 2 / 3,
                health: 100,
                species_id: SpeciesId(1),
                lineage_id: LineageId(1),
                offspring: 0,
                last_mating: 0,
                last_attack: 0,
                behavior: Behavior::Explore,
                genome,
                phenotype,
            });
            ancestry.insert(
                OrganismId(i),
                Ancestry {
                    birth_tick: 0,
                    parents: None,
                    death: None,
                },
            );
        }
        let mut lineages = BTreeMap::new();
        lineages.insert(
            LineageId(1),
            Lineage {
                id: LineageId(1),
                parent: None,
                origin_tick: 0,
                founder: base.clone(),
                species_id: SpeciesId(1),
                candidate_since: None,
                births: 0,
                cross_births: 0,
            },
        );
        let mut species = BTreeMap::new();
        species.insert(
            SpeciesId(1),
            Species {
                representative_genome: organisms[0].genome.clone(),
                representative_morphology: organisms[0].phenotype.morphology.clone(),
                morphology_summary: [0; 9],
                origin_environment: [
                    2000,
                    12,
                    cells
                        [(organisms[0].y / CELL_SIZE * side + organisms[0].x / CELL_SIZE) as usize]
                        .elevation,
                ],
                niche: [0; 6],
                feeding_observations: [0; 4],
                innovation_streaks: [0; 5],
                innovations: [false; 5],
                id: SpeciesId(1),
                name: scientific_name(config.seed, 1),
                ancestor: None,
                origin_tick: 0,
                origin_generation: 0,
                extinct_tick: None,
                founder: base,
                founder_population: organisms.len(),
                genetic_distance: 0,
                population: organisms.len(),
            },
        );
        let initial = organisms.len();
        let mut world = Self {
            state: State {
                config,
                tick: 0,
                rng,
                environment: Environment {
                    temperature: 2000,
                    regeneration: 12,
                    cells,
                },
                organisms,
                ancestry,
                lineages,
                species,
                history: vec![],
                telemetry: vec![],
                commands: vec![],
                next_organism: initial as u64 + 1,
                next_lineage: 2,
                next_species: 2,
                counters: Counters {
                    peak_population: initial,
                    ..Counters::default()
                },
            },
        };
        world.event(Some(SpeciesId(1)), HistoryKind::Origin);
        world.sample();
        Ok(world)
    }

    pub fn command(&mut self, command: Command) -> Result<(), String> {
        command.validate()?;
        if command.tick != self.state.tick {
            return Err("Command tick must match current world tick".into());
        }
        self.state.environment.temperature = command.temperature;
        self.state.environment.regeneration = command.regeneration;
        self.event(
            None,
            HistoryKind::Environment {
                temperature: command.temperature,
                regeneration: command.regeneration,
            },
        );
        self.state.commands.push(command);
        Ok(())
    }

    pub fn advance(&mut self, ticks: u64) {
        for _ in 0..ticks {
            self.step();
        }
    }

    fn event(&mut self, species: Option<SpeciesId>, kind: HistoryKind) {
        self.state.history.push(HistoricalEvent {
            id: EventId(self.state.history.len() as u64 + 1),
            tick: self.state.tick,
            species,
            kind,
        });
    }

    pub fn step(&mut self) {
        #[cfg(feature = "profile")]
        let mut profile_stamp = std::time::Instant::now();
        let s = &mut self.state;
        s.tick += 1;
        #[cfg(feature = "viability")]
        crate::ecological_diagnostics::tick(s.tick);
        #[cfg(feature = "viability")]
        for o in &s.organisms {
            crate::viability::tick(o, eligible(o, s.tick));
        }
        for cell in &mut s.environment.cells {
            cell.regenerate_channels(s.environment.regeneration);
        }
        #[cfg(feature = "profile")]
        crate::profile::mark("resource regeneration", &mut profile_stamp);
        let spatial = Spatial::new(s.config.size, &s.organisms);
        let side = s.config.size / CELL_SIZE;
        // Perception and intent: no writes to any other organism.
        let intents: Vec<Intent> = s
            .organisms
            .iter()
            .enumerate()
            .map(|(i, o)| {
                let p = &o.phenotype;
                let mut best_threat = None;
                let mut best_prey = None;
                let mut best_mate = None;
                for j in spatial.nearby(o.x, o.y, p.vision) {
                    if i == j {
                        continue;
                    }
                    let other = &s.organisms[j];
                    let d = distance_squared(o.x, o.y, other.x, other.y);
                    if d > p.vision.pow(2) {
                        continue;
                    }
                    let candidate = (d, other.id, j);
                    if other.phenotype.carnivory > 650
                        && other.phenotype.morphology.can_attack(&p.morphology)
                        && p.carnivory <= 650
                        && best_threat.is_none_or(|v| candidate < v)
                    {
                        best_threat = Some(candidate);
                    }
                    if p.carnivory > 650
                        && p.morphology.can_attack(&other.phenotype.morphology)
                        && other.phenotype.carnivory <= 650
                        && best_prey.is_none_or(|v| candidate < v)
                    {
                        best_prey = Some(candidate);
                    }
                    if other.energy >= other.phenotype.reproduction_threshold
                        && best_mate.is_none_or(|v| candidate < v)
                        && o.genome.compatible(&other.genome)
                    {
                        best_mate = Some(candidate);
                    }
                }
                #[cfg(feature = "viability")]
                crate::viability::perception(o, best_mate.is_some(), best_prey.is_some());
                let mut choices = vec![(
                    10,
                    5,
                    Intent {
                        behavior: Behavior::Rest,
                        x: o.x,
                        y: o.y,
                        target: None,
                    },
                )];
                if let Some((d, _, j)) = best_threat {
                    let t = &s.organisms[j];
                    choices.push((
                        1100 - d.min(1000),
                        0,
                        Intent {
                            behavior: Behavior::Flee,
                            x: (2 * o.x - t.x).clamp(0, s.config.size - 1),
                            y: (2 * o.y - t.y).clamp(0, s.config.size - 1),
                            target: None,
                        },
                    ));
                }
                let hunger = (p.energy_capacity - o.energy) * 1000 / p.energy_capacity;
                if let Some((_, _, j)) = best_prey.filter(|_| o.energy < p.energy_capacity / 2) {
                    let t = &s.organisms[j];
                    choices.push((
                        hunger + 300 + p.aggression / 4,
                        1,
                        Intent {
                            behavior: Behavior::Hunt,
                            x: t.x,
                            y: t.y,
                            target: Some(j),
                        },
                    ));
                }
                if o.energy >= p.reproduction_threshold
                    && s.tick - o.birth_tick >= 60 + p.morphology.complexity as u64 / 4
                    && s.tick - o.last_mating >= 80 + p.morphology.complexity as u64 / 4
                {
                    if let Some((_, _, j)) = best_mate {
                        let t = &s.organisms[j];
                        choices.push((
                            600 + p.social_affinity / 5,
                            2,
                            Intent {
                                behavior: Behavior::SeekMate,
                                x: t.x,
                                y: t.y,
                                target: Some(j),
                            },
                        ));
                    }
                }
                let cx = o.x / CELL_SIZE;
                let cy = o.y / CELL_SIZE;
                let radius = (p.vision / CELL_SIZE).max(1);
                let mut food = None;
                for y in (cy - radius).max(0)..=(cy + radius).min(side - 1) {
                    for x in (cx - radius).max(0)..=(cx + radius).min(side - 1) {
                        let idx = (y * side + x) as usize;
                        let fx = x * CELL_SIZE + CELL_SIZE / 2;
                        let fy = y * CELL_SIZE + CELL_SIZE / 2;
                        let d = distance_squared(o.x, o.y, fx, fy);
                        let c = &s.environment.cells[idx];
                        let (_, supply, efficiency) = c.preferred_feeding(o);
                        let score = supply * efficiency / 25 - d / 16;
                        let candidate = (score, std::cmp::Reverse(idx), fx, fy);
                        if food.is_none_or(|v| candidate > v) {
                            food = Some(candidate);
                        }
                    }
                }
                if let Some((_, _, x, y)) = food {
                    choices.push((
                        hunger + 100,
                        3,
                        Intent {
                            behavior: Behavior::SeekFood,
                            x,
                            y,
                            target: None,
                        },
                    ));
                }
                // Counter-derived exploration avoids consuming shared RNG in iteration-sensitive queries.
                let direction = ((o
                    .id
                    .0
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(s.tick / 20))
                    % 8) as usize;
                let dirs = [
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                ];
                choices.push((
                    50,
                    4,
                    Intent {
                        behavior: Behavior::Explore,
                        x: o.x + dirs[direction].0 * 20,
                        y: o.y + dirs[direction].1 * 20,
                        target: None,
                    },
                ));
                choices
                    .sort_by_key(|(utility, priority, _)| (std::cmp::Reverse(*utility), *priority));
                choices[0].2
            })
            .collect();
        #[cfg(feature = "profile")]
        crate::profile::mark("perception", &mut profile_stamp);
        // Commit movement; all evaluated intents already exist.
        for (o, intent) in s.organisms.iter_mut().zip(&intents) {
            let dx = intent.x - o.x;
            let dy = intent.y - o.y;
            let norm = dx.abs().max(dy.abs()).max(1);
            let terrain = &s.environment.cells[(o.y / CELL_SIZE * side + o.x / CELL_SIZE) as usize];
            let speed = o
                .phenotype
                .morphology
                .terrain_speed(o.phenotype.speed, terrain.elevation);
            #[cfg(feature = "viability")]
            crate::viability::movement(o, speed, intent.behavior == Behavior::Hunt);
            o.dx = dx * speed / norm;
            o.dy = dy * speed / norm;
            o.x = (o.x + o.dx).clamp(0, s.config.size - 1);
            o.y = (o.y + o.dy).clamp(0, s.config.size - 1);
            o.behavior = intent.behavior;
        }
        #[cfg(feature = "profile")]
        crate::profile::mark("movement", &mut profile_stamp);
        // Shared food is allocated in stable organism-ID order.
        for o in &mut s.organisms {
            if o.behavior == Behavior::Hunt {
                continue;
            }
            let cell =
                &mut s.environment.cells[(o.y / CELL_SIZE * side + o.x / CELL_SIZE) as usize];
            let (channel, available, efficiency) = cell.preferred_feeding(o);
            if efficiency <= 0 {
                #[cfg(feature = "viability")]
                crate::viability::food(o, available, efficiency, 25, 0, 0);
                continue;
            }
            let amount = available
                .min(25)
                .min((o.phenotype.energy_capacity - o.energy) * 100 / efficiency.max(1));
            #[cfg(feature = "viability")]
            crate::viability::food(
                o,
                available,
                efficiency,
                25,
                amount,
                (amount * efficiency / 100).min(o.phenotype.energy_capacity - o.energy),
            );
            if channel == 0 {
                cell.food -= amount;
            } else {
                cell.hard_food -= amount;
            }
            #[cfg(feature = "viability")]
            crate::ecological_diagnostics::feeding(
                o,
                cell,
                channel,
                amount,
                (amount * efficiency / 100).min(o.phenotype.energy_capacity - o.energy),
                0,
                cell.hardness(channel),
            );
            s.species
                .get_mut(&o.species_id)
                .expect("registered species")
                .feeding_observations[0] += amount as u64;
            o.energy = (o.energy + amount * efficiency / 100).min(o.phenotype.energy_capacity);
        }
        #[cfg(feature = "profile")]
        crate::profile::mark("feeding", &mut profile_stamp);
        let mut deaths: BTreeMap<usize, DeathCause> = BTreeMap::new();
        for (i, intent) in intents.iter().enumerate() {
            if intent.behavior != Behavior::Hunt || deaths.contains_key(&i) {
                continue;
            }
            if let Some(j) = intent.target {
                if deaths.contains_key(&j) {
                    continue;
                }
                let predator = &s.organisms[i];
                let prey = &s.organisms[j];
                if distance_squared(predator.x, predator.y, prey.x, prey.y) <= 64
                    && s.tick - predator.last_attack >= 10
                {
                    if !predator
                        .phenotype
                        .morphology
                        .can_attack(&prey.phenotype.morphology)
                    {
                        continue;
                    }
                    let gain = prey.energy / 2 + prey.phenotype.morphology.mass / 4;
                    let damage = predator.phenotype.morphology.damage(
                        &prey.phenotype.morphology,
                        predator.phenotype.aggression,
                        predator.phenotype.speed,
                        prey.phenotype.speed,
                    );
                    let attack_cost = 20 + predator.phenotype.morphology.mass / 40;
                    let attacker_species = predator.species_id;
                    let prey_species = prey.species_id;
                    let prey_mass = prey.phenotype.morphology.mass as u64;
                    #[cfg(feature = "viability")]
                    crate::viability::attack(predator, predator.energy.min(attack_cost));
                    s.organisms[i].energy = (s.organisms[i].energy - attack_cost).max(0);
                    s.organisms[i].last_attack = s.tick;
                    s.organisms[j].health -= damage;
                    s.species
                        .get_mut(&prey_species)
                        .expect("registered prey")
                        .feeding_observations[3] += 1;
                    if s.organisms[j].health <= 0 {
                        #[cfg(feature = "viability")]
                        crate::ecological_diagnostics::feeding(
                            &s.organisms[i],
                            &s.environment.cells[(s.organisms[i].y / CELL_SIZE * side
                                + s.organisms[i].x / CELL_SIZE)
                                as usize],
                            2,
                            0,
                            gain.min(
                                s.organisms[i].phenotype.energy_capacity - s.organisms[i].energy,
                            ),
                            prey_mass as i32,
                            0,
                        );
                        deaths.insert(j, DeathCause::Predation);
                        #[cfg(feature = "viability")]
                        crate::viability::kill(
                            &s.organisms[i],
                            gain.min(
                                s.organisms[i].phenotype.energy_capacity - s.organisms[i].energy,
                            ),
                        );
                        s.organisms[i].energy = (s.organisms[i].energy + gain)
                            .min(s.organisms[i].phenotype.energy_capacity);
                        s.counters.predations += 1;
                        let feeding = &mut s
                            .species
                            .get_mut(&attacker_species)
                            .expect("registered predator")
                            .feeding_observations;
                        feeding[1] += prey_mass;
                        feeding[2] += 1;
                    }
                }
            }
        }
        #[cfg(feature = "profile")]
        crate::profile::mark("predation", &mut profile_stamp);
        for (i, o) in s.organisms.iter_mut().enumerate() {
            if deaths.contains_key(&i) {
                continue;
            }
            let p = &o.phenotype;
            let cell = &s.environment.cells[(o.y / CELL_SIZE * side + o.x / CELL_SIZE) as usize];
            let excess = (s.environment.temperature + cell.temperature_offset
                - p.temperature_optimum)
                .abs()
                .saturating_sub(p.temperature_tolerance)
                .max(0);
            let cost = p.metabolism
                + (o.dx.abs() + o.dy.abs()) * p.morphology.movement_cost / 500
                + excess / (100 + p.morphology.mass / 50);
            #[cfg(feature = "viability")]
            crate::viability::metabolism(
                o,
                [
                    p.metabolism - p.morphology.maintenance_cost,
                    p.morphology.maintenance_cost,
                    (o.dx.abs() + o.dy.abs()) * p.morphology.movement_cost / 500,
                    excess / (100 + p.morphology.mass / 50),
                ],
                o.energy.min(cost),
                o.energy <= cost,
            );
            o.energy = (o.energy - cost).max(0);
            if o.energy == 0 {
                o.health -= 5;
            } else {
                o.health = (o.health + 1).min(100);
            }
            if excess > 2000 {
                o.health -= 5;
            }
            if o.health <= 0 {
                deaths.insert(
                    i,
                    if excess > 2000 {
                        DeathCause::Environment
                    } else {
                        DeathCause::Starvation
                    },
                );
            } else if s.tick - o.birth_tick >= 900 + (1000 - o.genome.0[3] as u64) {
                deaths.insert(i, DeathCause::Age);
            }
        }
        #[cfg(feature = "profile")]
        crate::profile::mark("metabolism", &mut profile_stamp);
        for (&i, &cause) in &deaths {
            #[cfg(feature = "viability")]
            crate::viability::death(&s.organisms[i], s.tick, cause);
            if cause == DeathCause::Starvation
                && s.organisms[i].phenotype.carnivory > 650
                && s.organisms[i].phenotype.morphology.mouth != crate::morphology::Mouth::Grazer
            {
                s.counters.predator_starvation_deaths += 1;
            }
            if let Some(record) = s.ancestry.get_mut(&s.organisms[i].id) {
                record.death = Some((s.tick, cause));
            }
            s.counters.deaths += 1;
        }
        #[cfg(feature = "profile")]
        crate::profile::mark("death accounting", &mut profile_stamp);
        // Matching uses post-resolution spatial positions, stable IDs, and one mating per tick.
        let mating_spatial = Spatial::new(s.config.size, &s.organisms);
        let mut paired = BTreeSet::new();
        let mut births = Vec::new();
        for i in 0..s.organisms.len() {
            let a = &s.organisms[i];
            if deaths.contains_key(&i) || paired.contains(&i) || !eligible(a, s.tick) {
                continue;
            }
            let mate = mating_spatial
                .nearby(a.x, a.y, 16)
                .filter(|j| *j != i && !deaths.contains_key(j) && !paired.contains(j))
                .filter(|j| {
                    let b = &s.organisms[*j];
                    eligible(b, s.tick)
                        && distance_squared(a.x, a.y, b.x, b.y) <= 16 * 16
                        && a.genome.compatible(&b.genome)
                })
                .min_by_key(|j| {
                    (
                        distance_squared(a.x, a.y, s.organisms[*j].x, s.organisms[*j].y),
                        s.organisms[*j].id,
                    )
                });
            #[cfg(feature = "viability")]
            {
                let mut candidates = [0; 9];
                for j in mating_spatial.nearby(a.x, a.y, 16) {
                    let b = &s.organisms[j];
                    if j == i
                        || deaths.contains_key(&j)
                        || paired.contains(&j)
                        || distance_squared(a.x, a.y, b.x, b.y) > 16 * 16
                    {
                        continue;
                    }
                    candidates[0] += 1;
                    if eligible(b, s.tick) {
                        candidates[1] += 1;
                        candidates[2] += u64::from(a.genome.compatible(&b.genome));
                        candidates[3] += u64::from(a.genome.distance(&b.genome) <= 220);
                        let differences: Vec<_> = a
                            .genome
                            .0
                            .iter()
                            .zip(b.genome.0)
                            .map(|(a, b)| a.abs_diff(b))
                            .collect();
                        candidates[4] += u64::from(differences[..14].iter().all(|d| *d <= 400));
                        candidates[5] += u64::from(differences[14..].iter().all(|d| *d <= 400));
                        candidates[6] += u64::from(differences[14] <= 400);
                        candidates[7] += u64::from(differences[17] <= 400);
                        candidates[8] += u64::from(differences[13] <= 400);
                    }
                }
                crate::viability::mate_search(a, candidates);
            }
            #[cfg(feature = "viability")]
            crate::viability::mating_attempt(a, mate.is_some());
            let Some(j) = mate else {
                continue;
            };
            if s.organisms.len() - deaths.len() + births.len() >= s.config.population_limit {
                break;
            }
            paired.insert(i);
            paired.insert(j);
            let a = s.organisms[i].clone();
            let b = s.organisms[j].clone();
            #[cfg(feature = "viability")]
            crate::viability::mating_attempt(&b, true);
            let count = a.phenotype.offspring_count.min(b.phenotype.offspring_count);
            let a_cost = a.energy / 4 + a.phenotype.morphology.reproduction_cost / 2;
            let b_cost = b.energy / 4 + b.phenotype.morphology.reproduction_cost / 2;
            let budget = (a_cost + b_cost
                - (a.phenotype.morphology.complexity + b.phenotype.morphology.complexity))
                .max(0);
            for _ in 0..count {
                if s.organisms.len() - deaths.len() + births.len() >= s.config.population_limit {
                    break;
                }
                let rate = ((a.phenotype.mutation_rate + b.phenotype.mutation_rate) / 2
                    * s.config.mutation_multiplier
                    / 100)
                    .min(1000);
                let (genome, mutations) = Genome::child(
                    &a.genome,
                    &b.genome,
                    &mut s.rng.reproduction,
                    &mut s.rng.mutation,
                    rate,
                );
                s.counters.mutations += u64::from(mutations);
                let phenotype = genome.phenotype();
                let parent_lineage = if genome.distance(&a.genome) <= genome.distance(&b.genome) {
                    a.lineage_id
                } else {
                    b.lineage_id
                };
                let source = s.lineages[&parent_lineage].clone();
                let lineage_id = if genome.distance(&source.founder) >= 120 {
                    // Join compatible existing local ancestry branches before creating a new candidate.
                    if let Some(id) = s
                        .lineages
                        .values()
                        .filter(|l| {
                            l.parent == Some(parent_lineage)
                                && l.species_id == source.species_id
                                && genome.distance(&l.founder) < 80
                        })
                        .map(|l| l.id)
                        .next()
                    {
                        id
                    } else {
                        let id = LineageId(s.next_lineage);
                        s.next_lineage += 1;
                        s.lineages.insert(
                            id,
                            Lineage {
                                id,
                                parent: Some(parent_lineage),
                                origin_tick: s.tick,
                                founder: genome.clone(),
                                species_id: source.species_id,
                                candidate_since: None,
                                births: 0,
                                cross_births: 0,
                            },
                        );
                        id
                    }
                } else {
                    parent_lineage
                };
                let line = s.lineages.get_mut(&lineage_id).expect("lineage created");
                line.births += 1;
                if a.lineage_id != b.lineage_id {
                    line.cross_births += 1;
                }
                let id = OrganismId(s.next_organism);
                s.next_organism += 1;
                let parents = Some([a.id, b.id]);
                s.ancestry.insert(
                    id,
                    Ancestry {
                        birth_tick: s.tick,
                        parents,
                        death: None,
                    },
                );
                births.push(Organism {
                    id,
                    genome_id: GenomeId(id.0),
                    birth_tick: s.tick,
                    generation: a.generation.max(b.generation) + 1,
                    parents,
                    genome,
                    phenotype: phenotype.clone(),
                    x: a.x,
                    y: a.y,
                    dx: 0,
                    dy: 0,
                    energy: (budget / count as i32).min(phenotype.energy_capacity),
                    health: 100,
                    species_id: line.species_id,
                    lineage_id,
                    offspring: 0,
                    last_mating: s.tick,
                    last_attack: s.tick,
                    behavior: Behavior::Rest,
                });
                s.counters.births += 1;
                #[cfg(feature = "viability")]
                crate::viability::birth(births.last().expect("just born"));
                #[cfg(feature = "viability")]
                {
                    let child = births.last().expect("just born");
                    crate::ecological_diagnostics::birth(
                        child,
                        &s.environment.cells
                            [(child.y / CELL_SIZE * side + child.x / CELL_SIZE) as usize],
                        &a,
                        &b,
                    );
                    crate::viability::parent_child(&a, child);
                    crate::viability::parent_child(&b, child);
                }
                s.organisms[i].offspring += 1;
                s.organisms[j].offspring += 1;
            }
            #[cfg(feature = "viability")]
            {
                if s.organisms[i].offspring > a.offspring {
                    crate::ecological_diagnostics::mating(
                        &a,
                        &b,
                        &s.environment.cells[(a.y / CELL_SIZE * side + a.x / CELL_SIZE) as usize],
                        &s.environment.cells[(b.y / CELL_SIZE * side + b.x / CELL_SIZE) as usize],
                    );
                }
                crate::viability::reproduction(
                    &a,
                    a.energy.min(a_cost),
                    s.organisms[i].offspring - a.offspring,
                );
                crate::viability::reproduction(
                    &b,
                    b.energy.min(b_cost),
                    s.organisms[j].offspring - b.offspring,
                );
            }
            s.organisms[i].energy = (a.energy - a_cost).max(0);
            s.organisms[j].energy = (b.energy - b_cost).max(0);
            s.organisms[i].last_mating = s.tick;
            s.organisms[j].last_mating = s.tick;
        }
        #[cfg(feature = "profile")]
        crate::profile::mark("mating", &mut profile_stamp);
        let mut index = 0;
        s.organisms.retain(|_| {
            let keep = !deaths.contains_key(&index);
            index += 1;
            keep
        });
        s.organisms.extend(births);
        let predators = s
            .organisms
            .iter()
            .filter(|o| {
                o.phenotype.carnivory > 650
                    && o.phenotype.morphology.mouth != crate::morphology::Mouth::Grazer
            })
            .count();
        if predators > 0 && predators < s.organisms.len() {
            s.counters.coexistence_ticks += 1;
        }
        s.counters.peak_population = s.counters.peak_population.max(s.organisms.len());
        self.account();
        if self.state.organisms.len() == self.state.config.population_limit {
            if self.state.counters.safety_ceiling_ticks == 0 {
                self.event(
                    None,
                    HistoryKind::SafetyPopulationCeiling {
                        population: self.state.organisms.len(),
                    },
                );
            }
            self.state.counters.safety_ceiling_ticks += 1;
        }
        #[cfg(feature = "profile")]
        crate::profile::mark("population accounting", &mut profile_stamp);
        if self.state.tick.is_multiple_of(TELEMETRY_INTERVAL) {
            self.detect_species();
            self.sample();
            #[cfg(feature = "viability")]
            crate::ecological_diagnostics::sample(self);
        }
        #[cfg(feature = "profile")]
        crate::profile::mark("species and telemetry", &mut profile_stamp);
        if self.state.tick.is_multiple_of(1000) {
            self.event(
                None,
                HistoryKind::PopulationMilestone {
                    population: self.state.organisms.len(),
                },
            );
        }
    }

    fn account(&mut self) {
        for species in self.state.species.values_mut() {
            species.population = 0;
        }
        for o in &self.state.organisms {
            self.state
                .species
                .get_mut(&o.species_id)
                .expect("registered species")
                .population += 1;
        }
        let extinct: Vec<_> = self
            .state
            .species
            .values()
            .filter(|sp| sp.population == 0 && sp.extinct_tick.is_none())
            .map(|sp| sp.id)
            .collect();
        for id in extinct {
            self.state
                .species
                .get_mut(&id)
                .expect("species exists")
                .extinct_tick = Some(self.state.tick);
            self.event(Some(id), HistoryKind::Extinction);
        }
    }

    fn detect_species(&mut self) {
        let mut populations: BTreeMap<LineageId, Vec<usize>> = BTreeMap::new();
        for (i, o) in self.state.organisms.iter().enumerate() {
            populations.entry(o.lineage_id).or_default().push(i);
        }
        let ids: Vec<_> = self.state.lineages.keys().copied().collect();
        for id in ids {
            let line = self.state.lineages[&id].clone();
            let members = populations.get(&id).cloned().unwrap_or_default();
            let ancestor = self.state.species[&line.species_id].clone();
            let mean = mean_genome(members.iter().map(|i| &self.state.organisms[*i].genome));
            let distance = mean.distance(&ancestor.founder);
            let isolated = line.births >= 8 && line.cross_births * 100 <= line.births * 20;
            let valid = line.parent.is_some() && members.len() >= 8 && distance >= 100 && isolated;
            if !valid {
                self.state
                    .lineages
                    .get_mut(&id)
                    .expect("lineage")
                    .candidate_since = None;
                continue;
            }
            if line.candidate_since.is_none() {
                self.state
                    .lineages
                    .get_mut(&id)
                    .expect("lineage")
                    .candidate_since = Some(self.state.tick);
                self.event(
                    Some(line.species_id),
                    HistoryKind::SpeciesCandidate { lineage: id },
                );
                continue;
            }
            if self.state.tick - line.candidate_since.unwrap_or(self.state.tick) < 200 {
                continue;
            }
            let spid = SpeciesId(self.state.next_species);
            self.state.next_species += 1;
            self.state.species.insert(
                spid,
                Species {
                    representative_genome: self.state.organisms[members[0]].genome.clone(),
                    representative_morphology: self.state.organisms[members[0]]
                        .phenotype
                        .morphology
                        .clone(),
                    morphology_summary: [0; 9],
                    origin_environment: [
                        self.state.environment.temperature,
                        self.state.environment.regeneration,
                        self.state.environment.cells[(self.state.organisms[members[0]].y
                            / CELL_SIZE
                            * (self.state.config.size / CELL_SIZE)
                            + self.state.organisms[members[0]].x / CELL_SIZE)
                            as usize]
                            .elevation,
                    ],
                    niche: [0; 6],
                    feeding_observations: [0; 4],
                    innovation_streaks: [0; 5],
                    innovations: [false; 5],
                    id: spid,
                    name: scientific_name(self.state.config.seed, spid.0),
                    ancestor: Some(line.species_id),
                    origin_tick: self.state.tick,
                    origin_generation: members
                        .iter()
                        .map(|i| self.state.organisms[*i].generation)
                        .min()
                        .unwrap_or(0),
                    extinct_tick: None,
                    founder: mean,
                    founder_population: members.len(),
                    genetic_distance: distance,
                    population: members.len(),
                },
            );
            let branch = self.state.lineages.get_mut(&id).expect("lineage");
            branch.species_id = spid;
            branch.candidate_since = None;
            branch.births = 0;
            branch.cross_births = 0;
            for i in members {
                self.state.organisms[i].species_id = spid;
            }
            self.event(
                Some(spid),
                HistoryKind::Speciation {
                    ancestor: line.species_id,
                    distance,
                    founders: populations[&id].len(),
                },
            );
        }
        self.account();
    }

    fn sample(&mut self) {
        let mut groups: BTreeMap<SpeciesId, Vec<&Organism>> = BTreeMap::new();
        for o in &self.state.organisms {
            groups.entry(o.species_id).or_default().push(o);
        }
        let mut species = groups
            .iter()
            .map(|(id, group)| {
                let mut sums = [0i64; 6];
                let mut bounds = [self.state.config.size, self.state.config.size, 0, 0];
                for o in group {
                    for (i, value) in [
                        o.phenotype.body_size,
                        o.phenotype.speed,
                        o.phenotype.vision,
                        o.phenotype.metabolism,
                        o.phenotype.temperature_tolerance,
                        o.phenotype.aggression,
                    ]
                    .iter()
                    .enumerate()
                    {
                        sums[i] += i64::from(*value);
                    }
                    bounds[0] = bounds[0].min(o.x);
                    bounds[1] = bounds[1].min(o.y);
                    bounds[2] = bounds[2].max(o.x);
                    bounds[3] = bounds[3].max(o.y);
                }
                SpeciesTelemetry {
                    morphology: morphology_mean(group),
                    niche: niche_mean(group, &self.state.environment, self.state.config.size),
                    species: *id,
                    population: group.len(),
                    means: sums.map(|v| (v / group.len() as i64) as i32),
                    bounds,
                }
            })
            .collect::<Vec<_>>();
        let mut innovations = Vec::new();
        let species_count = groups.len();
        drop(groups);
        for sample in &mut species {
            let sp = self
                .state
                .species
                .get_mut(&sample.species)
                .expect("registered species");
            sp.morphology_summary = sample.morphology;
            sp.niche = sample.niche;
            let f = sp.feeding_observations;
            sp.niche[0] = (f[0] * 1000 / (f[0] + f[1]).max(1)) as i32;
            sp.niche[1] = (f[1] / f[2].max(1) / 3).min(1000) as i32;
            sp.niche[2] = (f[3] * 1000 / (self.state.tick.max(1) * sample.population as u64))
                .min(1000) as i32;
            sample.niche = sp.niche;
            let m = sample.morphology;
            let crossings = [
                m[0] >= 2000,
                m[2] >= 300,
                m[8] >= 500 && f[2] > 0,
                m[1] >= 1400,
                m[5] >= 700,
            ];
            for (i, crossing) in crossings.into_iter().enumerate() {
                sp.innovation_streaks[i] = if crossing && sample.population >= 8 {
                    sp.innovation_streaks[i].saturating_add(1)
                } else {
                    0
                };
                if sp.innovation_streaks[i] >= 10 && !sp.innovations[i] {
                    sp.innovations[i] = true;
                    innovations.push((sp.id, i));
                }
            }
        }
        for (id, index) in innovations {
            self.event(Some(id), HistoryKind::MorphologicalInnovation { index });
        }
        let average = mean_genome(self.state.organisms.iter().map(|o| &o.genome));
        let diversity = if self.state.organisms.is_empty() {
            0
        } else {
            self.state
                .organisms
                .iter()
                .map(|o| u64::from(o.genome.distance(&average)))
                .sum::<u64>()
                / self.state.organisms.len() as u64
        } as u32;
        self.state.telemetry.push(Telemetry {
            tick: self.state.tick,
            population: self.state.organisms.len(),
            species_count,
            food: self
                .state
                .environment
                .cells
                .iter()
                .map(|c| i64::from(c.total_food()))
                .sum(),
            temperature: self.state.environment.temperature,
            births: self.state.counters.births,
            deaths: self.state.counters.deaths,
            mutations: self.state.counters.mutations,
            diversity,
            species,
        });
    }

    pub fn hash(&self) -> String {
        let bytes = bincode::serialize(&(SIMULATION_VERSION, crate::rng::RNG_VERSION, &self.state))
            .expect("state serializable");
        blake3::hash(&bytes).to_hex().to_string()
    }

    pub fn validate(&self) -> Result<(), String> {
        let s = &self.state;
        s.config.validate()?;
        if s.tick == u64::MAX
            || s.next_organism == u64::MAX
            || s.next_species == u64::MAX
            || s.next_lineage == u64::MAX
        {
            return Err("Exhausted simulation counters".into());
        }
        if s.environment.cells.len() != (s.config.size / CELL_SIZE).pow(2) as usize {
            return Err("Invalid resource grid".into());
        }
        if !(-2000..=6000).contains(&s.environment.temperature)
            || !(0..=100).contains(&s.environment.regeneration)
        {
            return Err("Invalid environment".into());
        }
        if s.environment.cells.iter().any(|c| {
            !(0..=1000).contains(&c.food)
                || !(0..=300).contains(&c.hard_food)
                || c.resource_remainders
                    .iter()
                    .zip(crate::resources::DENOMINATORS)
                    .any(|(n, d)| !(0..d).contains(n))
                || !(60..=140).contains(&c.fertility)
                || !(0..=1000).contains(&c.elevation)
                || !(0..=1000).contains(&c.moisture)
                || !(-600..=600).contains(&c.temperature_offset)
        }) {
            return Err("Invalid resources".into());
        }
        if s.organisms.len() > s.config.population_limit {
            return Err("Population limit exceeded".into());
        }
        let mut previous = 0;
        let mut counts = BTreeMap::new();
        for o in &s.organisms {
            if o.id.0 <= previous || o.id.0 >= s.next_organism || o.genome_id.0 != o.id.0 {
                return Err("Invalid/duplicate organism IDs".into());
            }
            previous = o.id.0;
            if !(0..s.config.size).contains(&o.x)
                || !(0..s.config.size).contains(&o.y)
                || o.energy < 0
                || o.energy > o.phenotype.energy_capacity
                || o.health <= 0
                || o.health > 100
            {
                return Err("Invalid living state".into());
            }
            if o.genome.0.iter().any(|g| *g > 1000) || o.phenotype != o.genome.phenotype() {
                return Err("Invalid genome/phenotype".into());
            }
            if !s.species.contains_key(&o.species_id) || !s.lineages.contains_key(&o.lineage_id) {
                return Err("Missing species/lineage".into());
            }
            let record = s.ancestry.get(&o.id).ok_or("Missing ancestry")?;
            if record.death.is_some()
                || record.birth_tick != o.birth_tick
                || record.parents != o.parents
                || o.birth_tick > s.tick
                || o.last_mating > s.tick
                || o.last_attack > s.tick
            {
                return Err("Invalid ancestry".into());
            }
            *counts.entry(o.species_id).or_insert(0usize) += 1;
        }
        for (id, record) in &s.ancestry {
            if id.0 >= s.next_organism || record.birth_tick > s.tick {
                return Err("Invalid historical ID".into());
            }
            if record
                .death
                .is_some_and(|(tick, _)| tick < record.birth_tick || tick > s.tick)
            {
                return Err("Invalid death chronology".into());
            }
            if let Some(parents) = record.parents {
                for parent in parents {
                    let p = s.ancestry.get(&parent).ok_or("Missing historical parent")?;
                    if p.birth_tick >= record.birth_tick || parent >= *id {
                        return Err("Invalid parent chronology".into());
                    }
                }
            }
        }
        for (id, sp) in &s.species {
            if sp.id != *id
                || id.0 == 0
                || id.0 >= s.next_species
                || sp.origin_tick > s.tick
                || sp
                    .extinct_tick
                    .is_some_and(|tick| tick < sp.origin_tick || tick > s.tick)
                || sp.founder.0.iter().any(|g| *g > 1000)
                || sp.representative_genome.0.iter().any(|g| *g > 1000)
                || sp.representative_morphology != sp.representative_genome.phenotype().morphology
                || sp
                    .morphology_summary
                    .iter()
                    .any(|v| !(0..=5000).contains(v))
                || sp.niche.iter().any(|v| !(0..=1000).contains(v))
                || sp.population != *counts.get(id).unwrap_or(&0)
                || sp.population > 0 && sp.extinct_tick.is_some()
                || sp
                    .ancestor
                    .is_some_and(|a| a >= *id || !s.species.contains_key(&a))
            {
                return Err("Invalid species accounting".into());
            }
        }
        for (id, line) in &s.lineages {
            if line.id != *id
                || id.0 == 0
                || id.0 >= s.next_lineage
                || line.origin_tick > s.tick
                || line
                    .candidate_since
                    .is_some_and(|tick| tick < line.origin_tick || tick > s.tick)
                || line.cross_births > line.births
                || line.founder.0.iter().any(|g| *g > 1000)
                || !s.species.contains_key(&line.species_id)
                || line
                    .parent
                    .is_some_and(|p| p >= *id || !s.lineages.contains_key(&p))
            {
                return Err("Invalid lineage registry".into());
            }
        }
        for command in &s.commands {
            command.validate()?;
            if command.tick > s.tick {
                return Err("Future command in history".into());
            }
        }
        for (index, event) in s.history.iter().enumerate() {
            if event.id.0 != index as u64 + 1
                || event.tick > s.tick
                || event.species.is_some_and(|id| !s.species.contains_key(&id))
            {
                return Err("Invalid historical event registry".into());
            }
            if matches!(event.kind, HistoryKind::MorphologicalInnovation { index } if index>=5) {
                return Err("Invalid morphological innovation".into());
            }
        }
        if s.commands.windows(2).any(|w| w[0].tick > w[1].tick) {
            return Err("Unordered command history".into());
        }
        Ok(())
    }
}

pub(crate) fn eligible(o: &Organism, tick: u64) -> bool {
    tick - o.birth_tick >= 60 + o.phenotype.morphology.complexity as u64 / 4
        && tick - o.last_mating >= 80 + o.phenotype.morphology.complexity as u64 / 4
        && o.energy >= o.phenotype.reproduction_threshold
}
fn morphology_mean(group: &[&Organism]) -> [i32; 9] {
    let mut sums = [0i64; 9];
    for o in group {
        let m = &o.phenotype.morphology;
        let values = [
            m.segment_count * 1000,
            m.mass,
            m.armor,
            m.bite_capacity,
            m.locomotion_efficiency,
            m.sensory_investment,
            m.complexity,
            o.phenotype.speed,
            if m.mouth == crate::morphology::Mouth::Piercer {
                1000
            } else {
                0
            },
        ];
        for (s, v) in sums.iter_mut().zip(values) {
            *s += i64::from(v);
        }
    }
    sums.map(|v| (v / group.len().max(1) as i64) as i32)
}
fn niche_mean(group: &[&Organism], environment: &Environment, size: i32) -> [i32; 6] {
    let mut sums = [0i64; 6];
    for o in group {
        let cell =
            &environment.cells[(o.y / CELL_SIZE * (size / CELL_SIZE) + o.x / CELL_SIZE) as usize];
        let values = [
            0,
            0,
            0,
            (environment.temperature + cell.temperature_offset + 2600) / 10,
            cell.elevation,
            o.phenotype.speed * 50,
        ];
        for (s, v) in sums.iter_mut().zip(values) {
            *s += i64::from(v);
        }
    }
    sums.map(|v| (v / group.len().max(1) as i64).clamp(0, 1000) as i32)
}
fn mean_genome<'a>(genomes: impl Iterator<Item = &'a Genome>) -> Genome {
    let mut sums = [0u64; LOCI];
    let mut n = 0;
    for genome in genomes {
        n += 1;
        for (sum, g) in sums.iter_mut().zip(genome.0) {
            *sum += u64::from(g);
        }
    }
    Genome(sums.map(|sum| sum.checked_div(n).unwrap_or(0) as u16))
}
pub fn scientific_name(seed: u64, id: u64) -> String {
    let stems = [
        "Protocella",
        "Microvorus",
        "Velocipodus",
        "Photovorus",
        "Silvavita",
        "Thermocella",
    ];
    let suffixes = ["prima", "minor", "ruber", "alba", "borealis", "silva"];
    format!(
        "{} {}-{}",
        stems[((seed ^ id) % stems.len() as u64) as usize],
        suffixes[(id as usize - 1) % suffixes.len()],
        id
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finite_resources_compete_in_stable_id_order() {
        let mut w = World::new(Config {
            starting_population: 2,
            ..Config::default()
        })
        .unwrap();
        w.state.environment.regeneration = 0;
        for cell in &mut w.state.environment.cells {
            cell.food = 0;
            cell.hard_food = 0;
            cell.temperature_offset = 0;
        }
        for o in &mut w.state.organisms {
            o.x = 104;
            o.y = 104;
            o.genome.0[13] = 0;
            o.genome.0[17] = 0;
            o.phenotype = o.genome.phenotype();
            o.energy = 1000;
        }
        let cell = (104 / CELL_SIZE * (w.state.config.size / CELL_SIZE) + 104 / CELL_SIZE) as usize;
        w.state.environment.cells[cell].food = 10;
        w.step();
        assert_eq!(w.state.environment.cells[cell].food, 0);
        assert_eq!(w.state.species[&SpeciesId(1)].feeding_observations[0], 10);
        assert!(w.state.organisms[0].energy > w.state.organisms[1].energy);
        w.validate().unwrap();
    }
    #[test]
    fn habitat_bounds_hold_for_full_seed_range() {
        for seed in [0, 1, 42, u64::MAX] {
            let w = World::new(Config {
                seed,
                starting_population: 1,
                ..Config::default()
            })
            .unwrap();
            w.validate().unwrap();
            let c = &w.state.environment.cells;
            assert!(c.iter().any(|a| a.elevation < 200) && c.iter().any(|a| a.elevation > 700));
            assert!(
                c.iter().any(|a| a.temperature_offset < 0)
                    && c.iter().any(|a| a.temperature_offset > 0)
            );
        }
    }
    #[test]
    fn innovations_require_persistence_and_preserve_real_ancestor_snapshot() {
        let mut w = World::new(Config {
            starting_population: 8,
            ..Config::default()
        })
        .unwrap();
        let founder = w.state.species[&SpeciesId(1)]
            .representative_morphology
            .clone();
        assert_eq!(founder.segment_count, 1);
        for o in &mut w.state.organisms {
            o.genome.0[14] = 500;
            o.phenotype = o.genome.phenotype();
        }
        for i in 1..=9 {
            w.state.tick = i * 100;
            w.sample();
        }
        assert!(!w.state.species[&SpeciesId(1)].innovations[0]);
        w.state.tick = 1000;
        w.sample();
        assert!(w.state.species[&SpeciesId(1)].innovations[0]);
        w.state.tick = 1100;
        w.sample();
        assert_eq!(
            w.state
                .history
                .iter()
                .filter(|e| matches!(e.kind, HistoryKind::MorphologicalInnovation { index: 0 }))
                .count(),
            1
        );
        assert_eq!(
            w.state.species[&SpeciesId(1)].representative_morphology,
            founder
        );
        let restored =
            crate::persistence::decode(&crate::persistence::encode(&w).unwrap()).unwrap();
        assert_eq!(restored.hash(), w.hash());
    }
    #[test]
    fn safety_ceiling_diagnostic_is_explicit_and_not_history_spam() {
        let mut w = World::new(Config {
            starting_population: 1,
            population_limit: 1,
            ..Config::default()
        })
        .unwrap();
        w.advance(10);
        assert_eq!(w.state.counters.safety_ceiling_ticks, 10);
        assert_eq!(
            w.state
                .history
                .iter()
                .filter(|e| matches!(e.kind, HistoryKind::SafetyPopulationCeiling { .. }))
                .count(),
            1
        );
        w.validate().unwrap();
    }
    #[test]
    fn extinct_species_keeps_last_morphology_and_observations() {
        let mut w = World::new(Config {
            starting_population: 1,
            ..Config::default()
        })
        .unwrap();
        let morphology = w.state.species[&SpeciesId(1)].morphology_summary;
        w.command(Command {
            tick: 0,
            temperature: 6000,
            regeneration: 0,
        })
        .unwrap();
        w.advance(2000);
        assert!(w.state.organisms.is_empty());
        assert_eq!(
            w.state.species[&SpeciesId(1)].morphology_summary,
            morphology
        );
        assert!(w.state.species[&SpeciesId(1)].extinct_tick.is_some());
        w.validate().unwrap();
    }
    #[test]
    fn same_seed_and_chunking() {
        let mut a = World::new(Config::default()).unwrap();
        let mut b = a.clone();
        a.advance(1000);
        for _ in 0..10 {
            b.advance(100);
        }
        assert_eq!(a.hash(), b.hash());
        a.validate().unwrap();
        assert!(a.state.counters.births > 0);
        assert!(a.state.counters.deaths > 0);
        assert!(a.state.counters.mutations > 0);
        assert!(a.state.counters.predations > 0);
    }
    #[test]
    fn food_regeneration_and_consumption() {
        let mut w = World::new(Config {
            starting_population: 1,
            ..Config::default()
        })
        .unwrap();
        for c in &mut w.state.environment.cells {
            c.food = 0;
            c.hard_food = 0;
        }
        w.state.environment.regeneration = 10;
        w.step();
        assert!(w.state.environment.cells.iter().any(|c| c.food > 0));
        w.state.environment.regeneration = 0;
        let total: i32 = w.state.environment.cells.iter().map(|c| c.food).sum();
        w.step();
        assert!(
            w.state
                .environment
                .cells
                .iter()
                .map(|c| c.food)
                .sum::<i32>()
                <= total
        );
    }
    #[test]
    fn command_validation_is_atomic() {
        let mut w = World::new(Config::default()).unwrap();
        let hash = w.hash();
        assert!(w
            .command(Command {
                tick: 0,
                temperature: 100000,
                regeneration: 12
            })
            .is_err());
        assert_eq!(w.hash(), hash);
    }
    #[test]
    fn environment_causes_death_and_extinction() {
        let mut w = World::new(Config::default()).unwrap();
        w.command(Command {
            tick: 0,
            temperature: 6000,
            regeneration: 0,
        })
        .unwrap();
        w.advance(2000);
        w.validate().unwrap();
        assert!(w.state.organisms.is_empty());
        assert!(w.state.species[&SpeciesId(1)].extinct_tick.is_some());
        assert!(w
            .state
            .ancestry
            .values()
            .any(|r| r.death.is_some_and(|(_, c)| c == DeathCause::Environment)));
    }
    #[test]
    fn naming_unique_and_stable() {
        let names: BTreeSet<_> = (1..10000).map(|id| scientific_name(42, id)).collect();
        assert_eq!(names.len(), 9999);
        assert_eq!(scientific_name(42, 10), scientific_name(42, 10));
    }
    #[test]
    fn species_requires_population_isolation_and_persistence() {
        let mut w = World::new(Config {
            starting_population: 20,
            ..Config::default()
        })
        .unwrap();
        let genome = Genome([800; LOCI]);
        let id = LineageId(2);
        w.state.lineages.insert(
            id,
            Lineage {
                id,
                parent: Some(LineageId(1)),
                origin_tick: 0,
                founder: genome.clone(),
                species_id: SpeciesId(1),
                candidate_since: None,
                births: 10,
                cross_births: 0,
            },
        );
        w.state.next_lineage = 3;
        for o in w.state.organisms.iter_mut().take(10) {
            o.lineage_id = id;
            o.genome = genome.clone();
            o.phenotype = genome.phenotype();
            // Synthetic classifier fixture: cohorts differ in generation as well as genome.
            o.generation = 7 + o.id.0;
        }
        w.detect_species();
        assert_eq!(w.state.species.len(), 1);
        w.state.tick = 100;
        w.detect_species();
        assert_eq!(w.state.species.len(), 1);
        w.state.tick = 200;
        w.detect_species();
        assert_eq!(w.state.species.len(), 2);
        assert_eq!(w.state.species[&SpeciesId(2)].ancestor, Some(SpeciesId(1)));
        assert_eq!(w.state.species[&SpeciesId(2)].origin_generation, 8);
        let loaded = crate::persistence::decode(&crate::persistence::encode(&w).unwrap()).unwrap();
        assert_eq!(loaded.state.species[&SpeciesId(2)].origin_generation, 8);
        assert!(matches!(
            w.state.history.last().unwrap().kind,
            HistoryKind::Speciation { .. }
        ));
        w.validate().unwrap();
    }
    #[test]
    fn multi_seed_invariants() {
        for seed in 0..10 {
            let mut w = World::new(Config {
                seed,
                starting_population: 50,
                ..Config::default()
            })
            .unwrap();
            for _ in 0..10 {
                w.advance(100);
                w.validate().unwrap();
            }
        }
    }

    #[test]
    fn predation_records_death_and_dead_prey_cannot_reproduce() {
        let mut w = World::new(Config {
            starting_population: 2,
            ..Config::default()
        })
        .unwrap();
        for o in &mut w.state.organisms {
            o.x = 100;
            o.y = 100;
            o.genome = Genome([500; LOCI]);
            o.phenotype = o.genome.phenotype();
            o.energy = 1000;
        }
        w.state.organisms[0].genome.0[13] = 900;
        w.state.organisms[0].phenotype = w.state.organisms[0].genome.phenotype();
        w.state.organisms[0].energy = 10;
        w.state.organisms[1].genome.0[13] = 0;
        w.state.organisms[1].phenotype = w.state.organisms[1].genome.phenotype();
        w.state.organisms[1].health = 20;
        w.state.tick = 10;
        w.step();
        assert_eq!(w.state.counters.predations, 1);
        assert_eq!(w.state.organisms.len(), 1);
        assert_eq!(w.state.organisms[0].id, OrganismId(1));
        assert!(w.state.organisms[0].energy > 10);
        assert_eq!(
            w.state.ancestry[&OrganismId(2)].death,
            Some((11, DeathCause::Predation))
        );
        w.validate().unwrap();
    }

    #[test]
    fn metabolism_and_thermal_pressure_have_real_costs() {
        let mut normal = World::new(Config {
            starting_population: 1,
            ..Config::default()
        })
        .unwrap();
        normal.state.environment.regeneration = 0;
        for cell in &mut normal.state.environment.cells {
            cell.food = 0;
            cell.hard_food = 0;
            cell.temperature_offset = 0;
        }
        let optimum = normal.state.organisms[0].phenotype.temperature_optimum;
        normal.state.environment.temperature = optimum;
        let energy = normal.state.organisms[0].energy;
        let mut harsh = normal.clone();
        harsh.state.environment.temperature = 6000;
        normal.step();
        harsh.step();
        assert!(normal.state.organisms[0].energy < energy);
        assert!(harsh.state.organisms[0].energy < normal.state.organisms[0].energy);
        assert_eq!(
            normal.state.organisms[0].genome,
            harsh.state.organisms[0].genome
        );
    }

    #[test]
    fn mature_parents_produce_inherited_children_with_valid_ancestry() {
        let mut w = World::new(Config {
            starting_population: 2,
            mutation_multiplier: 0,
            ..Config::default()
        })
        .unwrap();
        w.state.tick = 300;
        for o in &mut w.state.organisms {
            o.x = 100;
            o.y = 100;
            o.genome.0[13] = 200;
            o.phenotype = o.genome.phenotype();
            o.energy = o.phenotype.energy_capacity;
        }
        let parents = w.state.organisms.clone();
        w.step();
        assert!(w.state.organisms.len() > 2);
        for child in w.state.organisms.iter().skip(2) {
            assert_eq!(child.parents, Some([OrganismId(1), OrganismId(2)]));
            assert_eq!(child.generation, 1);
            for i in 0..LOCI {
                assert!(
                    child.genome.0[i] == parents[0].genome.0[i]
                        || child.genome.0[i] == parents[1].genome.0[i]
                );
            }
        }
        assert_eq!(w.state.counters.mutations, 0);
        w.validate().unwrap();
    }

    #[test]
    fn healthy_prey_survives_a_bite_and_attacks_have_cost_and_cooldown() {
        let mut w = World::new(Config {
            starting_population: 2,
            ..Config::default()
        })
        .unwrap();
        w.state.tick = 10;
        for o in &mut w.state.organisms {
            o.x = 100;
            o.y = 100;
            o.genome = Genome([500; LOCI]);
            o.phenotype = o.genome.phenotype();
            o.energy = 500;
        }
        w.state.organisms[0].genome.0[13] = 900;
        w.state.organisms[0].phenotype = w.state.organisms[0].genome.phenotype();
        w.state.organisms[1].genome.0[13] = 0;
        w.state.organisms[1].phenotype = w.state.organisms[1].genome.phenotype();
        w.step();
        assert_eq!(w.state.organisms.len(), 2);
        assert!(w.state.organisms[1].health < 100);
        assert!(w.state.organisms[0].energy < 500);
        assert_eq!(w.state.organisms[0].last_attack, 11);
        w.step();
        assert_eq!(w.state.organisms[0].last_attack, 11);
        w.validate().unwrap();
    }
}
