// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    clone::register(runner);
    digits::register(runner);
    floating_point_from_natural::register(runner);
    from_bool::register(runner);
    from_floating_point::register(runner);
    from_limbs::register(runner);
    from_primitive_int::register(runner);
    integer_mantissa_and_exponent::register(runner);
    is_integer::register(runner);
    primitive_int_from_natural::register(runner);
    sci_mantissa_and_exponent::register(runner);
    serde::register(runner);
    string::register(runner);
    to_limbs::register(runner);
}

mod clone;
mod digits;
mod floating_point_from_natural;
mod from_bool;
mod from_floating_point;
mod from_limbs;
mod from_primitive_int;
mod integer_mantissa_and_exponent;
mod is_integer;
mod primitive_int_from_natural;
mod sci_mantissa_and_exponent;
mod serde;
mod string;
mod to_limbs;
