use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs_negative_zero::register(runner);
    from_ordered_representation::register(runner);
    is_negative_zero::register(runner);
    max_precision_for_sci_exponent::register(runner);
    next_higher::register(runner);
    next_lower::register(runner);
    precision::register(runner);
    to_ordered_representation::register(runner);
}

mod abs_negative_zero;
mod from_ordered_representation;
mod is_negative_zero;
mod max_precision_for_sci_exponent;
mod next_higher;
mod next_lower;
mod precision;
mod to_ordered_representation;
