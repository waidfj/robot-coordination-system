use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use crate::robot::entity::Robot;

pub type HeartbeatRegistry = Arc<RwLock<HashMap<u32, bool>>>;

pub fn new_heartbeat_registry() -> HeartbeatRegistry {
    Arc::new(RwLock::new(HashMap::new()))
}

pub fn register_robot(registry: &HeartbeatRegistry, robot_id: u32) {
    if let Ok(mut map) = registry.write() {
        map.insert(robot_id, false);
    }
}


pub fn is_robot_alive(registry: &HeartbeatRegistry, robot_id: u32) -> bool {
    if let Ok(map) = registry.read() {
        map.get(&robot_id).copied().unwrap_or(false)
    } else {
        false
    }
}

/// Resets the heartbeat flag for a robot after checking it
/// This allows the next check cycle to properly detect if the robot is still alive
pub fn reset_heartbeat_flag(registry: &HeartbeatRegistry, robot_id: u32) {
    if let Ok(mut map) = registry.write() {
        if let Some(flag) = map.get_mut(&robot_id) {
            *flag = false;
        }
    }
}


pub fn monitor_heartbeats(fleet: Vec<Arc<Robot>>, registry: HeartbeatRegistry) {
    thread::spawn(move || {
        loop {
            // Wait 4 seconds before checking 
            thread::sleep(Duration::from_secs(4));

            // Check each robot's heartbeat
            for robot in &fleet {
                let is_alive = is_robot_alive(&registry, robot.id);

                if is_alive {

                    println!("{} is alive (heartbeat received)", robot.name);
                    // Robot is alive - reset flag for next cycle
                    reset_heartbeat_flag(&registry, robot.id);
                  
                } else {
                    println!("{} is OFFLINE (no heartbeat)", robot.name);
            
                }
            }
        }
    });
}