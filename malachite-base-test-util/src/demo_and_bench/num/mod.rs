use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    basic::register(runner);
    conversion::register(runner);
    logic::register(runner);
}

mod basic;
mod conversion;
mod logic;
