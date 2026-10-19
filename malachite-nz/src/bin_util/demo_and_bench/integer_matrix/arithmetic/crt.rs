// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::BalancedCrt;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::integer_matrix::arithmetic::crt::integer_matrix_balanced_crt_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_balanced_crt_val_val_val);
    register_demo!(runner, demo_integer_matrix_balanced_crt_val_val_ref);
    register_demo!(runner, demo_integer_matrix_balanced_crt_val_ref_val);
    register_demo!(runner, demo_integer_matrix_balanced_crt_val_ref_ref);
    register_demo!(runner, demo_integer_matrix_balanced_crt_ref_val_val);
    register_demo!(runner, demo_integer_matrix_balanced_crt_ref_val_ref);
    register_demo!(runner, demo_integer_matrix_balanced_crt_ref_ref_val);
    register_demo!(runner, demo_integer_matrix_balanced_crt_ref_ref_ref);

    register_bench!(
        runner,
        benchmark_integer_matrix_balanced_crt_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_matrix_balanced_crt_algorithms);
}

fn demo_integer_matrix_balanced_crt_val_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in integer_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let m1_old = m1.clone();
        let b_old = b.clone();
        println!(
            "{}.balanced_crt({}, {}, {}) = {}",
            a_old,
            m1_old,
            b_old,
            m2,
            a.balanced_crt(m1, b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_integer_matrix_balanced_crt_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in integer_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let m1_old = m1.clone();
        println!(
            "{}.balanced_crt({}, {}, {}) = {}",
            a_old,
            m1_old,
            b,
            m2,
            a.balanced_crt(m1, &b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_integer_matrix_balanced_crt_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in integer_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.balanced_crt({}, {}, {}) = {}",
            a_old,
            m1,
            b_old,
            m2,
            a.balanced_crt(&m1, b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_integer_matrix_balanced_crt_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in integer_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{}.balanced_crt({}, {}, {}) = {}",
            a_old,
            m1,
            b,
            m2,
            a.balanced_crt(&m1, &b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_integer_matrix_balanced_crt_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in integer_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let m1_old = m1.clone();
        let b_old = b.clone();
        println!(
            "{}.balanced_crt({}, {}, {}) = {}",
            a,
            m1_old,
            b_old,
            m2,
            (&a).balanced_crt(m1, b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_integer_matrix_balanced_crt_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in integer_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let m1_old = m1.clone();
        println!(
            "{}.balanced_crt({}, {}, {}) = {}",
            a,
            m1_old,
            b,
            m2,
            (&a).balanced_crt(m1, &b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_integer_matrix_balanced_crt_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in integer_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let b_old = b.clone();
        println!(
            "{}.balanced_crt({}, {}, {}) = {}",
            a,
            m1,
            b_old,
            m2,
            (&a).balanced_crt(&m1, b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn demo_integer_matrix_balanced_crt_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, m1, b, m2) in integer_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.balanced_crt({}, {}, {}) = {}",
            a,
            m1,
            b,
            m2,
            (&a).balanced_crt(&m1, &b, m2)
                .map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
        );
    }
}

fn benchmark_integer_matrix_balanced_crt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.balanced_crt(Natural, UnsignedMatrix<Limb>, Limb)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_integer_matrix_bit_bucketer("a"),
        &mut [
            (
                "IntegerMatrix.balanced_crt(Natural, UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!(a.balanced_crt(m1, b, m2));
                },
            ),
            (
                "IntegerMatrix.balanced_crt(Natural, &UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!(a.balanced_crt(m1, &b, m2));
                },
            ),
            (
                "IntegerMatrix.balanced_crt(&Natural, UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!(a.balanced_crt(&m1, b, m2));
                },
            ),
            (
                "IntegerMatrix.balanced_crt(&Natural, &UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!(a.balanced_crt(&m1, &b, m2));
                },
            ),
            (
                "&IntegerMatrix.balanced_crt(Natural, UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!((&a).balanced_crt(m1, b, m2));
                },
            ),
            (
                "&IntegerMatrix.balanced_crt(Natural, &UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!((&a).balanced_crt(m1, &b, m2));
                },
            ),
            (
                "&IntegerMatrix.balanced_crt(&Natural, UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!((&a).balanced_crt(&m1, b, m2));
                },
            ),
            (
                "&IntegerMatrix.balanced_crt(&Natural, &UnsignedMatrix<Limb>, Limb)",
                &mut |(a, m1, b, m2)| {
                    no_out!((&a).balanced_crt(&m1, &b, m2));
                },
            ),
        ],
    );
}

fn benchmark_integer_matrix_balanced_crt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.balanced_crt(&Natural, &UnsignedMatrix<Limb>, Limb)",
        BenchmarkType::Algorithms,
        integer_matrix_natural_unsigned_matrix_unsigned_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, m1, b, m2)| {
                no_out!((&a).balanced_crt(&m1, &b, m2));
            }),
            ("one element at a time", &mut |(a, m1, b, m2)| {
                no_out!(integer_matrix_balanced_crt_naive(&a, &m1, &b, m2));
            }),
        ],
    );
}
