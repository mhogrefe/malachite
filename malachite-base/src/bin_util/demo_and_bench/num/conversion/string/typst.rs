// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_to_typst_unsigned);
    register_signed_demos!(runner, demo_to_typst_signed);
    register_primitive_float_demos!(runner, demo_to_typst_primitive_float);
}

fn demo_to_typst_unsigned<T: PrimitiveUnsigned + ToTypst>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("{}.to_typst() = {}", x, x.to_typst());
    }
}

fn demo_to_typst_signed<T: PrimitiveSigned + ToTypst>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in signed_gen::<T>().get(gm, config).take(limit) {
        println!("{}.to_typst() = {}", x, x.to_typst());
    }
}

fn demo_to_typst_primitive_float<T: PrimitiveFloat + ToTypst>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!("{}.to_typst() = {}", NiceFloat(x), x.to_typst());
    }
}
