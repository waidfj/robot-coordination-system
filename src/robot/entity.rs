use crate::{
    enums::robot_status::RobotStatus,
    health::entity::HEARTBEAT_REGISTRY,
    task::entity::{TASK_QUEUE, Task},
    zone::behaviour::get_zone::get_zone,
};
use std::{sync::{Mutex, atomic::{AtomicU32, Ordering}}, thread, time::Duration};
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
    pub battery: AtomicU32
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
            battery: 100.into()
        }
    }

    // Robot updates it's activity
    pub fn send_heartbeat(&self) {
        // lock the heartbeat registry and update time
        if let Ok(mut map) = HEARTBEAT_REGISTRY.try_lock() {
            map.insert(self.id, Instant::now());
        }
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
        // Get zone and acquire lock (this is where waiting happens)
        let zone = get_zone(task.get_zone_id());
        let _zone_guard = zone.lock.lock().unwrap();

        // Mark as busy
        {
            *self.status.lock().unwrap() = RobotStatus::BUSY;
            *self.current_task_id.lock().unwrap() = Some(task.id);
            *self.current_zone_id.lock().unwrap() = Some(task.get_zone_id());
        }

        // === BATTERY LOOP (your original code) ===
        for _i in 1..=task.get_duration() {
            let battery = self.battery.load(Ordering::SeqCst);
            if battery != 0 {
                self.set_battery(battery - 10);
                thread::sleep(Duration::from_secs(1));
            } else {
                return;
            }
        }

        // Mark as Idle + recharge
        {
            *self.status.lock().unwrap() = RobotStatus::IDLE;
            *self.current_task_id.lock().unwrap() = None;
            *self.current_zone_id.lock().unwrap() = None;
        }

        if self.battery.load(Ordering::SeqCst) == 0 {
            self.battery.store(100, Ordering::SeqCst);
        }
    }

    // Set the batter to a new value
    pub fn set_battery(&self, battery: u32) {
        self.battery.store(battery, Ordering::SeqCst);
    }
    
}
