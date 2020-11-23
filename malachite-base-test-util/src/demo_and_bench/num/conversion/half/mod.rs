use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    join_halves::register(runner);
    lower_half::register(runner);
    split_in_half::register(runner);
    upper_half::register(runner);
}

mod join_halves;
mod lower_half;
mod split_in_half;
mod upper_half;
