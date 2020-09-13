use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    crement::register(runner);
}

pub mod crement;
