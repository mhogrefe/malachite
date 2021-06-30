use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    integer_mantissa_and_exponent::register(runner);
    raw_mantissa_and_exponent::register(runner);
    sci_mantissa_and_exponent::register(runner);
}

mod integer_mantissa_and_exponent;
mod raw_mantissa_and_exponent;
mod sci_mantissa_and_exponent;
