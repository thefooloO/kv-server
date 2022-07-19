use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use crate::thread_utils::Job;

const BOUND: usize = 10;

#[derive(Clone)]
pub struct Worker {
    sender   : mpsc::SyncSender<Job>,
    receiver : Arc<Mutex::<mpsc::Receiver<Job>>>,
    workloads: usize,
    shutdown : bool
}

impl Worker {
    pub fn new() -> Self {

        let (sender, receiver) = mpsc::sync_channel(BOUND);

        let mut worker = Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            workloads: 0,
            shutdown : false
        };

        let res = worker.clone();

        thread::spawn(move || {
            worker.run();
        });

        return res;
    }

    pub fn send(&self, job: Job) {
        self.sender.send(job);
    }

    pub fn run(&mut self) {
        loop {
            if self.shutdown {
                break;
            }

            let job = self.receiver.lock().unwrap().recv().unwrap().0;
            self.workloads += 1;
            job();
            self.workloads -= 1;
        };
    }

    pub fn shutdown(&mut self) {
        self.shutdown = true;
    }

    pub fn workloads(&self) -> usize {
        self.workloads
    }
}