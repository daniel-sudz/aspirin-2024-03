use std::thread::{self, JoinHandle};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Condvar, Mutex};
use std::collections::{HashMap, VecDeque};

pub struct ThreadPool<T: Send> {
    condition_queue: Arc<Condvar>,
    sendto: Vec<Sender<Box<dyn FnOnce() -> Box<T> + Send>>>,
    threads: Vec<thread::JoinHandle<()>>,
}

struct Task<T: Send> {
    task: Box<dyn FnOnce() -> T + Send>,
    task_id: usize,
}

// we implement a ThreadPool using standard locking mechanism on the consumer and producer
// 
impl<'pool, T: Send + 'pool> ThreadPool<T> {
    /// Create a new LocalThreadPool with num_threads threads.
    ///
    pub fn new(num_threads: usize) {
        let exec_queue_arc = Arc::new((Mutex::new(VecDeque::<Task<T>>::new()), Condvar::new()));
        let results_map_arc = Arc::new(Mutex::new(HashMap::<usize, T>::new()));

        let mut threads = Vec::new();
                // Start of Selection
                for i in 0..num_threads {
                    // get a copy of the arcs
                    let exec_queue_arc = exec_queue_arc.clone();
                    let results_map_arc = results_map_arc.clone();

                    // create one of the threads
                    let builder = thread::Builder::new()
                                .name(format!("threadpool-{}", i));
                    
                    let handle = unsafe { 
                        builder.spawn_unchecked(move || {
                        
                        let task = {
                            // wait for a task to be available by using a condition variable
                            let (lock, cvar) = &*exec_queue_arc;
                            let queue = lock.lock().unwrap();
                            let mut queue = cvar.wait_while(queue, |queue| queue.is_empty()).unwrap();

                            // task is now available, take it and execute it
                            queue.pop_front().unwrap()
                        };

                        // execute the task
                        let result: T = (task.task)();

                        // store the result in the results map
                        let mut results_map = results_map_arc.lock().unwrap();
                        results_map.insert(task.task_id, result);

                        }).expect("[threadpool]::fatal os fails to spawn thread")
                    };
                    threads.push(handle);
                }
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
