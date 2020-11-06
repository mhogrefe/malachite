use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    x_mul_y_is_zz::register(runner);
}

mod x_mul_y_is_zz;
