use malachite_test::common::DemoBenchRegistry;

pub mod and;
pub mod assign_bit;
pub mod assign_bits;
pub mod bits;
pub mod clear_bit;
pub mod count_ones;
pub mod flip_bit;
pub mod from_bits;
pub mod get_bit;
pub mod get_bits;
pub mod hamming_distance;
pub mod index_of_next_false_bit;
pub mod index_of_next_true_bit;
pub mod limb_count;
pub mod low_mask;
pub mod not;
pub mod or;
pub mod set_bit;
pub mod significant_bits;
pub mod to_bits;
pub mod trailing_zeros;
pub mod xor;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    and::register(registry);
    assign_bit::register(registry);
    assign_bits::register(registry);
    bits::register(registry);
    clear_bit::register(registry);
    count_ones::register(registry);
    flip_bit::register(registry);
    from_bits::register(registry);
    get_bit::register(registry);
    get_bits::register(registry);
    hamming_distance::register(registry);
    index_of_next_false_bit::register(registry);
    index_of_next_true_bit::register(registry);
    limb_count::register(registry);
    low_mask::register(registry);
    not::register(registry);
    or::register(registry);
    set_bit::register(registry);
    significant_bits::register(registry);
    to_bits::register(registry);
    trailing_zeros::register(registry);
    xor::register(registry);
}
