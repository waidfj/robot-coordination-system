use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

// The main container of all the tasks in the application
pub static TASK_QUEUE: LazyLock<Mutex<VecDeque<Task>>> =
    LazyLock::new(|| Mutex::new(VecDeque::new()));

// The main interface of the Task entity
pub struct Task {
    pub(crate) id: u32,
    duration: u64,
}

impl Task {
    // The default constructor
    pub fn new(id: u32, duration: u64) -> Self {
        Self { id, duration }
    }

    // Returns the time needed to execute some task, to achieve hiding information
    pub fn get_duration(&self) -> u64 {
        self.duration
    }
}
