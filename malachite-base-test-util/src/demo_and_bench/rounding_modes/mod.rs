use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    clone::register(runner);
    eq::register(runner);
    from_str::register(runner);
    hash::register(runner);
    neg::register(runner);
    to_string::register(runner);
}

mod clone;
mod eq;
mod from_str;
mod hash;
mod neg;
mod to_string;
