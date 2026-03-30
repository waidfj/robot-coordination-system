use robot_coordination_system;

#[cfg(test)]
mod program_tests {
    use robot_coordination_system::enums::robot_status::RobotStatus;
    use robot_coordination_system::health::behaviour::update_health::{DEATH_COUNT, update_health};
    use robot_coordination_system::health::entity::HEARTBEAT_REGISTRY;
    use robot_coordination_system::robot::behaviour::generate_robots::generate_robots;
    use robot_coordination_system::task::behaviour::generate_tasks::generate_tasks;
    use robot_coordination_system::task::entity::{TASK_QUEUE, Task};
    use robot_coordination_system::zone::behaviour::initialize_zones::initialize_zones;
    use robot_coordination_system::zone::entity::ZONE_REGISTRY;

    use super::*;
    use crate::robot_coordination_system::robot::entity::{Robot, ROBOT_REGISTRY};
    use std::sync::Arc;
    use std::sync::atomic::Ordering;
    use std::thread;
    use std::time::{Duration, Instant};
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

    /* Test zone access safety

        Scenario: System has two robot
        Two tasks are pushed to the queue, both assigned in the same zone
        Robots pop one task each
        one starts working (BUSY) while the other is blocked (IDLE)
        Their statuses don't match, test passes
    */ 
    #[test]
    fn safe_access_to_shared_zone(){
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
