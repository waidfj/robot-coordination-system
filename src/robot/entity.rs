use std::{sync::Mutex, thread, time::Duration};
use crate::{enums::robot_status::RobotStatus, task::entity::TASK_QUEUE};

// The main interface of the Robot entity
pub struct Robot {
    id: u32,
    pub(crate) name: String,
    pub(crate) status: Mutex<RobotStatus>,
    pub current_task_id: Mutex<Option<u32>>,
}

impl Robot {
    // The defualt constructor of the Robot entity
    pub fn new (id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            status: RobotStatus::IDLE.into(),
            current_task_id: Mutex::new(None),
        }
    }

    //////////////////////////////////////////////////
    // TODO: implement hearbeat logic for the robot //
    //////////////////////////////////////////////////
    pub fn update_heartbeat(&self) {
        println!("Robot {} (ID: {}) is pulsing...", self.name, self.id);
    }

    // Robot takes a task and executes it
    pub fn take_task (&self) {
        // Dequeing a task to execute
        let task = {
            let mut queue = TASK_QUEUE.lock().unwrap();
            queue.pop_front() // dequeue an element and return 
            // The lock is automatically released here because 'queue' goes out of scope
        };

        // If there is a task, update status => execute task => update status again
        if let Some(t) = task {
            // Mark as busy
            {
                // Update status to busy
                let mut s = self.status.lock().unwrap();
                *s = RobotStatus::BUSY;

                // Store id of task being executed (for task display)
                let mut t_id = self.current_task_id.lock().unwrap();
                *t_id = Some(t.id);
            }

            // Execute the task
                // Note: this is only a mock simulation of the real execution
            thread::sleep(Duration::from_secs(t.get_duration()));

            // Mark as Idle
            {
                // Update status to idle
                let mut s = self.status.lock().unwrap();
                *s = RobotStatus::IDLE;

                // Empty the content of the task being executed
                let mut t_id = self.current_task_id.lock().unwrap();
                *t_id = None;
            }
        }
    }
}
