mod enums;
mod robot;
mod task;

use crate::task::behaviour::generate_tasks::generate_tasks;
use robot::{entity::Robot, behaviour::spawn_robot::spawn_robot};
use std::{sync::Arc, thread, time::Duration};

fn main() {
    generate_tasks(10);

    ///////////////////////////////////////////////////
    // TODO: create a function that generates robots //
    ///////////////////////////////////////////////////
    let mut fleet = Vec::new();

    // Create a few robots
    for i in 1..=3 {
        let robot_instance = Robot::new(i, &format!("Robot {}", i));

        // Wrap it in an Arc so both 'main' and the thread can own it
        let shared_robot = Arc::new(robot_instance);

        // Put a "copy of the key" into our fleet list for monitoring
        fleet.push(Arc::clone(&shared_robot));

        // Send the robot off to work
        spawn_robot(shared_robot);
    }

    // IMPORTANT DECLARATION: this code is AI generated
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen
        println!(
            "{:<10} | {:<10} | {:<15}",
            "Robot", "Status", "Current Task"
        );
        println!("----------------------------------------------");

        for bot in &fleet {
            let s = bot.status.lock().unwrap();
            let t_id = bot.current_task_id.lock().unwrap();

            // Format the display
            let task_display = match *t_id {
                Some(id) => format!("Task #{}", id),
                None => "---".to_string(),
            };

            println!("{:<10} | {:<10?} | {:<15}", bot.name, *s, task_display);
        }
        thread::sleep(Duration::from_millis(500));
    }
}
