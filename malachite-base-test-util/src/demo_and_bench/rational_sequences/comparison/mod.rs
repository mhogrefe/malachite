use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    cmp::register(runner);
    eq::register(runner);
    hash::register(runner);
}

mod cmp;
mod eq;
mod hash;
