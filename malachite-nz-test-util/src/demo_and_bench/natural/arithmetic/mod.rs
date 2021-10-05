use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    coprime_with::register(runner);
    gcd::register(runner);
    lcm::register(runner);
    log_base::register(runner);
    log_base_2::register(runner);
    log_base_power_of_2::register(runner);
    mul::register(runner);
    neg::register(runner);
    root::register(runner);
    sqrt::register(runner);
}

mod coprime_with;
mod gcd;
mod lcm;
mod log_base;
mod log_base_2;
mod log_base_power_of_2;
mod mul;
mod neg;
mod root;
mod sqrt;
