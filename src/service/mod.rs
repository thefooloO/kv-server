pub mod command;

use std::sync::Arc;
use crate::{KvError, MemTable};
use crate::pb::abi::{command_request, CommandRequest, CommandResponse, Kvpair};
use crate::storage::Storage;

// 抽象Command
pub trait CommandService {
    fn execute(self, storage: &impl Storage) -> CommandResponse;
}

pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>
}

pub struct ServiceInner<Store> {
    store: Store
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

impl<Store> Service<Store>
    where Store: Storage
{
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner {
                store
            })
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        dispatch(cmd, &self.inner.store)
    }
}

pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(command_request::RequestData::Hget(command)) => command.execute(store),
        Some(command_request::RequestData::Hset(command)) => command.execute(store),
        _ => KvError::InvalidCommand("invalid command".to_string()).into()
    }
}