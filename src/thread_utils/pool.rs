use crate::thread_utils::Task;
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

    pub fn submit(&mut self, task: Task) {
        if self.workers.len() < self.size {
            self.workers.push(Worker::new());
        }
        self.min_work_loads_worker().send(task);
    }

    pub fn shutdown(&self) {
        for worker in &self.workers {
            worker.send(Task::Shutdown);
        }
    }


    fn min_work_loads_worker(&self) -> &Worker {
        let mut res = &self.workers[0];
        for worker in &self.workers {
            if res.workloads() > worker.workloads() {
                res = worker;
            }
        }
        return res;
    }
}