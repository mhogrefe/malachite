use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_string::register(runner);
    to_string::register(runner);
}

mod from_string;
mod to_string;
