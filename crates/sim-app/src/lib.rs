use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
pub use sim_core::model::SIMULATION_VERSION;
use sim_core::{persistence, replay::Replay, Command, Config, OrganismId, World};
use std::{
    path::PathBuf,
    sync::{mpsc, Arc},
    thread,
    time::{Duration, Instant},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Action {
    Snapshot,
    New { config: Config },
    Control { running: bool, speed: u32 },
    Environment { temperature: i32, regeneration: i32 },
    Detail { id: u64 },
    Species,
    History,
    Telemetry,
    Save { path: Option<String> },
    Load { path: Option<String> },
    Replay,
}
struct Request {
    action: Action,
    reply: mpsc::Sender<Result<Value, String>>,
}
#[derive(Clone)]
pub struct App {
    sender: Arc<mpsc::Sender<Request>>,
}
impl App {
    pub fn start(directory: PathBuf) -> Result<Self, String> {
        std::fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
        // Evidence-based starter habitat; larger capacities remain configurable.
        let world = World::new(Config {
            starting_population: 50,
            population_limit: 200,
            ..Config::default()
        })?;
        let (sender, receiver) = mpsc::channel::<Request>();
        thread::Builder::new().name("genesis-simulation".into()).spawn(move||{
            let mut world=world;let mut running=false;let mut speed=1;let mut last=Instant::now();
            let mut autosave_slot=0;let mut autosave_error:Option<String>=None;
            tracing::info!(seed=world.state.config.seed,"simulation_created");
            loop{
                let timeout=if running&&speed==0{Duration::from_millis(1)}else{Duration::from_millis(16)};
                match receiver.recv_timeout(timeout){
                    Ok(request)=>{
                        let result=match request.action{
                            Action::Snapshot=>Ok(snapshot(&world,running,speed,&autosave_error)),
                            Action::New{config}=>World::new(config).map(|new|{world=new;running=true;last=Instant::now();autosave_error=None;tracing::info!(seed=world.state.config.seed,"simulation_created");snapshot(&world,running,speed,&autosave_error)}),
                            Action::Control{running:r,speed:v}=>{
                                if [0,1,4,16,64].contains(&v){running=r;speed=v;last=Instant::now();Ok(snapshot(&world,running,speed,&autosave_error))}else{Err("Unsupported simulation speed".into())}
                            },
                            Action::Environment{temperature,regeneration}=>world.command(Command{tick:world.state.tick,temperature,regeneration}).map(|_|snapshot(&world,running,speed,&autosave_error)),
                            Action::Detail{id}=>world.state.organisms.iter().find(|o|o.id==OrganismId(id)).map(|o|json!(o)).ok_or("This organism is no longer alive".into()),
                            Action::Species=>Ok(json!(world.state.species.values().collect::<Vec<_>>())),
                            Action::History=>Ok(json!(world.state.history)),
                            Action::Telemetry=>Ok(json!(world.state.telemetry)),
                            Action::Save{path}=>{
                                let path=path.map(PathBuf::from).unwrap_or_else(||directory.join("manual.genesis"));
                                persistence::save_atomic(&world,&path).map(|_|{tracing::info!(path=%path.display(),tick=world.state.tick,"save_complete");json!({"path":path,"tick":world.state.tick,"hash":world.hash()})})
                            },
                            Action::Load{path}=>{
                                let path=path.map(PathBuf::from).unwrap_or_else(||directory.join("manual.genesis"));
                                persistence::load(&path).map(|loaded|{world=loaded;running=false;last=Instant::now();autosave_error=None;tracing::info!(path=%path.display(),tick=world.state.tick,"load_complete");snapshot(&world,running,speed,&autosave_error)})
                            },
                            Action::Replay=>{
                                let replay=Replay::from_world(&world);
                                // Replay runs on a separate thread and does not stop the simulation worker.
                                let reply=request.reply.clone();
                                thread::spawn(move||{let result=replay.verify().map(|verified|json!({"tick":verified.state.tick,"hash":verified.hash(),"verified":true}));if let Err(ref error)=result{tracing::error!(%error,"replay_failed");}let _=reply.send(result);});continue;
                            },
                        };
                        if let Err(ref error)=result{tracing::warn!(%error,"application_action_failed");}
                        let _=request.reply.send(result);
                    },
                    Err(mpsc::RecvTimeoutError::Disconnected)=>break,
                    Err(mpsc::RecvTimeoutError::Timeout)=>{},
                }
                if running&&(speed==0||last.elapsed()>=Duration::from_millis(16)){
                    let before=world.state.tick;world.advance(if speed==0{64}else{u64::from(speed)});last=Instant::now();
                    if before/5000<world.state.tick/5000{
                        let path=directory.join(format!("autosave-{}.genesis",autosave_slot));autosave_slot=(autosave_slot+1)%3;
                        autosave_error=persistence::save_atomic(&world,&path).err();
                        if let Some(ref error)=autosave_error{tracing::error!(%error,"autosave_failed");}
                    }
                }
            }
        }).map_err(|e|e.to_string())?;
        Ok(Self {
            sender: Arc::new(sender),
        })
    }
    pub fn execute(&self, action: Action) -> Result<Value, String> {
        let (reply, receiver) = mpsc::channel();
        self.sender
            .send(Request { action, reply })
            .map_err(|_| "Simulation worker unavailable")?;
        receiver
            .recv_timeout(Duration::from_secs(300))
            .map_err(|_| "Simulation request timed out")?
    }
}
fn snapshot(world: &World, running: bool, speed: u32, error: &Option<String>) -> Value {
    let s = &world.state;
    json!({"simulation_version":sim_core::model::SIMULATION_VERSION,"seed":s.config.seed.to_string(),"tick":s.tick,"generation":s.organisms.iter().map(|o|o.generation).max().unwrap_or(0),"size":s.config.size,"population":s.organisms.len(),"species_count":s.species.values().filter(|sp|sp.population>0).count(),"temperature":s.environment.temperature,"regeneration":s.environment.regeneration,"running":running,"speed":speed,"counters":s.counters,"autosave_error":error,"cells":s.environment.cells.iter().map(|c|[c.food,c.temperature_offset,c.fertility]).collect::<Vec<_>>(),"organisms":s.organisms.iter().map(|o|json!({"id":o.id.0,"x":o.x,"y":o.y,"dx":o.dx,"dy":o.dy,"species_id":o.species_id.0,"body_size":o.phenotype.body_size,"speed":o.phenotype.speed,"carnivory":o.phenotype.carnivory})).collect::<Vec<_>>()})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn worker_manual_save_load_and_replay() {
        let dir = std::env::temp_dir().join(format!("genesis-app-test-{}", std::process::id()));
        let app = App::start(dir.clone()).unwrap();
        let initial = app.execute(Action::Snapshot).unwrap();
        assert_eq!(initial["tick"], 0);
        app.execute(Action::Environment {
            temperature: 1200,
            regeneration: 5,
        })
        .unwrap();
        let save = app.execute(Action::Save { path: None }).unwrap();
        app.execute(Action::Environment {
            temperature: 3000,
            regeneration: 1,
        })
        .unwrap();
        let loaded = app.execute(Action::Load { path: None }).unwrap();
        assert_eq!(loaded["temperature"], 1200);
        let replay = app.execute(Action::Replay).unwrap();
        assert_eq!(replay["hash"], save["hash"]);
        assert!(app
            .execute(Action::Control {
                running: true,
                speed: 999
            })
            .is_err());
        drop(app);
        std::fs::remove_dir_all(dir).unwrap();
    }
}
