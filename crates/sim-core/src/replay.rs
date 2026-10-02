use crate::{
    model::{Command, Config, SIMULATION_VERSION},
    rng::RNG_VERSION,
    World,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checkpoint {
    pub tick: u64,
    pub hash: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Replay {
    pub simulation_version: u32,
    pub rng_version: u32,
    pub config: Config,
    pub commands: Vec<Command>,
    pub final_tick: u64,
    pub checkpoints: Vec<Checkpoint>,
}
impl Replay {
    pub fn from_world(world: &World) -> Self {
        Self {
            simulation_version: SIMULATION_VERSION,
            rng_version: RNG_VERSION,
            config: world.state.config.clone(),
            commands: world.state.commands.clone(),
            final_tick: world.state.tick,
            checkpoints: vec![Checkpoint {
                tick: world.state.tick,
                hash: world.hash(),
            }],
        }
    }
    pub fn verify(&self) -> Result<World, String> {
        if self.simulation_version != SIMULATION_VERSION || self.rng_version != RNG_VERSION {
            return Err("Incompatible replay version".into());
        }
        if self.final_tick > 10_000_000 {
            return Err("Replay tick limit exceeded".into());
        }
        if self.commands.windows(2).any(|c| c[0].tick > c[1].tick)
            || self.commands.iter().any(|c| c.tick > self.final_tick)
        {
            return Err("Invalid replay command ordering".into());
        }
        if self.checkpoints.windows(2).any(|c| c[0].tick >= c[1].tick)
            || self.checkpoints.iter().any(|c| c.tick > self.final_tick)
        {
            return Err("Invalid checkpoint ordering".into());
        }
        let mut world = World::new(self.config.clone())?;
        let mut ci = 0;
        let mut hi = 0;
        loop {
            while ci < self.commands.len() && self.commands[ci].tick == world.state.tick {
                world.command(self.commands[ci].clone())?;
                ci += 1;
            }
            if hi < self.checkpoints.len() && self.checkpoints[hi].tick == world.state.tick {
                if self.checkpoints[hi].hash != world.hash() {
                    return Err(format!("Replay mismatch at tick {}", world.state.tick));
                }
                hi += 1;
            }
            if world.state.tick == self.final_tick {
                break;
            }
            world.step();
        }
        world.validate()?;
        Ok(world)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn commands_and_same_tick_order_replay() {
        let mut world = World::new(Config::default()).unwrap();
        world
            .command(Command {
                tick: 0,
                temperature: 1500,
                regeneration: 10,
            })
            .unwrap();
        world.advance(300);
        world
            .command(Command {
                tick: 300,
                temperature: 1000,
                regeneration: 5,
            })
            .unwrap();
        world
            .command(Command {
                tick: 300,
                temperature: 2000,
                regeneration: 15,
            })
            .unwrap();
        world.advance(500);
        assert_eq!(
            Replay::from_world(&world).verify().unwrap().hash(),
            world.hash()
        );
    }
    #[test]
    fn mismatch_and_invalid_order_rejected() {
        let world = World::new(Config::default()).unwrap();
        let mut replay = Replay::from_world(&world);
        replay.checkpoints[0].hash = "bad".into();
        assert!(replay.verify().unwrap_err().contains("mismatch"));
        replay.commands.push(Command {
            tick: 1,
            temperature: 2000,
            regeneration: 12,
        });
        assert!(replay.verify().unwrap_err().contains("ordering"));
    }
}
