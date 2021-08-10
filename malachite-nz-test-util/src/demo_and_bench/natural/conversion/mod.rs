use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    digits::register(runner);
    floating_point_from_natural::register(runner);
    from_floating_point::register(runner);
    from_primitive_int::register(runner);
    integer_mantissa_and_exponent::register(runner);
    is_integer::register(runner);
    sci_mantissa_and_exponent::register(runner);
    serde::register(runner);
    string::register(runner);
}

mod digits;
mod floating_point_from_natural;
mod from_floating_point;
mod from_primitive_int;
mod integer_mantissa_and_exponent;
mod is_integer;
mod sci_mantissa_and_exponent;
mod serde;
mod string;
