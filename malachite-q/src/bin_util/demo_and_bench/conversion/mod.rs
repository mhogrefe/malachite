// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    clone::register(runner);
    continued_fraction::register(runner);
    digits::register(runner);
    from_bool::register(runner);
    from_float_simplest::register(runner);
    from_integer::register(runner);
    from_natural::register(runner);
    from_numerator_and_denominator::register(runner);
    from_primitive_float::register(runner);
    from_primitive_int::register(runner);
    integer_from_rational::register(runner);
    is_integer::register(runner);
    mutate_numerator_or_denominator::register(runner);
    natural_from_rational::register(runner);
    primitive_float_from_rational::register(runner);
    primitive_int_from_rational::register(runner);
    sci_mantissa_and_exponent::register(runner);
    serde::register(runner);
    string::register(runner);
    to_numerator_or_denominator::register(runner);
}

mod clone;
mod continued_fraction;
mod digits;
mod from_bool;
mod from_float_simplest;
mod from_integer;
mod from_natural;
mod from_numerator_and_denominator;
mod from_primitive_float;
mod from_primitive_int;
mod integer_from_rational;
mod is_integer;
mod mutate_numerator_or_denominator;
mod natural_from_rational;
mod primitive_float_from_rational;
mod primitive_int_from_rational;
mod sci_mantissa_and_exponent;
mod serde;
mod string;
mod to_numerator_or_denominator;
