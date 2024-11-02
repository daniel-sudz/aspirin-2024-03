use std::mem;
use std::sync::atomic::AtomicBool;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Condvar, Mutex};
use std::collections::{HashMap, VecDeque};

/// 
/// Lifetimes:
/// - 'pool is the lifetime of the thread pool
/// - all tasks are tied to 'pool
/// - all threads are tied to 'pool
/// 
/// Safety: 
/// - Threadpool uses the unsafe method spawn_unchecked to spawn threads
/// - this is ok because threadpool kills all threads when it is dropped
/// - since all tasks and threads are tied to 'pool there is no use after free risk
/// 
pub struct ThreadPool<'pool, T: Send + 'pool> {
    exec_queue_arc: Arc<(Mutex<VecDeque<Task<'pool, T>>>, Condvar)>,
    results_map_arc: Arc<(Mutex<HashMap<usize, T>>, Condvar)>,
    threads: Vec<thread::JoinHandle<()>>,
    active: Arc<AtomicBool>,
    next_task_id: usize
}

struct Task<'pool, T: Send + 'pool> {
    task: Box<dyn FnOnce() -> T + Send + 'pool>,
    task_id: usize,
}

// we implement a ThreadPool using standard locking mechanism on the consumer and producer
// 
impl<'pool, T: Send + 'pool> ThreadPool<'pool, T> {
    /// Create a new ThreadPool with num_threads threads.
    pub fn new(num_threads: usize) -> Self {
        let exec_queue_arc = Arc::new((Mutex::new(VecDeque::<Task<T>>::new()), Condvar::new()));
        let results_map_arc = Arc::new((Mutex::new(HashMap::<usize, T>::new()), Condvar::new()));
    
        let active = Arc::new(AtomicBool::new(true));

        let mut threads = Vec::new();
        for i in 0..num_threads {
            // get a copy of the arcs
            let exec_queue_arc = exec_queue_arc.clone();
            let results_map_arc = results_map_arc.clone();
            let active = active.clone();

            // create one of the threads
            let builder = thread::Builder::new()
                                .name(format!("threadpool-{}", i));
                    
            let handle = unsafe { 
                builder.spawn_unchecked(move || {
                    // threads run while the pool is active
                    while active.load(std::sync::atomic::Ordering::Relaxed) { 
                        let task: Option<Task<'pool, T>> = {
                            // wait for a task to be available by using a condition variable
                            let (lock, cvar) = &*exec_queue_arc;
                            let queue = lock.lock().unwrap();
                            let mut queue = cvar.wait_while(queue, |queue| {
                                active.load(std::sync::atomic::Ordering::Relaxed) && queue.is_empty()
                            }).unwrap();

                            // check if we exited because the threadpool is no longer active
                            match active.load(std::sync::atomic::Ordering::Relaxed) {
                                false => None,
                                true => Some(queue.pop_front().unwrap())
                            }
                        }; 

                        // notify another thread to check for a new task 
                        {
                            let (_, cvar) = &*exec_queue_arc;
                            cvar.notify_one();
                        }

                        // execute the task if the pool is active
                        match task {
                            None => return,
                            Some(task) => {
                                // execute the task
                                let result: T = (task.task)();

                                // store the result in the results map
                                {
                                    let (lock, _) = &*results_map_arc;
                                    let mut results_map = lock.lock().unwrap();
                                    results_map.insert(task.task_id, result);
                                }

                                // notify parent thread if it's waiting for all results to finish
                                {
                                    let (_, cvar) = &*results_map_arc;
                                    cvar.notify_one();
                                }
                           }
                        }
                    }
                    println!("thread exiting");
                }).expect("[threadpool]::fatal os fails to spawn thread")
            };
            threads.push(handle);
        }
        ThreadPool { 
            exec_queue_arc, 
            results_map_arc, 
            threads,
            active,
            next_task_id: 0
        }
    }

    /// Execute the provided function on the thread pool
    ///
    /// Errors:
    /// - If we fail to send a message, report an error
    pub fn execute<F>(&mut self, f: F) -> usize 
    where F: FnOnce() -> T + Send + 'static {
        let task_id = self.next_task_id + 1;
        self.next_task_id = task_id;

        let task = Task { task: Box::new(f), task_id };

        {
            // push the task to the execution queue
            self.exec_queue_arc.0.lock().unwrap().push_back(task);
        }

        // notify a thread in the pool that a task is available
        self.exec_queue_arc.1.notify_one();

        // return the task id so the caller can associate the result with the task
        task_id
    }

    /// Retrieve any results from the thread pool that have been computed
    /// 
    /// Returns:
    /// - A map of task ids to results
    /// 
    /// Mutability: 
    /// - The results map is replaced with an empty map 
    /// - future calls to get_results will not return prior results from get_results
    pub fn get_results(&self) -> HashMap<usize, T> {
        let (lock, _) = &*self.results_map_arc;
        let mut results_map = lock.lock().unwrap();
        let take_results = mem::take(&mut *results_map);
        take_results
    }

    /// Waits for all previously queued tasks to finish execution
    pub fn wait_for_all(&self) {
        let (results_map, cvar) = &*self.results_map_arc;
        let mut results_map = results_map.lock().unwrap();
        let _wait = cvar.wait_while(results_map, |results_map| results_map.len() != self.next_task_id).unwrap();
    }
    
    /// Wait for a specific task to finish execution
    pub fn wait_for_task(&self, task_id: usize) {
        let (results_map, cvar) = &*self.results_map_arc;
        let mut results_map = results_map.lock().unwrap();
        let _wait = cvar.wait_while(results_map, |results_map| !results_map.contains_key(&task_id)).unwrap();
    }

    /// drop the threadpool
    /// 
    /// Safety:
    /// - all threads are joined and all resources are reclaimed
    pub fn drop(self) {
        // set the pool to inactive
        self.active.store(false, std::sync::atomic::Ordering::Relaxed);

        // drop all threads
        for handle in self.threads {
            handle.join().unwrap();
        }
    }
}

 #[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    // basic test that threadpool can execute tasks and return results
    #[test]
    fn test_basic_threadpool() {
        for _ in 0..10 {
            let mut pool: ThreadPool<'_, i32> = ThreadPool::new(4);
            let one = pool.execute(|| 1);
            let two = pool.execute(|| 2);
            let three = pool.execute(|| 3);
            let four = pool.execute(|| 4);
            pool.wait_for_all();
            let results = pool.get_results();
            assert_eq!(results.get(&one).unwrap(), &1);
            assert_eq!(results.get(&two).unwrap(), &2);
            assert_eq!(results.get(&three).unwrap(), &3);
            assert_eq!(results.get(&four).unwrap(), &4);
        }
    }

    // variable timing test that threadpool can execute tasks and return results
    #[test]
    fn test_variable_timing_threadpool() {
        for _ in 0..10 {
            let mut pool: ThreadPool<'_, i32> = ThreadPool::new(4);
            let one = pool.execute(|| {
                thread::sleep(Duration::from_millis(rand::random::<u64>() % 100));
                1
            });
            let two = pool.execute(|| {
                thread::sleep(Duration::from_millis(rand::random::<u64>() % 100));
                2
            });
            let three = pool.execute(|| {
                thread::sleep(Duration::from_millis(rand::random::<u64>() % 100));
                3
            });
            let four = pool.execute(|| {
                thread::sleep(Duration::from_millis(rand::random::<u64>() % 100));
                4
            });
            pool.wait_for_all();
            let results = pool.get_results();
            assert_eq!(results.get(&one).unwrap(), &1);
            assert_eq!(results.get(&two).unwrap(), &2);
            assert_eq!(results.get(&three).unwrap(), &3);
            assert_eq!(results.get(&four).unwrap(), &4);
        }
    }

    // test that threadpool can wait for a specific task to finish execution
    #[test]
    fn test_wait_for_task() {
        let mut pool: ThreadPool<'_, i32> = ThreadPool::new(4);
        let task_id = pool.execute(|| 1);
        pool.wait_for_task(task_id);
        let results = pool.get_results();
        assert_eq!(results.get(&task_id).unwrap(), &1);
    }

}
