use robot_coordination_system;

#[cfg(test)]
mod program_tests {
    use robot_coordination_system::enums::robot_status::RobotStatus;
    use robot_coordination_system::health::behaviour::update_health::{DEATH_COUNT, update_health};
    use robot_coordination_system::health::entity::HEARTBEAT_REGISTRY;
    use robot_coordination_system::task::entity::TASK_QUEUE;
    use robot_coordination_system::zone::entity::ZONE_REGISTRY;

    use super::*;
    use crate::robot_coordination_system::robot::entity::{Robot, ROBOT_REGISTRY};
    use std::sync::Arc;
    use std::sync::atomic::Ordering;
    use std::time::{Duration, Instant};
    fn reset() {
        // Clean up globals before each test
        ZONE_REGISTRY.lock().unwrap().clear();
        TASK_QUEUE.lock().unwrap().clear();
        ROBOT_REGISTRY.lock().unwrap().clear();
        HEARTBEAT_REGISTRY.lock().unwrap().clear();
        DEATH_COUNT.store(0, Ordering::SeqCst);
    }

    /* Test the health monitor functionality

        Scenario: System has one robot
        The robot is assigned a 15 second task
        After 10 seconds the robot's battery dies
        Health monitor detects it and updates its status to offline
        Test passes if robot is offline within 13 seconds
    */ 
    #[test]
    fn robots_status_test() {   //Battery Reaches 0 And robot Show offline
        reset();
        let robot = Arc::new(Robot::new(999, "TestRobot"));
        ROBOT_REGISTRY.lock().unwrap().push(Arc::clone(&robot));
        println!("Number of robots: {}", ROBOT_REGISTRY.lock().unwrap().len());

        // Start the health monitor
        update_health();

        // Give the monitor a moment to start its loop
        std::thread::sleep(Duration::from_secs(1));

        // Force a stale heartbeat 
        {
            let mut map = HEARTBEAT_REGISTRY.lock().unwrap();
            map.insert(999, Instant::now() - Duration::from_secs(15)); // 15s ago, which is beyond the 10s timeout
        }

        // Wait for the monitoring loop to detect and mark OFFLINE
        std::thread::sleep(Duration::from_secs(13));

        let status = robot.status.lock().unwrap();
        assert_eq!(*status, RobotStatus::OFFLINE, 
                   "Robot should be marked OFFLINE when battery reaches zero");

        assert!(DEATH_COUNT.load(Ordering::SeqCst) >= 1,
                "DEATH_COUNT should have increased");
    }
}
