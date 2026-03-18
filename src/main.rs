mod enums;
mod robot;
mod task;
mod zone;
mod health;


use crate::{
    task::behaviour::generate_tasks::generate_tasks,
    zone::{
        behaviour::{get_zone::get_zone, initialize_zones::initialize_zones},
        entity::ZONE_REGISTRY,
    },
    health::{new_heartbeat_registry, register_robot, monitor_heartbeats},
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

    let heartbeat_registry = new_heartbeat_registry();


    // Create a few robots
    for i in 1..=3 {
        let robot_instance = Robot::new(i, &format!("Robot {}", i));

        // Register the robot in the heartbeat system
        register_robot(&heartbeat_registry, i);

        // Wrap it in an Arc so both 'main' and the thread can own it
        let shared_robot = Arc::new(robot_instance);

        // Put a "copy of the key" into our fleet list for monitoring
        fleet.push(Arc::clone(&shared_robot));

        // Send the robot off to work
        spawn_robot(shared_robot, Arc::clone(&heartbeat_registry));
    }
    // Start the heartbeat monitor thread
    monitor_heartbeats(fleet.clone(), heartbeat_registry);

     // Main loop to display status of robots and zones

    // IMPORTANT DECLARATION: this code is AI generated
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("------------------------------------------------------------");
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
        println!("------------------------------------------------");
        thread::sleep(Duration::from_millis(500));
    }
}
