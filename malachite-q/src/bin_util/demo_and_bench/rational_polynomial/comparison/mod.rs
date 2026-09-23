// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    cmp::register(runner);
    partial_eq_gaussian_integer::register(runner);
    partial_eq_gaussian_rational::register(runner);
    partial_eq_integer::register(runner);
    partial_eq_integer_polynomial::register(runner);
    partial_eq_natural::register(runner);
    partial_eq_natural_polynomial::register(runner);
    partial_eq_primitive_int::register(runner);
    partial_eq_rational::register(runner);
    partial_eq_unsigned_polynomial::register(runner);
    shortlex_cmp::register(runner);
}

mod cmp;
mod partial_eq_gaussian_integer;
mod partial_eq_gaussian_rational;
mod partial_eq_integer;
mod partial_eq_integer_polynomial;
mod partial_eq_natural;
mod partial_eq_natural_polynomial;
mod partial_eq_primitive_int;
mod partial_eq_rational;
mod partial_eq_unsigned_polynomial;
mod shortlex_cmp;
