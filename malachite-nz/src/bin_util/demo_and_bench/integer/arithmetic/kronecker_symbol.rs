// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{JacobiSymbol, KroneckerSymbol};
use malachite_base::test_util::bench::bucketers::quadruple_1_3_vec_max_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::large_type_gen_var_27;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::arithmetic::kronecker_symbol::limbs_kronecker_symbol;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_integer_bit_bucketer, pair_2_pair_integer_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_pair_gen, integer_pair_gen_rm, integer_pair_gen_var_4, integer_pair_gen_var_4_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_kronecker_symbol);
    register_demo!(runner, demo_integer_jacobi_symbol);
    register_demo!(runner, demo_integer_jacobi_symbol_val_ref);
    register_demo!(runner, demo_integer_jacobi_symbol_ref_val);
    register_demo!(runner, demo_integer_jacobi_symbol_ref_ref);
    register_demo!(runner, demo_integer_kronecker_symbol);
    register_demo!(runner, demo_integer_kronecker_symbol_val_ref);
    register_demo!(runner, demo_integer_kronecker_symbol_ref_val);
    register_demo!(runner, demo_integer_kronecker_symbol_ref_ref);

    register_bench!(runner, benchmark_limbs_kronecker_symbol);
    register_bench!(runner, benchmark_integer_jacobi_symbol_library_comparison);
    register_bench!(runner, benchmark_integer_jacobi_symbol_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_integer_kronecker_symbol_library_comparison
    );
    register_bench!(
        runner,
        benchmark_integer_kronecker_symbol_evaluation_strategy
    );
}

fn demo_limbs_kronecker_symbol(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x_sign, xs, y_sign, ys) in large_type_gen_var_27().get(gm, config).take(limit) {
        println!(
            "limbs_kronecker_symbol({}, {:?}, {}, {:?}) = {}",
            x_sign,
            xs,
            y_sign,
            ys,
            limbs_kronecker_symbol(x_sign, &xs, y_sign, &ys)
        );
    }
}

fn demo_integer_jacobi_symbol(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in integer_pair_gen_var_4().get(gm, config).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        println!(
            "({}).jacobi_symbol({}) = {}",
            n_old,
            m_old,
            n.jacobi_symbol(m)
        );
    }
}

fn demo_integer_jacobi_symbol_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in integer_pair_gen_var_4().get(gm, config).take(limit) {
        let n_old = n.clone();
        println!("({}).jacobi_symbol({}) = {}", n_old, m, n.jacobi_symbol(&m));
    }
}

fn demo_integer_jacobi_symbol_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in integer_pair_gen_var_4().get(gm, config).take(limit) {
        let m_old = m.clone();
        println!(
            "({}).jacobi_symbol({}) = {}",
            n,
            m_old,
            (&n).jacobi_symbol(m)
        );
    }
}

fn demo_integer_jacobi_symbol_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in integer_pair_gen_var_4().get(gm, config).take(limit) {
        println!("({}).jacobi_symbol({}) = {}", n, m, (&n).jacobi_symbol(&m));
    }
}

fn demo_integer_kronecker_symbol(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in integer_pair_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        println!(
            "({}).kronecker_symbol({}) = {}",
            n_old,
            m_old,
            n.kronecker_symbol(m)
        );
    }
}

fn demo_integer_kronecker_symbol_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in integer_pair_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        println!(
            "({}).kronecker_symbol({}) = {}",
            n_old,
            m,
            n.kronecker_symbol(&m)
        );
    }
}

fn demo_integer_kronecker_symbol_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in integer_pair_gen().get(gm, config).take(limit) {
        let m_old = m.clone();
        println!(
            "({}).kronecker_symbol({}) = {}",
            n,
            m_old,
            (&n).kronecker_symbol(m)
        );
    }
}

fn demo_integer_kronecker_symbol_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in integer_pair_gen().get(gm, config).take(limit) {
        println!(
            "({}).kronecker_symbol({}) = {}",
            n,
            m,
            (&n).kronecker_symbol(&m)
        );
    }
}

fn benchmark_limbs_kronecker_symbol(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_kronecker_symbol(bool, &[Limb], bool, &[Limb])",
        BenchmarkType::Single,
        large_type_gen_var_27().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_3_vec_max_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(x_sign, xs, y_sign, ys)| {
            no_out!(limbs_kronecker_symbol(x_sign, &xs, y_sign, &ys))
        })],
    );
}

fn benchmark_integer_jacobi_symbol_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.jacobi_symbol(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_4_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.jacobi_symbol(y))),
            ("rug", &mut |((x, y), _)| no_out!(x.jacobi(&y))),
        ],
    );
}

fn benchmark_integer_jacobi_symbol_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.jacobi_symbol(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("m"),
        &mut [
            ("Integer.jacobi_symbol(Integer)", &mut |(n, m)| {
                no_out!(n.jacobi_symbol(m))
            }),
            ("Integer.jacobi_symbol(&Integer)", &mut |(n, m)| {
                no_out!(n.jacobi_symbol(&m))
            }),
            ("(&Integer).jacobi_symbol(Integer)", &mut |(n, m)| {
                no_out!((&n).jacobi_symbol(m))
            }),
            ("(&Integer).jacobi_symbol(&Integer)", &mut |(n, m)| {
                no_out!((&n).jacobi_symbol(&m))
            }),
        ],
    );
}

fn benchmark_integer_kronecker_symbol_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.kronecker_symbol(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| {
                no_out!(x.kronecker_symbol(y))
            }),
            ("rug", &mut |((x, y), _)| no_out!(x.kronecker(&y))),
        ],
    );
}

fn benchmark_integer_kronecker_symbol_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.kronecker_symbol(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("m"),
        &mut [
            ("Integer.kronecker_symbol(Integer)", &mut |(n, m)| {
                no_out!(n.kronecker_symbol(m))
            }),
            ("Integer.kronecker_symbol(&Integer)", &mut |(n, m)| {
                no_out!(n.kronecker_symbol(&m))
            }),
            ("(&Integer).kronecker_symbol(Integer)", &mut |(n, m)| {
                no_out!((&n).kronecker_symbol(m))
            }),
            ("(&Integer).kronecker_symbol(&Integer)", &mut |(n, m)| {
                no_out!((&n).kronecker_symbol(&m))
            }),
        ],
    );
}
