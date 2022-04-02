use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_digits::register(runner);
    to_digits::register(runner);
}

mod from_digits;
mod to_digits;
