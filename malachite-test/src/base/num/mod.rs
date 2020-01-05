use common::DemoBenchRegistry;

pub mod assign_bit;
pub mod clear_bit;
pub mod comparison;
pub mod conversion;
pub mod decrement;
pub mod flip_bit;
pub mod get_bit;
pub mod increment;
pub mod log_two;
pub mod logic;
pub mod set_bit;
pub mod significant_bits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    assign_bit::register(registry);
    clear_bit::register(registry);
    comparison::register(registry);
    conversion::register(registry);
    decrement::register(registry);
    flip_bit::register(registry);
    get_bit::register(registry);
    increment::register(registry);
    logic::register(registry);
    log_two::register(registry);
    set_bit::register(registry);
    significant_bits::register(registry);
}
