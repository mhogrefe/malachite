use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    crement::register(runner);
}

mod crement;
