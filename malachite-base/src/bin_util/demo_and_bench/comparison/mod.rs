use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    macros::register(runner);
}

mod macros;
