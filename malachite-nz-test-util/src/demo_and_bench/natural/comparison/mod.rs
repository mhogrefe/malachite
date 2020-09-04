use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_primitive_int::register(runner);
}

pub mod from_primitive_int;
