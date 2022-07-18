pub mod error;
pub mod event;
pub mod net;
pub mod pb;
pub mod service;
pub mod storage;
pub use error::KvError;

use kv_server::net::basic::ProstServerStream;
use kv_server::service::{Service, ServiceInner};
use kv_server::storage::memory::MemTable;
use anyhow::Result;
use tokio::net::TcpListener;
use tracing::info;


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:9527";
    let service: Service = ServiceInner::new(MemTable::new()).into();
    let listener = TcpListener::bind(addr).await?;
    println!("Start listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Client {:?} connected", addr);
        let stream = ProstServerStream::new(stream, service.clone());
        tokio::spawn(async move {
            stream.process().await
        });
    }
}