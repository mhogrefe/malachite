// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::ifft_negacyclic::ifft_negacyclic;
use malachite_nz::test_util::generators::large_type_gen_var_43;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_ifft_negacyclic);
}

fn demo_ifft_negacyclic(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut ii, n, w) in large_type_gen_var_43().get(gm, config).take(limit) {
        let ii_old = ii.clone();
        let limbs = ii[0].len() - 1;
        let mut t1 = vec![0; limbs + 1];
        let mut t2 = vec![0; limbs + 1];
        let mut temp = vec![0; limbs + 1];
        ifft_negacyclic(&mut ii, n, w, &mut t1, &mut t2, &mut temp);
        println!("ifft_negacyclic({ii_old:?}, {n}, {w}, _, _, _) = {ii:?}");
    }
}
