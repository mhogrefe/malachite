use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    access::register(runner);
    basic::register(runner);
    comparison::register(runner);
    conversion::register(runner);
    to_string::register(runner);
}

mod access;
mod basic;
mod comparison;
mod conversion;
mod to_string;
