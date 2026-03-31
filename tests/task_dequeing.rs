use robot_coordination_system;

#[cfg(test)]
mod program_tests {
    use robot_coordination_system::enums::robot_status::RobotStatus;
    use robot_coordination_system::health::behaviour::update_health::DEATH_COUNT;
    use robot_coordination_system::health::entity::HEARTBEAT_REGISTRY;
    use robot_coordination_system::robot::behaviour::generate_robots::generate_robots;
    use robot_coordination_system::task::behaviour::generate_tasks::generate_tasks;
    use robot_coordination_system::task::entity::TASK_QUEUE;
    use robot_coordination_system::zone::behaviour::initialize_zones::initialize_zones;
    use robot_coordination_system::zone::entity::ZONE_REGISTRY;

    use super::*;
    use crate::robot_coordination_system::robot::entity::ROBOT_REGISTRY;
    use std::sync::atomic::Ordering;
    fn reset() {
        // Clean up globals before each test
        ZONE_REGISTRY.lock().unwrap().clear();
        TASK_QUEUE.lock().unwrap().clear();
        ROBOT_REGISTRY.lock().unwrap().clear();
        HEARTBEAT_REGISTRY.lock().unwrap().clear();
        DEATH_COUNT.store(0, Ordering::SeqCst);
    }


    /* Test the robot concurrency

        Scenario: System has 5 robot
        The system pushes 10 tasks in random zones
        The robots start working on the tasks in parallel
        Test passes
    */ 
    #[test]
    fn test_robots_grab_tasks() {
        reset();
        initialize_zones(vec!["kitchen", "room 1", "room 2"]);

        // Setup tasks
        generate_tasks(10);

        // create 5 robots to perform tasks
        generate_robots(5);

        let remain_tasks = TASK_QUEUE.lock().unwrap().len();
        let busy_robots = ROBOT_REGISTRY.lock().unwrap()
            .iter()
            .fold(0, |acc, r| if *r.status.lock().unwrap() == RobotStatus::BUSY { acc + 1 } else { acc });

        assert!(busy_robots <= 5);
        assert!(remain_tasks < 10);
    }
}
