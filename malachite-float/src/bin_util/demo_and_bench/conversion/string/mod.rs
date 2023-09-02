use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    to_string::register(runner);
}

mod to_string;
