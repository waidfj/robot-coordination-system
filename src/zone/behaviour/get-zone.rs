use crate::zone::entity::{ZONE_REGISTRY, Zone};
use std::sync::Arc;

// Get a zone by its id
pub fn get_zone(id: u32) -> Arc<Zone> {
    let registry = ZONE_REGISTRY.lock().unwrap();

    // We find the zone and return the pointer.
    Arc::clone(registry.get(&id).expect("Zone ID not found"))
}
