use common::DemoBenchRegistry;

pub mod bools;
pub mod chars;
pub mod limbs;
pub mod num;
pub mod rounding_mode;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    bools::register(registry);
    chars::register(registry);
    limbs::register(registry);
    num::register(registry);
    rounding_mode::register(registry);
}
