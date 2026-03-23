mod enums;
mod health;
mod robot;
mod task;
mod zone;

use crate::{
    health::behaviour::update_health::update_health,
    robot::{
        behaviour::generate_robots::generate_robots,
        entity::{ROBOT_REGISTRY, Robot},
    },
    task::behaviour::generate_tasks::generate_tasks,
    zone::{
        behaviour::{get_zone::get_zone, initialize_zones::initialize_zones},
        entity::ZONE_REGISTRY,
    },
};
use std::{sync::Arc, thread, time::Duration};

fn main() {
    // Initialize zones, generate tasks and robots
    initialize_zones(["kitchen", "room 1", "room 2"].to_vec());
    generate_tasks(10);
    generate_robots(3);

    // Main loop to display status of robots and zones
    // IMPORTANT DECLARATION: this code is AI generated
    loop {
        thread::spawn(move || {
            update_health();
        });

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("------------------------------------------------------------");
        println!(
            "{:<10} | {:<10} | {:<10} | {:<15}",
            "Robot", "Status", "Task", "Location"
        );
        println!("------------------------------------------------------------");

        let robots: Vec<Arc<Robot>> = ROBOT_REGISTRY.lock().unwrap().clone();
        for bot in robots {
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
