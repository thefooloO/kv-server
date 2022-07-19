use std::thread;

pub mod pool;
pub mod worker;

pub struct Job(pub Box<dyn FnOnce() + 'static + Send>);

impl Job {
    pub fn new(job: Box<dyn FnOnce() + 'static + Send>) -> Self {
        Self(job)
    }
}