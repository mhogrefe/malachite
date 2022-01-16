use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs::register(runner);
    add::register(runner);
    ceiling::register(runner);
    div::register(runner);
    floor::register(runner);
    is_power_of_2::register(runner);
    mul::register(runner);
    neg::register(runner);
    next_power_of_2::register(runner);
    power_of_2::register(runner);
    reciprocal::register(runner);
    shl::register(runner);
    shr::register(runner);
    sign::register(runner);
    sub::register(runner);
}

mod abs;
mod add;
mod ceiling;
mod div;
mod floor;
mod is_power_of_2;
mod mul;
mod neg;
mod next_power_of_2;
mod power_of_2;
mod reciprocal;
mod shl;
mod shr;
mod sign;
mod sub;
