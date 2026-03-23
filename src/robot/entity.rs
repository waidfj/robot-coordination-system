use crate::{
    enums::robot_status::RobotStatus,
    health::entity::HEARTBEAT_REGISTRY,
    task::entity::{TASK_QUEUE, Task},
    zone::behaviour::get_zone::get_zone,
};
use std::{sync::Mutex, thread, time::Duration};
use std::{
    sync::{Arc, LazyLock},
    time::Instant,
};

// The global registry that will hold all the robots in the system
pub static ROBOT_REGISTRY: LazyLock<Mutex<Vec<Arc<Robot>>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

// The main interface of the Robot entity
pub struct Robot {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) status: Mutex<RobotStatus>,
    pub current_task_id: Mutex<Option<u32>>,
    pub current_zone_id: Mutex<Option<u32>>,
}

impl Robot {
    // The defualt constructor of the Robot entity
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            status: RobotStatus::IDLE.into(),
            current_task_id: Mutex::new(None),
            current_zone_id: Mutex::new(None),
        }
    }

    // Robot updates it's activity
    pub fn send_heartbeat(&self) {
        {
            // lock the heartbeat registry and update time
            let mut map = HEARTBEAT_REGISTRY.lock().unwrap();
            map.insert(self.id, Instant::now());
        }

        thread::sleep(Duration::from_secs(2));
    }

    // Robot attempts to take a task
    pub fn take_task(&self) -> Option<Task> {
        // Dequeing a task
        let mut queue = TASK_QUEUE.lock().unwrap();

        queue.pop_front() // dequeue an element and return 
        // The lock is automatically released here because 'queue' goes out of scope
    }

    // Robot executes the task
    pub fn execute_task(&self, task: Task) {
        // Lock zone to prevent other robots from using it
        let zone = get_zone(task.get_zone_id());
        let _zone_guard = zone.lock.lock().unwrap();

        // Mark as busy
        {
            // Scope locks to avoide race conditions
            // Update status to busy
            *self.status.lock().unwrap() = RobotStatus::BUSY;

            // Store id of task being executed and zone occupied (for display)
            *self.current_task_id.lock().unwrap() = Some(task.id);
            *self.current_zone_id.lock().unwrap() = Some(task.get_zone_id());
        }

        // Execute the task
        // Note: this is a mock simulation of the real execution
        thread::sleep(Duration::from_secs(task.get_duration()));

        // Mark as Idle
        {
            // Scope locks to avoide race conditions
            // Update status to idle
            *self.status.lock().unwrap() = RobotStatus::IDLE;

            // Empty the content of the task being executed and zone occupied
            *self.current_task_id.lock().unwrap() = None;
            *self.current_zone_id.lock().unwrap() = None;
        }
    }
}
