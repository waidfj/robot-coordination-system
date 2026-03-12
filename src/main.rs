mod enums;
mod robot;
mod task;
mod zone;

use crate::{
    task::behaviour::generate_tasks::generate_tasks,
    zone::{
        behaviour::{get_zone::get_zone, initialize_zones::initialize_zones},
        entity::ZONE_REGISTRY,
    },
};
use robot::{behaviour::spawn_robot::spawn_robot, entity::Robot};
use std::{sync::Arc, thread, time::Duration};

fn main() {
    // Initialize zones and generate tasks
    initialize_zones(["kitchen", "room 1", "room 2"].to_vec());
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
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!(
            "{:<10} | {:<10} | {:<10} | {:<15}",
            "Robot", "Status", "Task", "Location"
        );
        println!("------------------------------------------------------------");

        for bot in &fleet {
            let s = bot.status.lock().unwrap();
            let t_id = bot.current_task_id.lock().unwrap();
            let z_id = bot.current_zone_id.lock().unwrap();

            let task_display = t_id.map(|id| format!("#{}", id)).unwrap_or("---".into());

            // Use your get_zone function to turn the ID into a Name
            let location_display = if let Some(id) = *z_id {
                get_zone(id).name.clone()
            } else {
                "---".to_string()
            };

            println!(
                "{:<10} | {:<10?} | {:<10} | {:<15}",
                bot.name, *s, task_display, location_display
            );
        }
        println!("\n--- Zone Status ---");
        if let Ok(registry) = ZONE_REGISTRY.lock() {
            // We iterate directly over the values in the map
            for zone in registry.values() {
                let status = match zone.lock.try_lock() {
                    Ok(_) => "Empty",
                    Err(_) => "OCCUPIED ⚠️",
                };

                println!("[ID: {:<2}] {:<12}: {}", zone.id, zone.name, status);
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
}
