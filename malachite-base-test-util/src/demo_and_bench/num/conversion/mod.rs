use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from::register(runner);
    half::register(runner);
    slice::register(runner);
}

mod from;
mod half;
mod slice;
