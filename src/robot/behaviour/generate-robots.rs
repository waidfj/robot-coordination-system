use std::sync::Arc;

use crate::robot::{
    behaviour::spawn_robot::spawn_robot,
    entity::{ROBOT_REGISTRY, Robot},
};

// Robot factory, runs before the system starts
pub fn generate_robots(quantity: u32) {
    for i in 1..=quantity {
        let robot_instance = Arc::new(Robot::new(i, &format!("Robot {}", i)));

        // Send the robot off to work
        //  cloned because to avoid race conditions (it will be accessed by multiple threads simultaneously)
        spawn_robot(Arc::clone(&robot_instance));

        // Keep the robots in the registry for monitoring purposes
        ROBOT_REGISTRY.lock().unwrap().push(robot_instance);
    }
}
