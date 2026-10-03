//! Application preferences and save markers are deliberately outside world truth.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sim_core::World;
use std::{
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Default, Serialize, Deserialize)]
struct Preferences {
    recent: Vec<Recent>,
}
#[derive(Serialize, Deserialize)]
struct Recent {
    path: String,
    last_opened_ms: u64,
}
pub(crate) struct Files {
    preferences: Preferences,
    preferences_path: PathBuf,
    pub(crate) current_path: Option<PathBuf>,
    saved: Option<(u64, usize)>,
    warning: Option<String>,
}
pub(crate) fn save_path(path: PathBuf) -> Result<PathBuf, String> {
    if path.as_os_str().is_empty() || path.file_name().is_none() {
        return Err("Choose a file name for your world".into());
    }
    if path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("genesis"))
    {
        return Ok(path);
    }
    let mut name = path.into_os_string();
    name.push(".genesis");
    Ok(PathBuf::from(name))
}
impl Files {
    pub(crate) fn new(directory: &Path) -> Self {
        let preferences_path = directory.join("preferences.json");
        let (preferences, warning) = match std::fs::read(&preferences_path) {
            Ok(bytes) => match serde_json::from_slice(&bytes) {
                Ok(p) => (p, None),
                Err(e) => (
                    Preferences::default(),
                    Some(format!("Recent worlds could not be read: {e}")),
                ),
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (Preferences::default(), None),
            Err(e) => (
                Preferences::default(),
                Some(format!("Recent worlds could not be read: {e}")),
            ),
        };
        Self {
            preferences,
            preferences_path,
            current_path: None,
            saved: None,
            warning,
        }
    }
    pub(crate) fn reset(&mut self) {
        self.current_path = None;
        self.saved = None;
    }
    pub(crate) fn mark_saved(&mut self, world: &World, path: PathBuf) {
        self.saved = Some((world.state.tick, world.state.commands.len()));
        self.current_path = Some(path.clone());
        let key = path.to_string_lossy().to_string();
        self.preferences
            .recent
            .retain(|r| !same_path(&r.path, &key));
        self.preferences.recent.insert(
            0,
            Recent {
                path: key,
                last_opened_ms: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
            },
        );
        self.preferences.recent.truncate(5);
        self.persist();
    }
    fn persist(&mut self) {
        let result = (|| -> Result<(), String> {
            let mut file = tempfile::NamedTempFile::new_in(
                self.preferences_path
                    .parent()
                    .ok_or("Missing preference directory")?,
            )
            .map_err(|e| e.to_string())?;
            file.write_all(
                &serde_json::to_vec_pretty(&self.preferences).map_err(|e| e.to_string())?,
            )
            .and_then(|_| file.as_file().sync_all())
            .map_err(|e| e.to_string())?;
            file.persist(&self.preferences_path)
                .map_err(|e| e.to_string())?;
            Ok(())
        })();
        self.warning = result
            .err()
            .map(|e| format!("Your world is safe, but recent worlds could not be updated: {e}"));
        if let Some(ref warning) = self.warning {
            tracing::warn!(%warning,"preferences_failed");
        }
    }
    pub(crate) fn forget(&mut self, path: &str) {
        self.preferences
            .recent
            .retain(|r| !same_path(&r.path, path));
        self.persist();
    }
    pub(crate) fn status(&self, world: &World) -> Value {
        json!({"dirty":self.saved!=Some((world.state.tick,world.state.commands.len())),"current_path":self.current_path,"warning":self.warning,"recent":self.preferences.recent.iter().map(|r|json!({"path":r.path,"last_opened_ms":r.last_opened_ms,"available":Path::new(&r.path).is_file()})).collect::<Vec<_>>()})
    }
}
fn same_path(a: &str, b: &str) -> bool {
    if cfg!(windows) {
        a.eq_ignore_ascii_case(b)
    } else {
        a == b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sim_core::{Command, Config};
    #[test]
    fn extension_handling() {
        assert_eq!(
            save_path("world".into()).unwrap(),
            PathBuf::from("world.genesis")
        );
        assert_eq!(
            save_path("world.GENESIS".into()).unwrap(),
            PathBuf::from("world.GENESIS")
        );
        assert_eq!(
            save_path("notes.txt".into()).unwrap(),
            PathBuf::from("notes.txt.genesis")
        );
        assert!(save_path("".into()).is_err());
    }
    #[test]
    fn dirty_marker_tracks_tick_and_same_tick_commands_without_rng() {
        let dir = tempfile::tempdir().unwrap();
        let mut files = Files::new(dir.path());
        let mut world = World::new(Config::default()).unwrap();
        let hash = world.hash();
        assert_eq!(files.status(&world)["dirty"], true);
        files.mark_saved(&world, dir.path().join("a.genesis"));
        assert_eq!(world.hash(), hash);
        assert_eq!(files.status(&world)["dirty"], false);
        world
            .command(Command {
                tick: 0,
                temperature: 1800,
                regeneration: 12,
            })
            .unwrap();
        assert_eq!(files.status(&world)["dirty"], true);
        files.mark_saved(&world, dir.path().join("a.genesis"));
        world.step();
        assert_eq!(files.status(&world)["dirty"], true);
        files.reset();
        assert!(files.current_path.is_none());
    }
    #[test]
    fn recent_deduplicates_limits_persists_and_removes_missing() {
        let dir = tempfile::tempdir().unwrap();
        let world = World::new(Config::default()).unwrap();
        let mut files = Files::new(dir.path());
        for i in 0..7 {
            files.mark_saved(&world, dir.path().join(format!("{i}.genesis")));
        }
        let path = dir.path().join("3.genesis");
        files.mark_saved(&world, path.clone());
        let mut restored = Files::new(dir.path());
        let status = restored.status(&world);
        assert_eq!(status["recent"].as_array().unwrap().len(), 5);
        assert_eq!(status["recent"][0]["path"], path.to_string_lossy().as_ref());
        assert_eq!(status["recent"][0]["available"], false);
        restored.forget(&path.to_string_lossy());
        assert_eq!(
            Files::new(dir.path()).status(&world)["recent"]
                .as_array()
                .unwrap()
                .len(),
            4
        );
    }
    #[test]
    fn malformed_preferences_do_not_prevent_startup() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("preferences.json"), b"broken").unwrap();
        let world = World::new(Config::default()).unwrap();
        let status = Files::new(dir.path()).status(&world);
        assert!(status["warning"]
            .as_str()
            .unwrap()
            .contains("could not be read"));
        assert_eq!(status["recent"], json!([]));
    }
}
