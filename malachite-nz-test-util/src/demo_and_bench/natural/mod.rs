use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    arithmetic::register(runner);
    conversion::register(runner);
}

mod arithmetic;
mod conversion;
