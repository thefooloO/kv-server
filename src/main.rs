mod error;
mod pb;
mod service;
mod storage;

pub use error::KvError;
use crate::pb::abi::{Value, value};
use crate::storage::memory::MemTable;
use crate::storage::Storage;

fn main() {
    println!("kv server!");
}


fn test() {
    let table = "table";
    let mem = MemTable::new();
    mem.set(table,"key".to_string(), Value {
        value: Some(value::Value::String("value".to_string()))
    });

    match mem.get(table, "key") {
        Ok(Some(v)) => {
            match v.value {
                Some(value::Value::String(x)) => println!("{}", x) ,
                _ => println!("none") ,
            }
        },
        _ => {}
    }
}