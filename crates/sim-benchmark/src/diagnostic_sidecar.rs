//! Compressed per-world diagnostic evidence. Never part of simulation authority.
use flate2::{write::GzEncoder, Compression};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};
const POINTER: &str = "/phase2/viability/ecology";
pub fn prepare(directory: &Path, journal: &Path) -> Result<PathBuf, String> {
    let parent = |p: &Path| {
        p.parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."))
            .canonicalize()
    };
    if directory.file_name().is_none()
        || parent(directory).map_err(|e| e.to_string())?
            != parent(journal).map_err(|e| e.to_string())?
    {
        return Err("Diagnostics directory must be a fresh sibling of --output journal".into());
    }
    if journal.exists() {
        return Err("Diagnostics journal already exists; refusing overwrite".into());
    }
    fs::create_dir(directory).map_err(|e| format!("Fresh diagnostics directory required: {e}"))?;
    Ok(directory.to_path_buf())
}
struct HashedWriter<W> {
    inner: W,
    digest: Sha256,
    bytes: u64,
}
impl<W: Write> Write for HashedWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(bytes)?;
        self.digest.update(&bytes[..n]);
        self.bytes += n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
pub fn detach(result: &mut Value, directory: &Path) -> Result<(), String> {
    let seed = result["seed"].as_u64().ok_or("Missing diagnostic seed")?;
    let tick = result["ticks"].as_u64().ok_or("Missing diagnostic tick")?;
    let temporal = result
        .pointer(POINTER)
        .and_then(|e| e.get("r4_temporal"))
        .ok_or("Missing temporal observations")?;
    if temporal["version"] != 1 {
        return Err("Unsupported temporal observation schema".into());
    }
    let filename = format!("seed-{seed}-tick-{tick}-temporal1.json.gz");
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(directory.join(&filename))
        .map_err(|e| e.to_string())?;
    let mut writer = HashedWriter {
        inner: GzEncoder::new(BufWriter::new(file), Compression::default()),
        digest: Sha256::new(),
        bytes: 0,
    };
    serde_json::to_writer(&mut writer, temporal).map_err(|e| e.to_string())?;
    let HashedWriter {
        inner,
        digest,
        bytes,
    } = writer;
    let mut file = inner.finish().map_err(|e| e.to_string())?;
    file.flush().map_err(|e| e.to_string())?;
    file.get_ref().sync_all().map_err(|e| e.to_string())?;
    let folder = directory
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("Invalid diagnostic directory name")?;
    let ecology = result
        .pointer_mut(POINTER)
        .and_then(Value::as_object_mut)
        .ok_or("Missing ecology ledger")?;
    ecology.remove("r4_temporal");
    ecology.insert("r4_temporal_ref".into(),json!({"version":1,"path":format!("{folder}/{filename}"),"sha256":format!("{:x}",digest.finalize()),"uncompressed_bytes":bytes,"seed":seed,"tick":tick}));
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    #[test]
    fn sidecars_preserve_data_checksum_and_refuse_overwrite() {
        let root = std::env::temp_dir().join(format!(
            "genesis-sidecar-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        let dir = root.join("temporal");
        let journal = root.join("run.jsonl");
        assert!(prepare(&dir, &root.join("missing/run.jsonl")).is_err());
        prepare(&dir, &journal).unwrap();
        assert!(prepare(&dir, &journal).is_err());
        let evidence =
            json!({"version":1,"samples":{"0":{"1":{"energy":[0,12,3],"population":8}}}});
        let original = json!({"seed":7,"ticks":1000,"phase2":{"viability":{"ecology":{"version":4,"r4_temporal":evidence}}}});
        let mut result = original.clone();
        detach(&mut result, &dir).unwrap();
        let reference = &result.pointer(POINTER).unwrap()["r4_temporal_ref"];
        let path = root.join(reference["path"].as_str().unwrap());
        let compressed = fs::read(&path).unwrap();
        let mut raw = Vec::new();
        flate2::read::GzDecoder::new(&compressed[..])
            .read_to_end(&mut raw)
            .unwrap();
        assert_eq!(serde_json::from_slice::<Value>(&raw).unwrap(), evidence);
        assert_eq!(reference["sha256"], format!("{:x}", Sha256::digest(&raw)));
        assert_eq!(reference["uncompressed_bytes"], raw.len() as u64);
        assert!(result
            .pointer(POINTER)
            .unwrap()
            .get("r4_temporal")
            .is_none());
        let mut duplicate = original.clone();
        assert!(detach(&mut duplicate, &dir).is_err());
        assert_eq!(duplicate, original);
        assert_eq!(fs::read(&path).unwrap(), compressed);
        fs::remove_file(path).unwrap();
        fs::remove_dir(dir).unwrap();
        fs::remove_dir(root).unwrap();
    }
}
