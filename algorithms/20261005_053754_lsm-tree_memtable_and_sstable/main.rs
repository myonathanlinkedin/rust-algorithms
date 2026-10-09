use std::collections::BTreeMap;
use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::{PathBuf};
use std::env;
use std::cmp::Ordering;

type Key = u64;
type Value = u64;

#[derive(Debug)]
struct SSTable {
    path: PathBuf,
    entries: Vec<(Key, Value)>,
}

impl SSTable {
    fn write_to_file(path: &PathBuf, entries: &[(Key, Value)]) -> std::io::Result<()> {
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        for &(k, v) in entries {
            file.write_all(&k.to_le_bytes())?;
            file.write_all(&v.to_le_bytes())?;
        }
        file.flush()?;
        Ok(())
    }

    fn read_from_file(path: &PathBuf) -> std::io::Result<Vec<(Key, Value)>> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let mut entries = Vec::with_capacity(buf.len() / 16);
        let mut i = 0;
        while i + 16 <= buf.len() {
            let k = Key::from_le_bytes(buf[i..i+8].try_into().unwrap());
            let v = Value::from_le_bytes(buf[i+8..i+16].try_into().unwrap());
            entries.push((k, v));
            i += 16;
        }
        Ok(entries)
    }

    fn new(path: PathBuf, mut entries: Vec<(Key, Value)>) -> std::io::Result<Self> {
        entries.sort_by_key(|&(k, _)| k);
        Self::write_to_file(&path, &entries)?;
        Ok(SSTable { path, entries })
    }

    fn load(path: PathBuf) -> std::io::Result<Self> {
        let entries = Self::read_from_file(&path)?;
        Ok(SSTable { path, entries })
    }

    fn search(&self, key: Key) -> Option<Value> {
        self.entries.binary_search_by(|&(k, _)| k.cmp(&key))
            .ok()
            .map(|idx| self.entries[idx].1)
    }
}

struct LSMTree {
    memtable: BTreeMap<Key, Value>,
    sstables: Vec<SSTable>,
    threshold: usize,
    max_sstables: usize,
    dir: PathBuf,
    next_id: usize,
}

impl LSMTree {
    fn new(threshold: usize, max_sstables: usize) -> Self {
        let mut dir = env::temp_dir();
        dir.push(format!("lsm_{}", std::process::id()));
        std::fs::create_dir_all(&dir).ok();
        LSMTree {
            memtable: BTreeMap::new(),
            sstables: Vec::new(),
            threshold,
            max_sstables,
            dir,
            next_id: 0,
        }
    }

    fn make_path(&mut self) -> PathBuf {
        let mut p = self.dir.clone();
        p.push(format!("sstable_{}.dat", self.next_id));
        self.next_id += 1;
        p
    }

    fn put(&mut self, key: Key, value: Value) {
        self.memtable.insert(key, value);
        if self.memtable.len() > self.threshold {
            self.flush().unwrap();
        }
    }

    fn get(&self, key: Key) -> Option<Value> {
        if let Some(v) = self.memtable.get(&key) {
            return Some(*v);
        }
        for sst in self.sstables.iter().rev() {
            if let Some(v) = sst.search(key) {
                return Some(v);
            }
        }
        None
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.memtable.is_empty() {
            return Ok(());
        }
        let entries: Vec<(Key, Value)> = self.memtable.iter().map(|(&k, &v)| (k, v)).collect();
        let path = self.make_path();
        let sst = SSTable::new(path, entries)?;
        self.sstables.push(sst);
        self.memtable.clear();
        if self.sstables.len() > self.max_sstables {
            self.compact()?;
        }
        Ok(())
    }

    fn compact(&mut self) -> std::io::Result<()> {
        // Merge all sstables into one sorted vector, newest value wins.
        let mut merged: Vec<(Key, Value)> = Vec::new();
        for sst in &self.sstables {
            for &(k, v) in &sst.entries {
                match merged.binary_search_by(|&(mk, _)| mk.cmp(&k)) {
                    Ok(idx) => merged[idx].1 = v, // overwrite with newer
                    Err(idx) => merged.insert(idx, (k, v)),
                }
            }
        }
        // Remove old files
        for sst in &self.sstables {
            let _ = remove_file(&sst.path);
        }
        self.sstables.clear();
        // Write merged as single sstable
        let path = self.make_path();
        let new_sst = SSTable::new(path, merged)?;
        self.sstables.push(new_sst);
        Ok(())
    }
}

fn main() {
    // Basic functionality test
    let mut lsm = LSMTree::new(4, 3);
    for i in 1..=10 {
        lsm.put(i, i * 10);
    }
    // Ensure all keys are retrievable
    for i in 1..=10 {
        assert_eq!(lsm.get(i), Some(i * 10));
    }
    // Overwrite some keys
    lsm.put(5, 555);
    lsm.put(9, 999);
    assert_eq!(lsm.get(5), Some(555));
    assert_eq!(lsm.get(9), Some(999));
    // Force flush and compaction
    lsm.flush().unwrap();
    lsm.compact().unwrap();
    for i in 1..=10 {
        let expected = match i {
            5 => 555,
            9 => 999,
            _ => i * 10,
        };
        assert_eq!(lsm.get(i), Some(expected));
    }
    // Clean up temporary directory
    std::fs::remove_dir_all(lsm.dir).ok();
}
