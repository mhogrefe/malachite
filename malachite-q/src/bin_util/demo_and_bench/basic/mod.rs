use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    significant_bits::register(runner);
}

mod significant_bits;
