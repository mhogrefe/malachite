use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    vec_from_other_type::register(runner);
}

mod vec_from_other_type;
