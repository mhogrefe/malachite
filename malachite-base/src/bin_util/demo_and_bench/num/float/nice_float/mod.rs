use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    cmp::register(runner);
    eq::register(runner);
    from_str::register(runner);
    hash::register(runner);
    to_string::register(runner);
}

mod cmp;
mod eq;
mod from_str;
mod hash;
mod to_string;
