use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    get_bit::register(runner);
}

mod get_bit;
