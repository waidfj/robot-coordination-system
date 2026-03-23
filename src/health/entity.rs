use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::Instant;

pub static HEARTBEAT_REGISTRY: LazyLock<Mutex<HashMap<u32, Instant>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
