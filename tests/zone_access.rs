use robot_coordination_system;

#[cfg(test)]
mod program_tests {
    use robot_coordination_system::health::behaviour::update_health::DEATH_COUNT;
    use robot_coordination_system::health::entity::HEARTBEAT_REGISTRY;
    use robot_coordination_system::robot::behaviour::generate_robots::generate_robots;
    use robot_coordination_system::task::entity::{TASK_QUEUE, Task};
    use robot_coordination_system::zone::behaviour::initialize_zones::initialize_zones;
    use robot_coordination_system::zone::entity::ZONE_REGISTRY;

    use super::*;
    use crate::robot_coordination_system::robot::entity::ROBOT_REGISTRY;
    use std::sync::atomic::Ordering;
    use std::thread;
    use std::time::Duration;
    fn reset() {
        // Clean up globals before each test
        ZONE_REGISTRY.lock().unwrap().clear();
        TASK_QUEUE.lock().unwrap().clear();
        ROBOT_REGISTRY.lock().unwrap().clear();
        HEARTBEAT_REGISTRY.lock().unwrap().clear();
        DEATH_COUNT.store(0, Ordering::SeqCst);
    }

    /* Test zone access safety

        Scenario: System has two robot
        Two tasks are pushed to the queue, both assigned in the same zone
        Robots pop one task each
        one starts working (BUSY) while the other is blocked (IDLE)
        Their statuses don't match, test passes
    */ 
    #[test]
    fn safe_access_to_shared_zone(){
        reset();
        initialize_zones(vec!["kitchen", "room 1", "room 2"]);

        // generate two tasks that run in the same zone
        {
            let mut task_queue = TASK_QUEUE.lock().unwrap();
            task_queue.push_back(Task::new(1, 5, 1));
            task_queue.push_back(Task::new(2, 5, 1));
        }

        // generate robots
        generate_robots(2);

        // wait for robots to be created and take tasks
        thread::sleep(Duration::from_secs(1));
        
        let robot1_status = ROBOT_REGISTRY.lock().unwrap()[0].status.lock().unwrap().clone();
        let robot2_status = ROBOT_REGISTRY.lock().unwrap()[1].status.lock().unwrap().clone();
        assert!(robot1_status != robot2_status);
    }
}
