use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    clone::register(runner);
    from_integer::register(runner);
    from_natural::register(runner);
    from_primitive_int::register(runner);
    integer_from_rational::register(runner);
    is_integer::register(runner);
    natural_from_rational::register(runner);
    primitive_int_from_rational::register(runner);
    serde::register(runner);
    string::register(runner);
}

mod clone;
mod from_integer;
mod from_natural;
mod from_primitive_int;
mod integer_from_rational;
mod is_integer;
mod natural_from_rational;
mod primitive_int_from_rational;
mod serde;
mod string;
