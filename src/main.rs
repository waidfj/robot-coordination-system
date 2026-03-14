mod task;

use std::sync::{Arc, Mutex};
use std::collections::BinaryHeap;
use task::{Task, start_task_generator};

fn main() {
    let task_queue: Arc<Mutex<BinaryHeap<Task>>> = Arc::new(Mutex::new(BinaryHeap::new()));
    let task_generator =start_task_generator(task_queue.clone());
    println!("📋 Task generator started! Will create tasks every 5 seconds...");
    loop{
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

}