use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use crate::KvError;
use crate::pb::abi::Kvpair;
use crate::pb::abi::value::Value;
use crate::storage::Storage;

#[derive(Clone, Debug, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {

    fn get(&self, table: &str, key: &str) -> Result<Option<crate::pb::abi::Value>, KvError> {
        todo!()
    }

    fn set(&self, table: &str, key: String, value: crate::pb::abi::Value) -> Result<Option<crate::pb::abi::Value>, KvError> {
        todo!()
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<crate::pb::abi::Value>, KvError> {
        todo!()
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        todo!()
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        todo!()
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item=Kvpair>>, KvError> {
        todo!()
    }
}