// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Crt;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_matrix::arithmetic::crt::natural_matrix_crt_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_crt_val_val_val);
    register_demo!(runner, demo_natural_matrix_crt_val_val_ref);
    register_demo!(runner, demo_natural_matrix_crt_val_ref_val);
    register_demo!(runner, demo_natural_matrix_crt_val_ref_ref);
    register_demo!(runner, demo_natural_matrix_crt_ref_val_val);
    register_demo!(runner, demo_natural_matrix_crt_ref_val_ref);
    register_demo!(runner, demo_natural_matrix_crt_ref_ref_val);
    register_demo!(runner, demo_natural_matrix_crt_ref_ref_ref);

    register_bench!(runner, benchmark_natural_matrix_crt_evaluation_strategy);
    register_bench!(runner, benchmark_natural_matrix_crt_algorithms);
}

fn demo_natural_matrix_crt_val_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in natural_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let m1_old = m1.clone();
        let b_old = b.clone();
        println!(
            "{}.crt({}, {}, {}) = {}",
            a_old,
            m1_old,
            b_old,
            m2,
            a.crt(m1, b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_natural_matrix_crt_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in natural_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let m1_old = m1.clone();
        println!(
            "{}.crt({}, {}, {}) = {}",
            a_old,
            m1_old,
            b,
            m2,
            a.crt(m1, &b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_natural_matrix_crt_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in natural_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.crt({}, {}, {}) = {}",
            a_old,
            m1,
            b_old,
            m2,
            a.crt(&m1, b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_natural_matrix_crt_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in natural_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{}.crt({}, {}, {}) = {}",
            a_old,
            m1,
            b,
            m2,
            a.crt(&m1, &b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_natural_matrix_crt_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in natural_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let m1_old = m1.clone();
        let b_old = b.clone();
        println!(
            "{}.crt({}, {}, {}) = {}",
            a,
            m1_old,
            b_old,
            m2,
            (&a).crt(m1, b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_natural_matrix_crt_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in natural_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let m1_old = m1.clone();
        println!(
            "{}.crt({}, {}, {}) = {}",
            a,
            m1_old,
            b,
            m2,
            (&a).crt(m1, &b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_natural_matrix_crt_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in natural_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let b_old = b.clone();
        println!(
            "{}.crt({}, {}, {}) = {}",
            a,
            m1,
            b_old,
            m2,
            (&a).crt(&m1, b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_natural_matrix_crt_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in natural_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.crt({}, {}, {}) = {}",
            a,
            m1,
            b,
            m2,
            (&a).crt(&m1, &b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn benchmark_natural_matrix_crt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.crt(Natural, UnsignedMatrix<Limb>, Limb)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.crt(Natural, UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!(a.crt(m1, b, m2));
                },
            ),
            (
                "NaturalMatrix.crt(Natural, &UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!(a.crt(m1, &b, m2));
                },
            ),
            (
                "NaturalMatrix.crt(&Natural, UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!(a.crt(&m1, b, m2));
                },
            ),
            (
                "NaturalMatrix.crt(&Natural, &UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!(a.crt(&m1, &b, m2));
                },
            ),
            (
                "&NaturalMatrix.crt(Natural, UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!((&a).crt(m1, b, m2));
                },
            ),
            (
                "&NaturalMatrix.crt(Natural, &UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!((&a).crt(m1, &b, m2));
                },
            ),
            (
                "&NaturalMatrix.crt(&Natural, UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!((&a).crt(&m1, b, m2));
                },
            ),
            (
                "&NaturalMatrix.crt(&Natural, &UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!((&a).crt(&m1, &b, m2));
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_crt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.crt(&Natural, &UnsignedMatrix<Limb>, Limb)",
        BenchmarkType::Algorithms,
        natural_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, m1, b, m2)| {
                no_out!((&a).crt(&m1, &b, m2));
            }),
            ("one element at a time", &mut |(a, m1, b, m2)| {
                no_out!(natural_matrix_crt_naive(&a, &m1, &b, m2));
            }),
        ],
    );
}
