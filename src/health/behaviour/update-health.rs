use crate::{
    enums::robot_status::RobotStatus, health::entity::HEARTBEAT_REGISTRY,
    robot::entity::ROBOT_REGISTRY,
};
use std::{sync::atomic::{AtomicU32, Ordering}, thread, time::{Duration, Instant}};

pub static DEATH_COUNT: AtomicU32 = AtomicU32::new(0);


// This function monitors the latest activity of robots and marks abscent ones as offlines
pub fn update_health() {
    thread::spawn(move || loop {

        let now = Instant::now();
        let timeout = Duration::from_secs(10);

        let dead_ids: Vec<u32> = {
            let mut map = HEARTBEAT_REGISTRY.lock().unwrap();
            let mut detected = Vec::new();
            
            // Use retain to remove them so we don't check them again!
            map.retain(|&id, &mut last_seen| {
                if now.duration_since(last_seen) > timeout {
                    detected.push(id);
                    false // This REMOVES the entry from the HashMap
                } else {
                    true  // This KEEPS the entry
                }
            });
            detected
        };

        if !dead_ids.is_empty() {
            let robots = ROBOT_REGISTRY.lock().unwrap();
            for id in dead_ids {
                // Find the specific robot in the registry by ID
                if let Some(robot) = robots.iter().find(|r| r.id == id) {
                    let mut status = robot.status.lock().unwrap();
                    
                    // // Only update and print if it's not already offline
                    if *status != RobotStatus::OFFLINE {
                        *status = RobotStatus::OFFLINE;
                        DEATH_COUNT.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        }

    });
}
