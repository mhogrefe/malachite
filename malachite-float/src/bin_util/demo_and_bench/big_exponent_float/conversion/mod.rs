// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    big_exponent_float_from_float::register(runner);
    float_from_big_exponent_float::register(runner);
    from_gaussian_integer::register(runner);
    from_gaussian_rational::register(runner);
    from_integer::register(runner);
    from_natural::register(runner);
    from_primitive_float::register(runner);
    from_primitive_int::register(runner);
    from_rational::register(runner);
    gaussian_integer_from_big_exponent_float::register(runner);
    gaussian_rational_from_big_exponent_float::register(runner);
    integer_from_big_exponent_float::register(runner);
    is_integer::register(runner);
    mantissa_and_exponent::register(runner);
    natural_from_big_exponent_float::register(runner);
    primitive_float_from_big_exponent_float::register(runner);
    primitive_int_from_big_exponent_float::register(runner);
    rational_from_big_exponent_float::register(runner);
    string::register(runner);
}

mod big_exponent_float_from_float;
mod float_from_big_exponent_float;
mod from_gaussian_integer;
mod from_gaussian_rational;
mod from_integer;
mod from_natural;
mod from_primitive_float;
mod from_primitive_int;
mod from_rational;
mod gaussian_integer_from_big_exponent_float;
mod gaussian_rational_from_big_exponent_float;
mod integer_from_big_exponent_float;
mod is_integer;
mod mantissa_and_exponent;
mod natural_from_big_exponent_float;
mod primitive_float_from_big_exponent_float;
mod primitive_int_from_big_exponent_float;
mod rational_from_big_exponent_float;
mod string;
