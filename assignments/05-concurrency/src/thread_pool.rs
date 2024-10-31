use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Condvar, Mutex};
use std::collections::VecDeque;

pub struct ThreadPool<T: Send> {
    condition_queue: Arc<Condvar>,
    sendto: Vec<Sender<Box<dyn FnOnce() -> Box<T> + Send>>>,
    threads: Vec<thread::JoinHandle<()>>,
}

// we implement a ThreadPool using standard locking mechanism on the consumer and producer
// 
impl<'pool, T: Send + 'pool> ThreadPool<T> {
    /// Create a new LocalThreadPool with num_threads threads.
    ///
    /// Errors:
    /// - If num_threads is 0, return an error
    pub fn new(num_threads: usize) {
        let exec_queue_arc = Arc::new((Mutex::new(VecDeque::<T>::new()), Condvar::new()));
        let mut sendto: Vec<Sender<Box<dyn FnOnce() -> Box<dyn std::any::Any + Send> + Send>>> = Vec::new();
        let mut threads = Vec::new();
                // Start of Selection
                for i in 0..num_threads {
                    let exec_queue_arc = exec_queue_arc.clone();

                    // Create one of the threads
                    let handle = thread::spawn(move || {
                        // wait for a task to be available
                        let (lock, cvar) = &*exec_queue_arc;
                        let mut queue = lock.lock().unwrap();
                        while queue.is_empty() {
                            queue = cvar.wait(queue).unwrap();
                        }
                        // Execute tasks here
                    });
                    threads.push(handle);
                }
                todo!()
    }
    /// Execute the provided function on the thread pool
    ///
    /// Errors:
    /// - If we fail to send a message, report an error
    pub fn execute<F>(&self, f: F) {
        todo!()
    }
    /// Retrieve any results from the thread pool that have been computed
    pub fn get_results(&self) {
        todo!()
    }
}
