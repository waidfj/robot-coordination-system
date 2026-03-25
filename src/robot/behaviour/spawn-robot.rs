use crate::robot::entity::Robot;
use std::{sync::{Arc, atomic::Ordering, mpsc}, thread, time::Duration};

// create a thread to run the passed robot
pub fn spawn_robot(robot: Arc<Robot>) {
    // the main thread for the robot functionality
    thread::spawn(move || {
        let robot_clone = Arc::clone(&robot);
        let (tx, rx) = mpsc::channel();

        // a child process that will update the heartbeat of the robot
        thread::spawn(move || {
            loop {
                let current_battery = robot_clone.battery.load(Ordering::SeqCst);

                if current_battery != 0 {
                    robot_clone.send_heartbeat();
                    thread::sleep(Duration::from_secs(2)); // Send heartbeat every 2 seconds
                } else {
                    let _ = tx.send("SHUTDOWN");
                    // println!("Robot no. {} stopped beating", robot_clone.id);
                    break;
                }

            }
        });

        // the main functionality run by the robot
        loop {
            if let Ok(msg) = rx.try_recv() {
                if msg == "SHUTDOWN" {
                    // println!("Robot no. {} recieved signal and will shut down 1", robot.id);
                    break;
                }
            }
            // Take a task
            if let Some(task) = robot.take_task() {
                // println!("Robot no: {}, took task no: {}", robot.id, task.id);

                if let Ok(msg) = rx.try_recv() {
                    if msg == "SHUTDOWN" {
                    // println!("Robot no. {} recieved signal and will shut down 2", robot.id);
                        break;
                    }
                }
                // Execute the task (if exists)
                robot.execute_task(task);

                if let Ok(msg) = rx.try_recv() {
                    if msg == "SHUTDOWN" {
                    // println!("Robot no. {} recieved signal and will shut down 3", robot.id);
                        break;
                    }
                }

                if robot.battery.load(Ordering::SeqCst) != 0 {
                    robot.battery.store(100, Ordering::SeqCst);
                } else {
                    break;
                }
                // println!("Robot no: {}, has status of: {:?}", robot.id, *robot.status.lock().unwrap());
            }
        }
    });
}
