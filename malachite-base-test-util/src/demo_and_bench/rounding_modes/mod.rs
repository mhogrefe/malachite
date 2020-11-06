use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    clone::register(runner);
    display::register(runner);
    eq::register(runner);
    hash::register(runner);
    neg::register(runner);
}

mod clone;
mod display;
mod eq;
mod hash;
mod neg;
