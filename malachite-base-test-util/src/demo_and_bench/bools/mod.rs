use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    not_assign::register(runner);
}

mod not_assign;
