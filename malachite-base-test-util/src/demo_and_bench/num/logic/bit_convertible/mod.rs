use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    to_bits::register(runner);
}

mod to_bits;
