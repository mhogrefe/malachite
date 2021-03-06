use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs::register(runner);
    add_mul::register(runner);
    arithmetic_checked_shl::register(runner);
    arithmetic_checked_shr::register(runner);
    sub_mul::register(runner);
    x_mul_y_is_zz::register(runner);
}

mod abs;
mod add_mul;
mod arithmetic_checked_shl;
mod arithmetic_checked_shr;
mod sub_mul;
mod x_mul_y_is_zz;
