// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModInverse;
use malachite_base::num::basic::traits::One;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_2_natural_bit_bucketer;
use malachite_q::Rational;
use malachite_q::test_util::bench::bucketers::quadruple_2_natural_bit_bucketer;
use malachite_q::test_util::generators::{natural_pair_gen_var_1, natural_quadruple_gen_var_1};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_reconstruct);
    register_demo!(runner, demo_rational_reconstruct_ref);
    register_demo!(runner, demo_rational_reconstruct_with_bounds);
    register_demo!(runner, demo_rational_reconstruct_with_bounds_ref);
    register_demo!(runner, demo_rational_reconstruct_tier_rows);

    register_bench!(runner, benchmark_rational_reconstruct_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_rational_reconstruct_with_bounds_evaluation_strategy
    );
}

fn demo_rational_reconstruct(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_pair_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        let m_old = m.clone();
        println!(
            "Rational::reconstruct({a_old}, {m_old}) = {:?}",
            Rational::reconstruct(a, m)
        );
    }
}

fn demo_rational_reconstruct_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m) in natural_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "Rational::reconstruct_ref({a}, {m}) = {:?}",
            Rational::reconstruct_ref(&a, &m)
        );
    }
}

fn demo_rational_reconstruct_with_bounds(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m, n_bound, d_bound) in natural_quadruple_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        let m_old = m.clone();
        let n_bound_old = n_bound.clone();
        let d_bound_old = d_bound.clone();
        println!(
            "Rational::reconstruct_with_bounds({a_old}, {m_old}, {n_bound_old}, {d_bound_old}) = \
            {:?}",
            Rational::reconstruct_with_bounds(a, m, &n_bound, &d_bound)
        );
    }
}

fn demo_rational_reconstruct_with_bounds_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m, n_bound, d_bound) in natural_quadruple_gen_var_1().get(gm, config).take(limit) {
        println!(
            "Rational::reconstruct_with_bounds_ref({a}, {m}, {n_bound}, {d_bound}) = {:?}",
            Rational::reconstruct_with_bounds_ref(&a, &m, &n_bound, &d_bound)
        );
    }
}

// Deterministic rows that pin every size tier of the reconstruction, including the huge moduli that
// engage the subquadratic splitter, which random demo inputs cannot reach. The residues are
// computed here rather than written out; each line is diffed against FLINT by the oracle mode that
// recognizes it, and the other lines are skipped.
fn demo_rational_reconstruct_tier_rows(_gm: GenMode, _config: &GenConfig, limit: usize) {
    let mut rows = Vec::new();
    for (num, den, m) in [
        (22u32, 7u32, Natural::from(97u32)),
        (22, 7, (Natural::ONE << 64u32) - Natural::from(59u32)),
        (22, 7, (Natural::ONE << 200u32) - Natural::from(75u32)),
        (22, 7, (Natural::ONE << 765u32) - Natural::from(111u32)),
        (22, 7, (Natural::ONE << 800u32) - Natural::from(105u32)),
        (22, 7, (Natural::ONE << 2000u32) - Natural::from(63u32)),
        (22, 7, (Natural::ONE << 66000u32) - Natural::from(63u32)),
        (
            12345,
            617,
            (Natural::ONE << 33100u32) - Natural::from(121u32),
        ),
    ] {
        let a = (Natural::from(num) * Natural::from(den).mod_inverse(&m).unwrap()) % &m;
        rows.push((a, m));
    }
    for (a, m) in rows.into_iter().take(limit) {
        println!(
            "Rational::reconstruct({a}, {m}) = {:?}",
            Rational::reconstruct_ref(&a, &m)
        );
        let n_bound = Natural::from(999983u32);
        println!(
            "Rational::reconstruct_with_bounds({a}, {m}, {n_bound}, {n_bound}) = {:?}",
            Rational::reconstruct_with_bounds(a.clone(), m.clone(), &n_bound, &n_bound)
        );
    }
}

fn benchmark_rational_reconstruct_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::reconstruct(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Rational::reconstruct(Natural, Natural)", &mut |(a, m)| {
                no_out!(Rational::reconstruct(a, m));
            }),
            (
                "Rational::reconstruct_ref(&Natural, &Natural)",
                &mut |(a, m)| {
                    no_out!(Rational::reconstruct_ref(&a, &m));
                },
            ),
        ],
    );
}

fn benchmark_rational_reconstruct_with_bounds_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::reconstruct_with_bounds(Natural, Natural, Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_natural_bit_bucketer("m"),
        &mut [
            (
                "Rational::reconstruct_with_bounds(Natural, Natural, Natural, Natural)",
                &mut |(a, m, n_bound, d_bound)| {
                    no_out!(Rational::reconstruct_with_bounds(a, m, &n_bound, &d_bound));
                },
            ),
            (
                "Rational::reconstruct_with_bounds_ref(&Natural, &Natural, &Natural, &Natural)",
                &mut |(a, m, n_bound, d_bound)| {
                    no_out!(Rational::reconstruct_with_bounds_ref(
                        &a, &m, &n_bound, &d_bound
                    ));
                },
            ),
        ],
    );
}
