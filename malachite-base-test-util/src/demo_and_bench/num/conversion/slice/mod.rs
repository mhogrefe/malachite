use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_other_type_slice::register(runner);
    vec_from_other_type::register(runner);
    vec_from_other_type_slice::register(runner);
}

mod from_other_type_slice;
mod vec_from_other_type;
mod vec_from_other_type_slice;
