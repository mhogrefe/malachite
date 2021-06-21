use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    abs_negative_zero::register(runner);
    from_ordered_representation::register(runner);
    from_integer_mantissa_and_exponent::register(runner);
    from_raw_mantissa_and_exponent::register(runner);
    from_sci_mantissa_and_exponent::register(runner);
    integer_mantissa_and_exponent::register(runner);
    is_negative_zero::register(runner);
    next_higher::register(runner);
    next_lower::register(runner);
    raw_mantissa_and_exponent::register(runner);
    sci_mantissa_and_exponent::register(runner);
    to_ordered_representation::register(runner);
}

mod abs_negative_zero;
mod from_integer_mantissa_and_exponent;
mod from_ordered_representation;
mod from_raw_mantissa_and_exponent;
mod from_sci_mantissa_and_exponent;
mod integer_mantissa_and_exponent;
mod is_negative_zero;
mod next_higher;
mod next_lower;
mod raw_mantissa_and_exponent;
mod sci_mantissa_and_exponent;
mod to_ordered_representation;
