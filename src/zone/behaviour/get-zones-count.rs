use crate::zone::entity::ZONE_REGISTRY;

// Get the total number of zones in the applicarion
pub fn get_zones_count() -> u32 {
    let registry = ZONE_REGISTRY.lock().unwrap();
    registry.len().try_into().unwrap()
}
