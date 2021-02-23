use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    digits::register(runner);
    string::register(runner);
}

mod digits;
mod string;
