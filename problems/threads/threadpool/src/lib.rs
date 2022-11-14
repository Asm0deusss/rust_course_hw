#![forbid(unsafe_code)]
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;

pub enum Message<Job> {
    NewJob(Job),
    Terminate,
}

pub struct ThreadTask {
    thread: JoinHandle<()>,
}

impl ThreadTask {
    pub fn new<Job>(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message<Job>>>>) -> Self
    where
        Job: FnOnce() + Send + Sync + 'static,
        Message<Job>: Send + Sync,
    {
        let thread = std::thread::Builder::new()
            .name("Task ".to_string() + &id.to_string())
            .spawn(move || loop {
                let rec_guard = receiver.lock().unwrap();

                let message = rec_guard.recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        drop(rec_guard);
                        job();
                    }
                    Message::Terminate => {
                        break;
                    }
                }
            })
            .unwrap();

        Self { thread }
    }
}

pub struct ThreadPool<Job>
where
    Job: FnOnce() + Send + Sync + 'static,
    Message<Job>: Send + Sync,
{
    tasks: Vec<ThreadTask>,
    sender: mpsc::Sender<Message<Job>>,
}

impl<Job> ThreadPool<Job>
where
    Job: FnOnce() + Send + Sync + 'static,
    Message<Job>: Send + Sync,
{
    pub fn new(size: usize) -> Self {
        let (sender, receiver): (mpsc::Sender<Message<Job>>, mpsc::Receiver<Message<Job>>) =
            mpsc::channel();

        let mut tasks: Vec<ThreadTask> = vec![];

        let rec_mut = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            tasks.push(ThreadTask::new(id, rec_mut.clone()));
        }

        Self {
            tasks: (tasks),
            sender: (sender),
        }
    }

    pub fn execute(&self, job: Job) {
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl<Job> Drop for ThreadPool<Job>
where
    Job: FnOnce() + Send + Sync,
    Message<Job>: Send + Sync,
{
    fn drop(&mut self) {
        for _ in 0..self.tasks.len() {
            self.sender.send(Message::Terminate).unwrap();
        }

        while let Some(cur_thread) = self.tasks.pop() {
            cur_thread.thread.join().unwrap();
        }
    }
}
