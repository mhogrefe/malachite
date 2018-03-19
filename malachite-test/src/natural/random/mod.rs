use common::DemoBenchRegistry;

pub mod random_natural_below;
pub mod random_natural_up_to_bits;
pub mod random_natural_with_bits;
pub mod special_random_natural_below;
pub mod special_random_natural_up_to_bits;
pub mod special_random_natural_with_bits;

pub fn register(registry: &mut DemoBenchRegistry) {
    random_natural_below::register(registry);
}
