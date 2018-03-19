use common::DemoBenchRegistry;

pub mod clone;
pub mod eq;
pub mod hash;
pub mod neg;

pub fn register(registry: &mut DemoBenchRegistry) {
    clone::register(registry);
    eq::register(registry);
    hash::register(registry);
    neg::register(registry);
}
