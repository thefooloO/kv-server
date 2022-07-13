use crate::pb::abi::*;
use crate::service::CommandService;
use crate::storage::Storage;
use crate::KvError;

impl CommandService for Hget {
    fn execute(self, storage: &impl Storage) -> CommandResponse {
        match storage.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, storage: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => match storage.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => KvError::Internal("internal error".to_string()).into(),
                Err(e) => e.into(),
            },
            None => KvError::InvalidCommand("invalid command".to_string()).into(),
        }
    }
}
