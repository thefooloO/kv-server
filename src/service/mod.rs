pub mod command;

use crate::pb::abi::CommandResponse;
use crate::storage::Storage;

// 抽象Command
pub trait CommandService {
    fn execute(self, storage: &impl Storage) -> CommandResponse;
}
