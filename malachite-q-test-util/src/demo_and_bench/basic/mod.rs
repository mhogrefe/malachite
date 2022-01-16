use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_numerator_and_denominator::register(runner);
    mutate_numerator_or_denominator::register(runner);
    significant_bits::register(runner);
    to_numerator_or_denominator::register(runner);
}

mod from_numerator_and_denominator;
mod mutate_numerator_or_denominator;
mod significant_bits;
mod to_numerator_or_denominator;
