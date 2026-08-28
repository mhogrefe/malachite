// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from::register(runner);
    from_primitive_float::register(runner);
    gaussian_integer_from_gaussian_rational::register(runner);
    imaginary_from::register(runner);
    integer_from_gaussian_rational::register(runner);
    is_gaussian_integer::register(runner);
    is_integer::register(runner);
    is_real::register(runner);
    natural_from_gaussian_rational::register(runner);
    primitive_float_from_gaussian_rational::register(runner);
    primitive_int_from_gaussian_rational::register(runner);
    rational_from_gaussian_rational::register(runner);
    string::register(runner);
}

mod from;
mod from_primitive_float;
mod gaussian_integer_from_gaussian_rational;
mod imaginary_from;
mod integer_from_gaussian_rational;
mod is_gaussian_integer;
mod is_integer;
mod is_real;
mod natural_from_gaussian_rational;
mod primitive_float_from_gaussian_rational;
mod primitive_int_from_gaussian_rational;
mod rational_from_gaussian_rational;
mod string;
