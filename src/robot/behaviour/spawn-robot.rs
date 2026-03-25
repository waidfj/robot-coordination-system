use crate::robot::entity::Robot;
use std::{sync::Arc, thread, time::Duration};

// create a thread to run the passed robot
pub fn spawn_robot(robot: Arc<Robot>) {
    // the main thread for the robot functionality
    thread::spawn(move || {
       let robot_clone = Arc::clone(&robot);     
       /* if robot_clone.id == 1    //Thread time out test!
         {
        thread::sleep(Duration::from_secs(6)); // Pause 6s to trigger OFFLINE
        }*/

        ////////////////////////////////////////////////////////////////
        // TODO: Implement dying logic for the robots, use batteries //
        ///////////////////////////////////////////////////////////////
        // a child process that will update the heartbeat of the robot
        thread::spawn(move || {
            loop {
                robot_clone.send_heartbeat();
                thread::sleep(Duration::from_secs(2)); // Send heartbeat every 2 seconds
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
