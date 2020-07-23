use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_primitive_integer::register(runner);
}

pub mod from_primitive_integer;
