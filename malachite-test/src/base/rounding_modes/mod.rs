use common::DemoBenchRegistry;

pub mod clone;
pub mod display;
pub mod eq;
pub mod from_str;
pub mod hash;
pub mod neg;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    clone::register(registry);
    display::register(registry);
    eq::register(registry);
    from_str::register(registry);
    hash::register(registry);
    neg::register(registry);
}
