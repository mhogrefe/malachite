use common::DemoBenchRegistry;

pub mod char;
pub mod limbs {
    pub mod limbs_delete_left;
    pub mod limbs_pad_left;
    pub mod limbs_set_zero;
    pub mod limbs_test_zero;
}
pub mod num {
    pub mod assign_bit;
    pub mod clear_bit;
    pub mod decrement;
    pub mod flip_bit;
    pub mod increment;
    pub mod join_halves;
    pub mod log_two;
    pub mod lower_half;
    pub mod get_bit;
    pub mod set_bit;
    pub mod significant_bits;
    pub mod split_in_half;
    pub mod upper_half;
}
pub mod rounding_mode {
    pub mod clone;
    pub mod eq;
    pub mod hash;
    pub mod neg;
}

pub fn register(registry: &mut DemoBenchRegistry) {
    self::char::register(registry);
}
