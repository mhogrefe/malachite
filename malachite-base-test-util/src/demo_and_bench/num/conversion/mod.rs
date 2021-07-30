use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    digits::register(runner);
    from::register(runner);
    half::register(runner);
    is_integer::register(runner);
    mantissa_and_exponent::register(runner);
    slice::register(runner);
    string::register(runner);
}

mod digits;
mod from;
mod half;
mod is_integer;
mod mantissa_and_exponent;
mod slice;
mod string;
