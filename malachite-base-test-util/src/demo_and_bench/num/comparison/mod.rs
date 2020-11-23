use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    eq_abs::register(runner);
    cmp_abs_and_partial_cmp_abs::register(runner);
    ord_abs_comparators::register(runner);
}

mod cmp_abs_and_partial_cmp_abs;
mod eq_abs;
mod ord_abs_comparators;
