use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    nice_float::register(runner);
}

mod nice_float;
