use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs::register(runner);
    add_mul::register(runner);
    arithmetic_checked_shl::register(runner);
    arithmetic_checked_shr::register(runner);
    checked_add_mul::register(runner);
    checked_square::register(runner);
    checked_sub_mul::register(runner);
    div_exact::register(runner);
    div_mod::register(runner);
    div_round::register(runner);
    sub_mul::register(runner);
    x_mul_y_is_zz::register(runner);
}

mod abs;
mod add_mul;
mod arithmetic_checked_shl;
mod arithmetic_checked_shr;
mod checked_add_mul;
mod checked_square;
mod checked_sub_mul;
mod div_exact;
mod div_mod;
mod div_round;
mod sub_mul;
mod x_mul_y_is_zz;
