use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    clone::register(runner);
    from_integer::register(runner);
    from_natural::register(runner);
    from_primitive_float::register(runner);
    from_primitive_int::register(runner);
    from_rational::register(runner);
    integer_from_float::register(runner);
    integer_mantissa_and_exponent::register(runner);
    natural_from_float::register(runner);
    primitive_float_from_float::register(runner);
    primitive_int_from_float::register(runner);
    rational_from_float::register(runner);
    raw_mantissa_and_exponent::register(runner);
    sci_mantissa_and_exponent::register(runner);
    string::register(runner);
}

mod clone;
mod from_integer;
mod from_natural;
mod from_primitive_float;
mod from_primitive_int;
mod from_rational;
mod integer_from_float;
mod integer_mantissa_and_exponent;
mod natural_from_float;
mod primitive_float_from_float;
mod primitive_int_from_float;
mod rational_from_float;
mod raw_mantissa_and_exponent;
mod sci_mantissa_and_exponent;
mod string;
