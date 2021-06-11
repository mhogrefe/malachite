use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    basic::register(runner);
    nice_float::register(runner);
}

mod basic;
mod nice_float;
