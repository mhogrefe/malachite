use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    conversion::register(runner);
    logic::register(runner);
}

pub mod conversion;
pub mod logic;
