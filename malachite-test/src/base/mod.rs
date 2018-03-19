use common::DemoBenchRegistry;

pub mod char;
pub mod limbs;
pub mod num;
pub mod rounding_mode;

pub fn register(registry: &mut DemoBenchRegistry) {
    char::register(registry);
    limbs::register(registry);
    num::register(registry);
    rounding_mode::register(registry);
}
