// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::option_unsigned_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_option_to_typst_unsigned);
}

fn demo_option_to_typst_unsigned<T: PrimitiveUnsigned + ToTypst>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for o in option_unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("{:?}.to_typst() = {}", o, o.to_typst());
    }
}
