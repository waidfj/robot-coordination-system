use crate::zone::entity::{ZONE_REGISTRY, Zone};
use std::sync::Arc;

// Add the zones to the application for usage
//  takes the list of zones as a parameter
pub fn initialize_zones(names: Vec<&str>) {
    let mut registry = ZONE_REGISTRY.lock().unwrap();

    // Add to the the global registry
    for (index, name) in names.into_iter().enumerate() {
        // The id of the zone is the index of it
        let id = index as u32;

        // The key is the id, the value is the name
        registry.insert(id, Arc::new(Zone::new(id, name)));
    }
}
