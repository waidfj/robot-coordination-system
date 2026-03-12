use rand::Rng;
use std::{thread, time::Duration};

use crate::{
    task::entity::{TASK_QUEUE, Task},
    zone::behaviour::get_zones_count::get_zones_count,
};

// This is only a mock of the guy who creates tasks for the robots
// CAUTION: it is not related to the functionality of the application but used only for test purposes
pub fn generate_tasks(number_of_tasks: u32) {
    // Create a thread for generating the tasks
    thread::spawn(move || {
        let mut count = 0;
        let mut rng = rand::thread_rng();

        loop {
            if count >= number_of_tasks {
                break;
            }
            count += 1;
            let new_task = Task::new(count, 2, rng.gen_range(0..=(get_zones_count() - 1)));

            // Adding the task to the task queue
            // Scope the lock so it is released immediately after pushing
            {
                let mut tasl_queue = TASK_QUEUE.lock().unwrap();
                tasl_queue.push_back(new_task);
            }

            // This allows the robots to keep up (gives some reality for the simulation)
            thread::sleep(Duration::from_millis(200));
        }
    });
}
