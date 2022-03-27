use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    cmp::register(runner);
    eq::register(runner);
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
mod hash;
mod partial_cmp_abs_primitive_float;
mod partial_cmp_abs_primitive_int;
mod partial_cmp_primitive_float;
mod partial_cmp_primitive_int;
mod partial_eq_primitive_float;
mod partial_eq_primitive_int;
