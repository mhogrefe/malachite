// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::certify_lu_mod_p::rref_upper_certify_lu_mod_p;
use malachite_nz::integer_matrix::arithmetic::word_lu::lu_with_pivots;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{integer_matrix_gen, integer_matrix_pair_gen_var_4};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_rref_upper_certify_lu_mod_p);
    register_demo!(
        runner,
        demo_integer_matrix_rref_upper_certify_lu_mod_p_low_rank
    );
}

// Small primes, modulo which matrices often have lower rank than over the integers.
const PRIMES: [Limb; 4] = [2, 3, 5, 7];

fn print_certificates(a: &IntegerMatrix) {
    for p in PRIMES {
        let (rank, perm, pivots_nonpivots) = lu_with_pivots(a, p);
        println!(
            "rref_upper_certify_lu_mod_p({a}, {p}) = {}",
            rref_upper_certify_lu_mod_p(a, rank, &perm, &pivots_nonpivots).map_or_else(
                || "None".to_string(),
                |(e, den)| format!("Some(({e}, {den}))")
            )
        );
    }
}

fn demo_integer_matrix_rref_upper_certify_lu_mod_p(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        print_certificates(&a);
    }
}

// Products of matrices whose inner dimension is often smaller than the outer ones, so that many
// have less than full rank.
fn demo_integer_matrix_rref_upper_certify_lu_mod_p_low_rank(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        print_certificates(&(a * b));
    }
}
