use crate::robot::entity::Robot;
use crate::health::HeartbeatRegistry;
use std::{sync::Arc, thread, time::Duration};

// create a thread to run the passed robot
pub fn spawn_robot(robot: Arc<Robot>, heartbeat_registry: HeartbeatRegistry) {
    // the main thread for the robot functionality
    thread::spawn(move || {
        let reg_clone = Arc::clone(&heartbeat_registry);
        let robot_clone = Arc::clone(&robot);

        // a child process that will update the heartbeat of the robot
        thread::spawn(move || {
            loop {
                robot_clone.send_heartbeat(&reg_clone);
                thread::sleep(Duration::from_secs(3)); // Send heartbeat every 2 seconds
            }
        });

        // the main functionality run by the robot
        loop {
            // Take a task
            if let Some(task) = robot.take_task() {
                // Execute the task (if exists)
                robot.execute_task(task);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}
