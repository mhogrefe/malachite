use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    clone::register(runner);
    continued_fraction::register(runner);
    digits::register(runner);
    floating_point_from_rational::register(runner);
    from_float_simplest::register(runner);
    from_floating_point::register(runner);
    from_integer::register(runner);
    from_natural::register(runner);
    from_primitive_int::register(runner);
    integer_from_rational::register(runner);
    is_integer::register(runner);
    natural_from_rational::register(runner);
    primitive_int_from_rational::register(runner);
    sci_mantissa_and_exponent::register(runner);
    serde::register(runner);
    string::register(runner);
}

mod clone;
mod continued_fraction;
mod digits;
mod floating_point_from_rational;
mod from_float_simplest;
mod from_floating_point;
mod from_integer;
mod from_natural;
mod from_primitive_int;
mod integer_from_rational;
mod is_integer;
mod natural_from_rational;
mod primitive_int_from_rational;
mod sci_mantissa_and_exponent;
mod serde;
mod string;
