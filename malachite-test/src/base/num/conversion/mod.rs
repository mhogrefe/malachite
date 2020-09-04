use malachite_test::common::DemoBenchRegistry;

pub mod from_other_type_slice;
pub mod join_halves;
pub mod lower_half;
pub mod split_in_half;
pub mod upper_half;
pub mod vec_from_other_type;
pub mod vec_from_other_type_slice;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    from_other_type_slice::register(registry);
    join_halves::register(registry);
    lower_half::register(registry);
    split_in_half::register(registry);
    upper_half::register(registry);
    vec_from_other_type::register(registry);
    vec_from_other_type_slice::register(registry);
}
