// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    cmp::register(runner);
    cmp_abs::register(runner);
    eq::register(runner);
    hash::register(runner);
    partial_cmp_abs_integer::register(runner);
    partial_cmp_abs_natural::register(runner);
    partial_cmp_abs_primitive_float::register(runner);
    partial_cmp_abs_primitive_int::register(runner);
    partial_cmp_abs_rational::register(runner);
    partial_cmp_integer::register(runner);
    partial_cmp_natural::register(runner);
    partial_cmp_primitive_float::register(runner);
    partial_cmp_primitive_int::register(runner);
    partial_cmp_rational::register(runner);
    partial_eq_integer::register(runner);
    partial_eq_natural::register(runner);
    partial_eq_primitive_float::register(runner);
    partial_eq_primitive_int::register(runner);
    partial_eq_rational::register(runner);
}

mod cmp;
mod cmp_abs;
mod eq;
mod hash;
mod partial_cmp_abs_integer;
mod partial_cmp_abs_natural;
mod partial_cmp_abs_primitive_float;
mod partial_cmp_abs_primitive_int;
mod partial_cmp_abs_rational;
mod partial_cmp_integer;
mod partial_cmp_natural;
mod partial_cmp_primitive_float;
mod partial_cmp_primitive_int;
mod partial_cmp_rational;
mod partial_eq_integer;
mod partial_eq_natural;
mod partial_eq_primitive_float;
mod partial_eq_primitive_int;
mod partial_eq_rational;
