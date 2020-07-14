use malachite_test::common::DemoBenchRegistry;

pub mod checked_from_and_exact_from;
pub mod convertible_from;
pub mod from_other_type_slice;
pub mod join_halves;
pub mod lower_half;
pub mod overflowing_from;
pub mod saturating_from;
pub mod split_in_half;
pub mod upper_half;
pub mod vec_from_other_type;
pub mod vec_from_other_type_slice;
pub mod wrapping_from;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    checked_from_and_exact_from::register(registry);
    convertible_from::register(registry);
    from_other_type_slice::register(registry);
    join_halves::register(registry);
    lower_half::register(registry);
    overflowing_from::register(registry);
    saturating_from::register(registry);
    split_in_half::register(registry);
    upper_half::register(registry);
    vec_from_other_type::register(registry);
    vec_from_other_type_slice::register(registry);
    wrapping_from::register(registry);
}
