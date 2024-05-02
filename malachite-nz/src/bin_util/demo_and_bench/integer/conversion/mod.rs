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
    floating_point_from_integer::register(runner);
    from_bool::register(runner);
    from_floating_point::register(runner);
    from_natural::register(runner);
    from_primitive_int::register(runner);
    from_twos_complement_limbs::register(runner);
    is_integer::register(runner);
    natural_from_integer::register(runner);
    primitive_int_from_integer::register(runner);
    serde::register(runner);
    string::register(runner);
    to_twos_complement_limbs::register(runner);
}

mod clone;
mod floating_point_from_integer;
mod from_bool;
mod from_floating_point;
mod from_natural;
mod from_primitive_int;
mod from_twos_complement_limbs;
mod is_integer;
mod natural_from_integer;
mod primitive_int_from_integer;
mod serde;
mod string;
mod to_twos_complement_limbs;
