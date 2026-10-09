use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecordType {
    A,
    CNAME,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DNSRecord {
    pub name: String,
    pub rtype: RecordType,
    pub value: String, // IP for A, target name for CNAME

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DNSZone {
    // Mapping from name to list of records (normally one per type)
    pub records: HashMap<String, Vec<DNSRecord>>,

}

impl DNSZone {
    pub fn new() -> Self {
        DNSZone {
            records: HashMap::new(),
        }
    }

    pub fn insert(&mut self, record: DNSRecord) {
        self.records
            .entry(record.name.clone())
            .or_insert_with(Vec::new)
            .push(record);
    }

    fn get_records(&self, name: &str) -> Option<&Vec<DNSRecord>> {
        self.records.get(name)
    }
}

pub struct Resolver<'a> {
    pub zone: &'a DNSZone,
    pub max_depth: usize,

}

impl<'a> Resolver<'a> {
    pub fn new(zone: &'a DNSZone) -> Self {
        Resolver { zone, max_depth: 10 }
    }

    /// Resolve a name to an IPv4 address string.
    /// Returns `None` if the name cannot be resolved or a loop is detected.
    pub fn resolve(&self, name: &str) -> Option<String> {
        self.resolve_internal(name, 0, &mut HashSet::new())
    }

    fn resolve_internal(
        &self,
        name: &str,
        depth: usize,
        visited: &mut HashSet<String>,
    ) -> Option<String> {
        if depth > self.max_depth {
            return None; // Exceeded recursion limit
        }
        if !visited.insert(name.to_string()) {
            return None; // Loop detected
        }

        let records = self.zone.get_records(name)?;
        // Prefer A record; otherwise follow CNAME
        for rec in records {
            match rec.rtype {
                RecordType::A => return Some(rec.value.clone()),
                RecordType::CNAME => {
                    // Recurse on the target name
                    return self.resolve_internal(&rec.value, depth + 1, visited);
                }
            }
        }
        None
    }
}