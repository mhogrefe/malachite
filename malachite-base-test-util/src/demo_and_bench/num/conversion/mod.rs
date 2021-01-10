use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    digits::register(runner);
    from::register(runner);
    half::register(runner);
    slice::register(runner);
}

mod digits;
mod from;
mod half;
mod slice;
