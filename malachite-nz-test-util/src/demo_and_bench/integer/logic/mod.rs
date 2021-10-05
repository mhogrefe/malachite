use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    and::register(runner);
    assign_bit::register(runner);
    assign_bits::register(runner);
    bits::register(runner);
    checked_count_ones::register(runner);
    checked_count_zeros::register(runner);
    checked_hamming_distance::register(runner);
    clear_bit::register(runner);
    flip_bit::register(runner);
    from_bits::register(runner);
    get_bit::register(runner);
    get_bits::register(runner);
    index_of_next_false_bit::register(runner);
    index_of_next_true_bit::register(runner);
    low_mask::register(runner);
    not::register(runner);
    or::register(runner);
    set_bit::register(runner);
    significant_bits::register(runner);
    to_bits::register(runner);
    trailing_zeros::register(runner);
    xor::register(runner);
}

mod and;
mod assign_bit;
mod assign_bits;
mod bits;
mod checked_count_ones;
mod checked_count_zeros;
mod checked_hamming_distance;
mod clear_bit;
mod flip_bit;
mod from_bits;
mod get_bit;
mod get_bits;
mod index_of_next_false_bit;
mod index_of_next_true_bit;
mod low_mask;
mod not;
mod or;
mod set_bit;
mod significant_bits;
mod to_bits;
mod trailing_zeros;
mod xor;
