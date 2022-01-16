use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    arithmetic::register(runner);
    basic::register(runner);
    comparison::register(runner);
    conversion::register(runner);
    logic::register(runner);
}

mod arithmetic;
mod basic;
mod comparison;
mod conversion;
mod logic;
