use common::DemoBenchRegistry;

pub mod and;
pub mod and_i32;
pub mod and_u32;
pub mod assign_bit;
pub mod checked_count_ones;
pub mod checked_count_zeros;
pub mod checked_hamming_distance;
pub mod checked_hamming_distance_i32;
pub mod checked_hamming_distance_u32;
pub mod clear_bit;
pub mod flip_bit;
pub mod get_bit;
pub mod not;
pub mod set_bit;
pub mod significant_bits;
pub mod trailing_zeros;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    and::register(registry);
    and_i32::register(registry);
    and_u32::register(registry);
    assign_bit::register(registry);
    checked_count_ones::register(registry);
    checked_count_zeros::register(registry);
    checked_hamming_distance::register(registry);
    checked_hamming_distance_i32::register(registry);
    checked_hamming_distance_u32::register(registry);
    clear_bit::register(registry);
    flip_bit::register(registry);
    get_bit::register(registry);
    not::register(registry);
    set_bit::register(registry);
    significant_bits::register(registry);
    trailing_zeros::register(registry);
}
