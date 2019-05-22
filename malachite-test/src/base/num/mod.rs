use common::DemoBenchRegistry;

pub mod assign_bit;
pub mod clear_bit;
pub mod conversion;
pub mod decrement;
pub mod flip_bit;
pub mod get_bit;
pub mod increment;
pub mod join_halves;
pub mod log_two;
pub mod lower_half;
pub mod set_bit;
pub mod significant_bits;
pub mod split_in_half;
pub mod upper_half;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    assign_bit::register(registry);
    clear_bit::register(registry);
    conversion::register(registry);
    decrement::register(registry);
    flip_bit::register(registry);
    get_bit::register(registry);
    increment::register(registry);
    join_halves::register(registry);
    log_two::register(registry);
    lower_half::register(registry);
    set_bit::register(registry);
    significant_bits::register(registry);
    split_in_half::register(registry);
    upper_half::register(registry);
}
