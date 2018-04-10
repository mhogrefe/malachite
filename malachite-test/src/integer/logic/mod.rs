use common::DemoBenchRegistry;

pub mod assign_bit;
pub mod checked_count_ones;
pub mod checked_count_zeros;
pub mod clear_bit;
pub mod flip_bit;
pub mod get_bit;
pub mod not;
pub mod set_bit;
pub mod significant_bits;
pub mod trailing_zeros;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    assign_bit::register(registry);
    checked_count_ones::register(registry);
    checked_count_zeros::register(registry);
    clear_bit::register(registry);
    flip_bit::register(registry);
    get_bit::register(registry);
    not::register(registry);
    set_bit::register(registry);
    significant_bits::register(registry);
    trailing_zeros::register(registry);
}
