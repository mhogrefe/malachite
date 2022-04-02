use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    cmp::register(runner);
    eq::register(runner);
    hash::register(runner);
}

mod cmp;
mod eq;
mod hash;
