use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::thread_utils::Task;
use crate::thread_utils::Task::{ Job, Shutdown };

const BOUND: usize = 10;

#[derive(Clone)]
pub struct Worker {
    sender   : mpsc::SyncSender<Task>,
    receiver : Arc<Mutex::<mpsc::Receiver<Task>>>,
    workloads: usize,
}

impl Worker {
    pub fn new() -> Self {

        let (sender, receiver) = mpsc::sync_channel(BOUND);

        let mut worker = Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            workloads: 0,
        };

        let res = worker.clone();

        thread::spawn(move || {
            worker.run();
        });

        return res;
    }

    pub fn send(&self, task: Task) {
        self.sender.send(task);
    }

    pub fn run(&mut self) {
        loop {
            let task = self.receiver.lock().unwrap().recv().unwrap();

            match task {
                Job(job) => {
                    self.workloads += 1;
                    job();
                    self.workloads -= 1;
                },
                Shutdown => break
            }
        };
    }

    pub fn workloads(&self) -> usize {
        self.workloads
    }
}