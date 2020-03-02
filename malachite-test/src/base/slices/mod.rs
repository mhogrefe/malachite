use common::DemoBenchRegistry;

pub mod slice_leading_zeros;
pub mod slice_move_left;
pub mod slice_set_zero;
pub mod slice_test_zero;
pub mod slice_trailing_zeros;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    slice_leading_zeros::register(registry);
    slice_move_left::register(registry);
    slice_set_zero::register(registry);
    slice_test_zero::register(registry);
    slice_trailing_zeros::register(registry);
}
