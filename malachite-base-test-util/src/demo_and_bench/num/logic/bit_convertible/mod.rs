use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_bits::register(runner);
    to_bits::register(runner);
}

mod from_bits;
mod to_bits;
