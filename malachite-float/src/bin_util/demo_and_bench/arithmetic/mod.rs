use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs::register(runner);
    add::register(runner);
    is_power_of_2::register(runner);
    neg::register(runner);
    power_of_2::register(runner);
    sign::register(runner);
    sub::register(runner);
}

mod abs;
mod add;
mod is_power_of_2;
mod neg;
mod power_of_2;
mod sign;
mod sub;
