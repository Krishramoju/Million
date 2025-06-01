// neurokernel/src/scheduler.rs

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::time::{Duration, Instant};

/// Task struct representing a scheduled task
#[derive(Eq, PartialEq)]
pub struct Task {
    pub id: usize,
    pub priority: usize,
    pub scheduled_time: Instant,
    pub task_fn: Box<dyn FnOnce() + Send + 'static>,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority tasks come first; if same priority, earlier time comes first
        other.priority.cmp(&self.priority)
            .then_with(|| self.scheduled_time.cmp(&other.scheduled_time))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Scheduler manages task queue and executes tasks based on priority and scheduled time.
pub struct Scheduler {
    task_queue: BinaryHeap<Task>,
    task_map: HashMap<usize, Task>,
    next_id: usize,
}

impl Scheduler {
    /// Creates a new empty Scheduler
    pub fn new() -> Self {
        Scheduler {
            task_queue: BinaryHeap::new(),
            task_map: HashMap::new(),
            next_id: 0,
        }
    }

    /// Schedule a new task to run after `delay` with given priority.
    /// Returns task id.
    pub fn schedule<F>(&mut self, delay: Duration, priority: usize, task_fn: F) -> usize
    where
        F: FnOnce() + Send + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;

        let task = Task {
            id,
            priority,
            scheduled_time: Instant::now() + delay,
            task_fn: Box::new(task_fn),
        };
        self.task_queue.push(task);
        id
    }

    /// Run all tasks that are ready to be executed (scheduled_time <= now)
    pub fn run_ready_tasks(&mut self) {
        let now = Instant::now();
        while let Some(task) = self.task_queue.peek() {
            if task.scheduled_time <= now {
                let task = self.task_queue.pop().unwrap();
                (task.task_fn)();
            } else {
                break;
            }
        }
    }

    /// Cancel a scheduled task by its id
    pub fn cancel(&mut self, task_id: usize) -> bool {
        // Removing from BinaryHeap is not trivial, so this is a stub.
        // Could implement with a separate cancelled set or use different data structure.
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_scheduler_runs_task() {
        let mut scheduler = Scheduler::new();
        let flag = Arc::new(Mutex::new(false));
        let flag_clone = flag.clone();

        scheduler.schedule(Duration::from_millis(0), 10, move || {
            let mut val = flag_clone.lock().unwrap();
            *val = true;
        });

        scheduler.run_ready_tasks();
        assert_eq!(*flag.lock().unwrap(), true);
    }

    #[test]
    fn test_scheduler_priority_order() {
        let mut scheduler = Scheduler::new();
        let mut results = Vec::new();

        scheduler.schedule(Duration::from_millis(0), 5, || {
            println!("Task with priority 5");
        });
        scheduler.schedule(Duration::from_millis(0), 10, || {
            println!("Task with priority 10");
        });

        // Just check priority ordering by popping from the heap
        let task1 = scheduler.task_queue.pop().unwrap();
        let task2 = scheduler.task_queue.pop().unwrap();
        assert!(task1.priority > task2.priority);
    }
}
