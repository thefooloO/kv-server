extern crate core;

mod pb;
mod net;
mod error;
mod event;
mod service;
mod storage;

pub use error::KvError;
use crate::pb::abi::{CommandRequest, Hget, Value, value};
use crate::service::Service;
use crate::storage::memory::MemTable;
use crate::storage::Storage;

fn main() {
    println!("kv server!");
}


// fn test() {
//     let service = Service::new(MemTable::default());
//     service.execute(CommandRequest::new_hset("table".to_string(), "key".to_string(), Value {
//         value: Some(value::Value::String("values".to_string()))
//     }));
//
//     let res = service.execute(CommandRequest::new_hget("table".to_string(), "key".to_string()));
//     for x in res.values.iter() {
//         match &x.value {
//             Some(value::Value::String(v)) => println!("{}", v),
//             _ => {}
//         }
//     }
//
//     match &(res.values[0].value) {
//         Some(value::Value::String(v)) => println!("{}", v),
//         _ => {}
//     }
// }