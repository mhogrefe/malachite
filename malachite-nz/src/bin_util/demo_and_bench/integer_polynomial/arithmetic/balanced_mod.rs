// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{BalancedMod, BalancedModAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_integer_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_balanced_mod);
    register_demo!(runner, demo_integer_polynomial_balanced_mod_ref);
    register_demo!(runner, demo_integer_polynomial_balanced_mod_assign);
    register_demo!(runner, demo_integer_polynomial_balanced_mod_small_moduli);

    register_bench!(
        runner,
        benchmark_integer_polynomial_balanced_mod_evaluation_strategy
    );
}

fn demo_integer_polynomial_balanced_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in integer_polynomial_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!("({p_old}).balanced_mod({m}) = {}", p.balanced_mod(&m));
    }
}

fn demo_integer_polynomial_balanced_mod_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, m) in integer_polynomial_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&({p})).balanced_mod({m}) = {}", (&p).balanced_mod(&m));
    }
}

fn demo_integer_polynomial_balanced_mod_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, m) in integer_polynomial_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.balanced_mod_assign(&m);
        println!("p := {p_old}; p.balanced_mod_assign({m}); p = {p}");
    }
}

// The moduli 1, -1, and 2 are the edge cases the property tests single out: everything vanishes
// for the first two, and the third leaves only 0s and 1s.
fn demo_integer_polynomial_balanced_mod_small_moduli(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        for m in [Integer::from(1), Integer::from(-1), Integer::from(2)] {
            println!("(&({p})).balanced_mod({m}) = {}", (&p).balanced_mod(&m));
        }
    }
}

fn benchmark_integer_polynomial_balanced_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.balanced_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_polynomial_integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("IntegerPolynomial.balanced_mod(Integer)", &mut |(p, m)| {
                no_out!(p.balanced_mod(m));
            }),
            ("IntegerPolynomial.balanced_mod(&Integer)", &mut |(p, m)| {
                no_out!(p.balanced_mod(&m));
            }),
            (
                "(&IntegerPolynomial).balanced_mod(Integer)",
                &mut |(p, m)| {
                    no_out!((&p).balanced_mod(m));
                },
            ),
            (
                "(&IntegerPolynomial).balanced_mod(&Integer)",
                &mut |(p, m)| {
                    no_out!((&p).balanced_mod(&m));
                },
            ),
            (
                "IntegerPolynomial.balanced_mod_assign(Integer)",
                &mut |(mut p, m)| p.balanced_mod_assign(m),
            ),
            (
                "IntegerPolynomial.balanced_mod_assign(&Integer)",
                &mut |(mut p, m)| p.balanced_mod_assign(&m),
            ),
        ],
    );
}
