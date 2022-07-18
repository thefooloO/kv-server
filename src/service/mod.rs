pub mod command;

use crate::event::{Notify, NotifyMut};
use crate::pb::abi::{command_request, CommandRequest, CommandResponse};
use crate::storage::memory::MemTable;
use crate::storage::Storage;
use crate::KvError;
use std::sync::Arc;

pub trait CommandService {
    fn execute(self, storage: &impl Storage) -> CommandResponse;
}

pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

pub struct ServiceInner<Store> {
    store: Store,
    on_received: Vec<fn(&CommandRequest)>,
    on_executed: Vec<fn(&CommandResponse)>,
    on_before_send: Vec<fn(&mut CommandResponse)>,
    on_after_send: Vec<fn()>,
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<Store> Service<Store>
where
    Store: Storage,
{
    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        self.inner.on_received.notify(&cmd);
        let mut resp = dispatch(cmd, &self.inner.store);
        self.inner.on_executed.notify(&resp);
        self.inner.on_before_send.notify(&mut resp);
        if !self.inner.on_after_send.is_empty() {}
        resp
    }
}

impl<Store> ServiceInner<Store>
where
    Store: Storage,
{
    pub fn new(store: Store) -> Self {
        Self {
            store,
            on_received: Vec::new(),
            on_executed: Vec::new(),
            on_before_send: Vec::new(),
            on_after_send: Vec::new(),
        }
    }

    pub fn fn_received(mut self, f: fn(&CommandRequest)) -> Self {
        self.on_received.push(f);
        self
    }

    pub fn fn_executed(mut self, f: fn(&CommandResponse)) -> Self {
        self.on_executed.push(f);
        self
    }

    pub fn fn_before_send(mut self, f: fn(&mut CommandResponse)) -> Self {
        self.on_before_send.push(f);
        self
    }

    pub fn fn_after_send(mut self, f: fn()) -> Self {
        self.on_after_send.push(f);
        self
    }
}

impl<Store> From<ServiceInner<Store>> for Service<Store>
where
    Store: Storage,
{
    fn from(inner: ServiceInner<Store>) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(command_request::RequestData::Hget(command)) => command.execute(store),
        Some(command_request::RequestData::Hset(command)) => command.execute(store),
        _ => KvError::InvalidCommand("invalid command".to_string()).into(),
    }
}
