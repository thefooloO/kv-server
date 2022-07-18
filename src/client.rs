pub mod error;
pub mod event;
pub mod net;
pub mod pb;
pub mod service;
pub mod storage;
pub use error::KvError;

use crate::net::basic::ProstClientStream;
use crate::pb::abi::CommandRequest;
use anyhow::Result;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:9527";
    let stream = TcpStream::connect(addr).await?;
    let mut client = ProstClientStream::new(stream);
    let cmd = CommandRequest::new_hget("table", "hello");
    let data = client.execute(cmd).await?;
    println!("{}",data.status);
    Ok(())
}