use crate::{
    model::{State, SIMULATION_VERSION},
    rng::RNG_VERSION,
    World,
};
use bincode::Options;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::{
    io::{Read, Write},
    path::Path,
};

pub const SAVE_FORMAT_VERSION: u32 = 1;
pub const MAX_SAVE_BYTES: u64 = 128 * 1024 * 1024;
const MAGIC: &[u8; 8] = b"GENESIS1";
const HEADER: usize = 60;

pub fn encode(world: &World) -> Result<Vec<u8>, String> {
    world.validate()?;
    let raw = bincode::serialize(&world.state).map_err(|e| e.to_string())?;
    if raw.len() as u64 > MAX_SAVE_BYTES {
        return Err("World exceeds the Phase 1 save size limit".into());
    }
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(&raw).map_err(|e| e.to_string())?;
    let compressed = encoder.finish().map_err(|e| e.to_string())?;
    let mut bytes = Vec::with_capacity(HEADER + compressed.len());
    bytes.extend_from_slice(MAGIC);
    for version in [SAVE_FORMAT_VERSION, SIMULATION_VERSION, RNG_VERSION] {
        bytes.extend_from_slice(&version.to_le_bytes());
    }
    bytes.extend_from_slice(&(raw.len() as u64).to_le_bytes());
    bytes.extend_from_slice(blake3::hash(&raw).as_bytes());
    bytes.extend(compressed);
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<World, String> {
    if bytes.len() < HEADER || bytes.len() as u64 > MAX_SAVE_BYTES || &bytes[..8] != MAGIC {
        return Err("Not a valid GENESIS save file".into());
    }
    for (offset, expected) in [
        (8, SAVE_FORMAT_VERSION),
        (12, SIMULATION_VERSION),
        (16, RNG_VERSION),
    ] {
        let version = u32::from_le_bytes(
            bytes[offset..offset + 4]
                .try_into()
                .map_err(|_| "Invalid version")?,
        );
        if version != expected {
            return Err(format!(
                "Incompatible save version at offset {offset}: {version}, expected {expected}"
            ));
        }
    }
    let expected_len = u64::from_le_bytes(bytes[20..28].try_into().map_err(|_| "Invalid size")?);
    if expected_len > MAX_SAVE_BYTES {
        return Err("Save decompression limit exceeded".into());
    }
    let mut raw = Vec::new();
    ZlibDecoder::new(&bytes[HEADER..])
        .take(MAX_SAVE_BYTES + 1)
        .read_to_end(&mut raw)
        .map_err(|e| format!("Cannot decompress save: {e}"))?;
    if raw.len() as u64 != expected_len || blake3::hash(&raw).as_bytes() != &bytes[28..60] {
        return Err("Save checksum or size mismatch".into());
    }
    let state: State = bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(MAX_SAVE_BYTES)
        .reject_trailing_bytes()
        .deserialize(&raw)
        .map_err(|e| format!("Invalid save payload: {e}"))?;
    let world = World { state };
    world.validate()?;
    Ok(world)
}

pub fn save_atomic(world: &World, path: &Path) -> Result<(), String> {
    let bytes = encode(world)?;
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut file = tempfile::NamedTempFile::new_in(parent)
        .map_err(|e| format!("Cannot create temporary save: {e}"))?;
    file.write_all(&bytes)
        .and_then(|_| file.as_file().sync_all())
        .map_err(|e| format!("Cannot write save: {e}"))?;
    file.persist(path)
        .map_err(|e| format!("Cannot atomically replace save: {}", e.error))?;
    Ok(())
}
pub fn load(path: &Path) -> Result<World, String> {
    let file = std::fs::File::open(path).map_err(|e| format!("Cannot open save: {e}"))?;
    if file.metadata().map_err(|e| e.to_string())?.len() > MAX_SAVE_BYTES {
        return Err("Save file too large".into());
    }
    let mut bytes = Vec::new();
    file.take(MAX_SAVE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    decode(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config;
    #[test]
    fn roundtrip_and_continuation() {
        let mut a = World::new(Config::default()).unwrap();
        a.advance(321);
        let mut b = decode(&encode(&a).unwrap()).unwrap();
        assert_eq!(a.hash(), b.hash());
        a.advance(500);
        b.advance(500);
        assert_eq!(a.hash(), b.hash());
    }
    #[test]
    fn corrupted_truncated_versioned_saves_rejected() {
        let world = World::new(Config::default()).unwrap();
        let bytes = encode(&world).unwrap();
        for len in [0, 4, 59, 60, bytes.len() - 10] {
            assert!(decode(&bytes[..len]).is_err());
        }
        let mut bad = bytes.clone();
        bad[30] ^= 1;
        assert!(decode(&bad).is_err());
        let mut bad = bytes.clone();
        bad[12] = 99;
        assert!(decode(&bad).unwrap_err().contains("Incompatible"));
        let mut bad = bytes;
        bad[20..28].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(decode(&bad).is_err());
    }
    #[test]
    fn atomic_overwrite_and_failed_load_preserve_previous() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("world.genesis");
        let mut w = World::new(Config::default()).unwrap();
        save_atomic(&w, &path).unwrap();
        w.advance(100);
        save_atomic(&w, &path).unwrap();
        assert_eq!(load(&path).unwrap().hash(), w.hash());
        assert!(save_atomic(&w, &dir.path().join("missing/world.genesis")).is_err());
        assert_eq!(load(&path).unwrap().hash(), w.hash());
    }
    #[test]
    fn future_chronology_and_registry_counters_rejected() {
        let world = World::new(Config::default()).unwrap();
        let mut bad = world.clone();
        bad.state.organisms[0].last_mating = 1;
        assert!(encode(&bad).is_err());
        let mut bad = world.clone();
        bad.state
            .lineages
            .values_mut()
            .next()
            .unwrap()
            .candidate_since = Some(1);
        assert!(encode(&bad).is_err());
        let mut bad = world.clone();
        bad.state.next_species = 1;
        assert!(encode(&bad).is_err());
        let mut bad = world;
        bad.state.tick = u64::MAX;
        assert!(encode(&bad).is_err());
    }
    #[test]
    fn invalid_state_rejected_before_writing() {
        let mut w = World::new(Config::default()).unwrap();
        w.state.organisms[0].energy = -1;
        assert!(encode(&w).is_err());
    }
}
