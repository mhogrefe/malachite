use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs_negative_zero::register(runner);
    from_ordered_representation::register(runner);
    is_negative_zero::register(runner);
    next_higher::register(runner);
    next_lower::register(runner);
    to_ordered_representation::register(runner);
}

mod abs_negative_zero;
mod from_ordered_representation;
mod is_negative_zero;
mod next_higher;
mod next_lower;
mod to_ordered_representation;
