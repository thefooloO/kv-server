use std::thread;
use std::thread::sleep;
use crate::thread_utils::Job;
use crate::thread_utils::worker::Worker;

pub struct ThreadPool {
    size: usize,
    workers: Vec<Worker>
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            workers: Vec::new()
        }
    }

    pub fn submit(&mut self, job: Job) {
        if self.workers.len() < self.size {
            self.workers.push(Worker::new());
        }
        self.minWorkLoadsWorker().send(job);
    }

    pub fn shutdown(&self) {
        for mut worker in self.workers.clone() {
            worker.shutdown();
        }
    }


    fn minWorkLoadsWorker(&self) -> &Worker {
        let mut res = &self.workers[0];
        for worker in &self.workers {
            if res.workloads() > worker.workloads() {
                res = worker;
            }
        }
        return res;
    }
}