use common::DemoBenchRegistry;

pub mod assign_bit;
pub mod clear_bit;
pub mod count_ones;
pub mod flip_bit;
pub mod get_bit;
pub mod hamming_distance;
pub mod hamming_distance_u32;
pub mod limb_count;
pub mod not;
pub mod set_bit;
pub mod significant_bits;
pub mod trailing_zeros;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    assign_bit::register(registry);
    clear_bit::register(registry);
    count_ones::register(registry);
    flip_bit::register(registry);
    get_bit::register(registry);
    hamming_distance::register(registry);
    hamming_distance_u32::register(registry);
    limb_count::register(registry);
    not::register(registry);
    set_bit::register(registry);
    significant_bits::register(registry);
    trailing_zeros::register(registry);
}
