// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;
use malachite_float::BigExponentFloat;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_power_of_2_u64);
    register_demo!(runner, demo_big_exponent_float_power_of_2_i64);
}

fn demo_big_exponent_float_power_of_2_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen::<u64>().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::power_of_2({x}) = {:#x}",
            BigExponentFloat::power_of_2(x)
        );
    }
}

fn demo_big_exponent_float_power_of_2_i64(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in signed_gen::<i64>().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::power_of_2({x}) = {:#x}",
            BigExponentFloat::power_of_2(x)
        );
    }
}
