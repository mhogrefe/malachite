use common::DemoBenchRegistry;

pub mod limbs_delete_left;
pub mod limbs_pad_left;
pub mod limbs_set_zero;
pub mod limbs_test_zero;

pub fn register(registry: &mut DemoBenchRegistry) {
    limbs_delete_left::register(registry);
    limbs_pad_left::register(registry);
    limbs_set_zero::register(registry);
    limbs_test_zero::register(registry);
}
