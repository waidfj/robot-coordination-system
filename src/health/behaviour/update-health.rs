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
                        // println!("Robot no. {} is marked offline (No heartbeat for 10s)", id);
                    }

                    // *robot.status.lock().unwrap() = RobotStatus::OFFLINE;
                    // DEATH_COUNT.fetch_add(1, Ordering::SeqCst);
                }
            }
        }

    });

}


#[cfg(test)]
mod program_tests {
    use super::*;
    use crate::robot::entity::{Robot, ROBOT_REGISTRY};
    use std::sync::Arc;
    use std::time::{Duration, Instant};
    fn Reset() {
        // Clean up globals before each test
        ROBOT_REGISTRY.lock().unwrap().clear();
        HEARTBEAT_REGISTRY.lock().unwrap().clear();
        DEATH_COUNT.store(0, Ordering::SeqCst);
    }

    #[test]
    fn robots_status_test() {   //Battery Reaches 0 And robot Show offline
        Reset();
        let robot = Arc::new(Robot::new(999, "TestRobot"));
        ROBOT_REGISTRY.lock().unwrap().push(Arc::clone(&robot));

        // Start the health monitor
        update_health();

        // Give the monitor a moment to start its loop
        std::thread::sleep(Duration::from_secs(1));

        //Force a stale heartbeat 
        {
            let mut map = HEARTBEAT_REGISTRY.lock().unwrap();
            map.insert(999, Instant::now() - Duration::from_secs(15)); // 15s ago, which is beyond the 10s timeout
        }

        // Force battery to zero (so heartbeat thread stops)
        robot.set_battery(0);

        // Wait for the monitoring loop to detect and mark OFFLINE
        std::thread::sleep(Duration::from_secs(13));

        let status = robot.status.lock().unwrap();
        assert_eq!(*status, RobotStatus::OFFLINE, 
                   "Robot should be marked OFFLINE when battery reaches zero");

        assert!(DEATH_COUNT.load(Ordering::SeqCst) >= 1,
                "DEATH_COUNT should have increased");
    }
}