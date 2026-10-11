// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAddMulShl, ModAddMulShlAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quintuple_1_2_3_5_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural::arithmetic::mod_add_mul_shl::mod_add_mul_shl_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_add_mul_shl);
    register_demo!(runner, demo_natural_mod_add_mul_shl_ref);
    register_demo!(runner, demo_natural_mod_add_mul_shl_assign);

    register_bench!(
        runner,
        benchmark_natural_mod_add_mul_shl_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_add_mul_shl_algorithms);
}

fn demo_natural_mod_add_mul_shl(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, bits, m) in natural_natural_natural_unsigned_natural_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        let m_old = m.clone();
        println!(
            "{x_old} + {y_old} * {z_old} * 2^{bits} ≡ {} mod {m_old}",
            x.mod_add_mul_shl(y, z, bits, m)
        );
    }
}

fn demo_natural_mod_add_mul_shl_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, bits, m) in natural_natural_natural_unsigned_natural_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).mod_add_mul_shl(&{y}, &{z}, {bits}, &{m}) = {}",
            (&x).mod_add_mul_shl(&y, &z, bits, &m)
        );
    }
}

fn demo_natural_mod_add_mul_shl_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, bits, m) in natural_natural_natural_unsigned_natural_quintuple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.mod_add_mul_shl_assign(&y, &z, bits, &m);
        println!("x := {x_old}; x.mod_add_mul_shl_assign(&{y}, &{z}, {bits}, &{m}); x = {x}");
    }
}

fn benchmark_natural_mod_add_mul_shl_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_add_mul_shl(Natural, Natural, u64, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_natural_unsigned_natural_quintuple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_2_3_5_natural_max_bit_bucketer("x", "y", "z", "m"),
        &mut [
            (
                "Natural.mod_add_mul_shl(Natural, Natural, u64, Natural)",
                &mut |(x, y, z, bits, m)| {
                    no_out!(x.mod_add_mul_shl(y, z, bits, m));
                },
            ),
            (
                "Natural.mod_add_mul_shl(Natural, Natural, u64, &Natural)",
                &mut |(x, y, z, bits, m)| {
                    no_out!(x.mod_add_mul_shl(y, z, bits, &m));
                },
            ),
            (
                "Natural.mod_add_mul_shl(Natural, &Natural, u64, Natural)",
                &mut |(x, y, z, bits, m)| {
                    no_out!(x.mod_add_mul_shl(y, &z, bits, m));
                },
            ),
            (
                "Natural.mod_add_mul_shl(Natural, &Natural, u64, &Natural)",
                &mut |(x, y, z, bits, m)| {
                    no_out!(x.mod_add_mul_shl(y, &z, bits, &m));
                },
            ),
            (
                "Natural.mod_add_mul_shl(&Natural, Natural, u64, Natural)",
                &mut |(x, y, z, bits, m)| {
                    no_out!(x.mod_add_mul_shl(&y, z, bits, m));
                },
            ),
            (
                "Natural.mod_add_mul_shl(&Natural, Natural, u64, &Natural)",
                &mut |(x, y, z, bits, m)| {
                    no_out!(x.mod_add_mul_shl(&y, z, bits, &m));
                },
            ),
            (
                "Natural.mod_add_mul_shl(&Natural, &Natural, u64, Natural)",
                &mut |(x, y, z, bits, m)| {
                    no_out!(x.mod_add_mul_shl(&y, &z, bits, m));
                },
            ),
            (
                "Natural.mod_add_mul_shl(&Natural, &Natural, u64, &Natural)",
                &mut |(x, y, z, bits, m)| {
                    no_out!(x.mod_add_mul_shl(&y, &z, bits, &m));
                },
            ),
            (
                "(&Natural).mod_add_mul_shl(&Natural, &Natural, u64, &Natural)",
                &mut |(x, y, z, bits, m)| {
                    no_out!((&x).mod_add_mul_shl(&y, &z, bits, &m));
                },
            ),
            (
                "Natural.mod_add_mul_shl_assign(Natural, Natural, u64, Natural)",
                &mut |(mut x, y, z, bits, m)| {
                    x.mod_add_mul_shl_assign(y, z, bits, m);
                },
            ),
            (
                "Natural.mod_add_mul_shl_assign(Natural, Natural, u64, &Natural)",
                &mut |(mut x, y, z, bits, m)| {
                    x.mod_add_mul_shl_assign(y, z, bits, &m);
                },
            ),
            (
                "Natural.mod_add_mul_shl_assign(Natural, &Natural, u64, Natural)",
                &mut |(mut x, y, z, bits, m)| {
                    x.mod_add_mul_shl_assign(y, &z, bits, m);
                },
            ),
            (
                "Natural.mod_add_mul_shl_assign(Natural, &Natural, u64, &Natural)",
                &mut |(mut x, y, z, bits, m)| {
                    x.mod_add_mul_shl_assign(y, &z, bits, &m);
                },
            ),
            (
                "Natural.mod_add_mul_shl_assign(&Natural, Natural, u64, Natural)",
                &mut |(mut x, y, z, bits, m)| {
                    x.mod_add_mul_shl_assign(&y, z, bits, m);
                },
            ),
            (
                "Natural.mod_add_mul_shl_assign(&Natural, Natural, u64, &Natural)",
                &mut |(mut x, y, z, bits, m)| {
                    x.mod_add_mul_shl_assign(&y, z, bits, &m);
                },
            ),
            (
                "Natural.mod_add_mul_shl_assign(&Natural, &Natural, u64, Natural)",
                &mut |(mut x, y, z, bits, m)| {
                    x.mod_add_mul_shl_assign(&y, &z, bits, m);
                },
            ),
            (
                "Natural.mod_add_mul_shl_assign(&Natural, &Natural, u64, &Natural)",
                &mut |(mut x, y, z, bits, m)| {
                    x.mod_add_mul_shl_assign(&y, &z, bits, &m);
                },
            ),
        ],
    );
}

fn benchmark_natural_mod_add_mul_shl_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_add_mul_shl(Natural, Natural, u64, Natural)",
        BenchmarkType::Algorithms,
        natural_natural_natural_unsigned_natural_quintuple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_2_3_5_natural_max_bit_bucketer("x", "y", "z", "m"),
        &mut [
            ("default", &mut |(x, y, z, bits, m)| {
                no_out!(x.mod_add_mul_shl(y, z, bits, m));
            }),
            ("naive", &mut |(x, y, z, bits, m)| {
                no_out!(mod_add_mul_shl_naive(&x, &y, &z, bits, &m));
            }),
        ],
    );
}
