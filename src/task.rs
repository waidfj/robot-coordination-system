use std::time::{SystemTime, Duration};
use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;
use std::collections::BinaryHeap;
use std::cmp::Ordering;


pub struct Task {
    pub id: u64,
    pub task_type: TaskType,
    pub zoneid: Zone,
    pub created_at: SystemTime,
    pub status: Status,
    pub priority: Priority,
    pub max_wait_time: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Operatingroom,
    Ward,
    Pharmacy,
    Corridor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskType {
    Delivery,
    Disinfection,
    SurgicalAssistance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pending,
    Assigned,
    Completed,
    Failed,
    TimedOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        if self.id == other.id {
            true
        } else {
            false
        }
    }
}


impl Eq for Task {}

impl Ord for Task{
    fn cmp(&self, other: &Self) -> Ordering {
        let now_task_priority = self.priority;
        let other_task_priority = other.priority;
        if now_task_priority  == Priority::High && other_task_priority == Priority::High{
            Ordering::Equal
        }
        else if now_task_priority == Priority::High && other_task_priority == Priority::Medium{
            Ordering::Greater
        }
        else if now_task_priority == Priority::High && other_task_priority == Priority::Low{
            Ordering::Greater
        }
        else if now_task_priority == Priority::Medium&& other_task_priority == Priority::High{
            Ordering::Less
        }
        else if now_task_priority == Priority::Low&& other_task_priority == Priority::High{
            Ordering::Less
        }
        else if now_task_priority == Priority::Medium&& other_task_priority == Priority::Low{
            Ordering::Greater
        }
        else if now_task_priority == Priority::Low&& other_task_priority == Priority::Medium{
            Ordering::Less
        }
        else if now_task_priority == Priority::Medium&& other_task_priority == Priority::Medium{
            Ordering::Equal
        }
        else{
            Ordering::Equal
        }
    }
}

pub fn start_task_generator(task_queue: Arc<Mutex<BinaryHeap<Task>>>) {
    thread::spawn(move || {
        let mut id = 0;
        let mut rng = rand::thread_rng();
        loop {
            let task = create_task(id, &mut rng);
            {
                let mut queue = task_queue.lock().unwrap();
                queue.push(task);
                println!("The new task is created! \n The Task ID is {} ", id);
            }
            id += 1;
            thread::sleep(Duration::from_secs(5)); // sleep time = 5 
        }
    });
}
  
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn create_task (id: u64, rng: &mut rand::rngs::ThreadRng) -> Task{
    let zoneid = rng.gen_range(0..=2);
    let zone = if zoneid == 0{
        Zone::Operatingroom
    }
    else if zoneid==1{
        Zone::Ward
    }
    else {
        Zone::Pharmacy
    };
    let task_num = rng.gen_range(0..=2);

    let task_type = if task_num == 0{
            TaskType::Delivery
        }
        else if task_num == 1{
            TaskType::Disinfection   
        }
        else{
            TaskType::SurgicalAssistance
        };

    let priority = match task_type {
        TaskType::SurgicalAssistance => Priority::High,
        TaskType::Disinfection => Priority::Medium,
        TaskType::Delivery => Priority::Low,
    };

    Task{
        id,
        task_type,
        zoneid: zone,
        created_at: SystemTime::now(),
        status: Status::Pending,
        priority,
        max_wait_time: Duration::from_secs(30),

    }

    }
    
