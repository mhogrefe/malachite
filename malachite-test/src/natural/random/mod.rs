use malachite_test::common::DemoBenchRegistry;

pub mod random_natural_below;
pub mod random_natural_up_to_bits;
pub mod random_natural_with_bits;
pub mod special_random_natural_below;
pub mod special_random_natural_up_to_bits;
pub mod special_random_natural_with_bits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    random_natural_below::register(registry);
    random_natural_up_to_bits::register(registry);
    random_natural_with_bits::register(registry);
    special_random_natural_below::register(registry);
    special_random_natural_up_to_bits::register(registry);
    special_random_natural_with_bits::register(registry);
}
