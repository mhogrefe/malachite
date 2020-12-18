use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    add_mul::register(runner);
    sub_mul::register(runner);
    x_mul_y_is_zz::register(runner);
}

mod add_mul;
mod sub_mul;
mod x_mul_y_is_zz;
