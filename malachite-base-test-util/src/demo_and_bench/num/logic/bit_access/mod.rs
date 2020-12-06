use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    flip_bit::register(runner);
    get_bit::register(runner);
}

mod flip_bit;
mod get_bit;
