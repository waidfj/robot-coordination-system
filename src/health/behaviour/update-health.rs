use crate::{
    enums::robot_status::RobotStatus, health::entity::HEARTBEAT_REGISTRY,
    robot::entity::ROBOT_REGISTRY,
};
use std::time::{Duration, Instant};

// This function monitors the latest activity of robots and marks abscent ones as offlines
pub fn update_health() {
    // Clone the registry for robots to record their latest alive moment
    //  (to keep the original one avilable for others to use)
    let robots_health = HEARTBEAT_REGISTRY.lock().unwrap().clone();

    // Loop on the robots
    for robot_health in robots_health {
        if robot_health.1 < Instant::now() - Duration::from_secs(5) {
            // Extract the abscent robot and mark it as offline
            let robots = ROBOT_REGISTRY.lock().unwrap().clone();
            let robot = &robots[(robot_health.0 - 1) as usize];
            *robot.status.lock().unwrap() = RobotStatus::OFFLINE;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        enums::robot_status::RobotStatus,
        health::entity::HEARTBEAT_REGISTRY,
        robot::entity::{Robot, ROBOT_REGISTRY},
    };
    use std::{sync::Arc, time::{Duration, Instant}};

    fn reset_globals() {
        ROBOT_REGISTRY.lock().unwrap().clear();
        HEARTBEAT_REGISTRY.lock().unwrap().clear();
    }

    #[test]
    fn update_health_marks_stale_robot_offline() {
        reset_globals();

        let robot = Arc::new(Robot::new(1, "robot-1"));
        ROBOT_REGISTRY.lock().unwrap().push(Arc::clone(&robot));

        let stale_time = Instant::now() - Duration::from_secs(10);
        HEARTBEAT_REGISTRY.lock().unwrap().insert(1, stale_time);

        update_health();

        assert_eq!(*robot.status.lock().unwrap(), RobotStatus::OFFLINE);
    }
}