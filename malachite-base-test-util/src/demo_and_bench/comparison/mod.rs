use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs_comparators::register(runner);
    cmp_abs_and_partial_cmp_abs::register(runner);
    macros::register(runner);
}

mod abs_comparators;
mod cmp_abs_and_partial_cmp_abs;
mod macros;
