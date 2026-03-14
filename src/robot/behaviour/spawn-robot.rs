use crate::robot::entity::Robot;
use std::{sync::Arc, thread, time::Duration};

// create a thread to run the passed robot
pub fn spawn_robot(robot: Arc<Robot>) {
    // the main thread for the robot functionality
    thread::spawn(move || {
        let heartbeat_robot = Arc::clone(&robot);

        // a child process that will update the heartbeat of the robot
        thread::spawn(move || {
            loop {
                heartbeat_robot.update_heartbeat();
                thread::sleep(Duration::from_secs(1));
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
