use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    cmp::register(runner);
    cmp_abs::register(runner);
    eq::register(runner);
    hash::register(runner);
    partial_cmp_abs_natural::register(runner);
    partial_cmp_abs_primitive_float::register(runner);
    partial_cmp_abs_primitive_int::register(runner);
    partial_cmp_natural::register(runner);
    partial_cmp_primitive_float::register(runner);
    partial_cmp_primitive_int::register(runner);
    partial_eq_natural::register(runner);
    partial_eq_primitive_float::register(runner);
    partial_eq_primitive_int::register(runner);
}

mod cmp;
mod cmp_abs;
mod eq;
mod hash;
mod partial_cmp_abs_natural;
mod partial_cmp_abs_primitive_float;
mod partial_cmp_abs_primitive_int;
mod partial_cmp_natural;
mod partial_cmp_primitive_float;
mod partial_cmp_primitive_int;
mod partial_eq_natural;
mod partial_eq_primitive_float;
mod partial_eq_primitive_int;
