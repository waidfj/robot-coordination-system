use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

// The main container of all the zones in the application
pub static ZONE_REGISTRY: LazyLock<Mutex<HashMap<u32, Arc<Zone>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct Zone {
    pub id: u32,
    pub name: String,
    pub lock: Mutex<()>,
}

impl Zone {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            lock: Mutex::new(()),
        }
    }
}
