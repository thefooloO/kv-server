mod error;
mod pb;
mod service;
mod storage;

pub use error::KvError;

fn main() {
    println!("kv server!");
}
