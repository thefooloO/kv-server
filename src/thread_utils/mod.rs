pub mod pool;
pub mod worker;

pub enum Task {
    Job(Box<dyn FnOnce() + 'static + Send>),
    Shutdown
}