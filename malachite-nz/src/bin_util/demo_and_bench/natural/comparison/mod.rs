use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    cmp::register(runner);
    eq::register(runner);
    eq_abs_primitive_float::register(runner);
    eq_abs_primitive_int::register(runner);
    hash::register(runner);
    partial_cmp_abs_primitive_float::register(runner);
    partial_cmp_abs_primitive_int::register(runner);
    partial_cmp_primitive_float::register(runner);
    partial_cmp_primitive_int::register(runner);
    partial_eq_primitive_int::register(runner);
    partial_eq_primitive_float::register(runner);
}

mod cmp;
mod eq;
mod eq_abs_primitive_float;
mod eq_abs_primitive_int;
mod hash;
mod partial_cmp_abs_primitive_float;
mod partial_cmp_abs_primitive_int;
mod partial_cmp_primitive_float;
mod partial_cmp_primitive_int;
mod partial_eq_primitive_float;
mod partial_eq_primitive_int;
