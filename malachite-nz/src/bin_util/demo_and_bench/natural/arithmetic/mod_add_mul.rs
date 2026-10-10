// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModAdd, ModAddMul, ModAddMulAssign, ModMul};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_quadruple_gen_var_1;
use malachite_nz::test_util::natural::arithmetic::mod_add_mul::mod_add_mul_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_add_mul);
    register_demo!(runner, demo_natural_mod_add_mul_ref);
    register_demo!(runner, demo_natural_mod_add_mul_assign);

    register_bench!(runner, benchmark_natural_mod_add_mul_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_add_mul_algorithms);
}

fn demo_natural_mod_add_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, m) in natural_quadruple_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        let m_old = m.clone();
        println!(
            "{} + {} * {} ≡ {} mod {}",
            x_old,
            y_old,
            z_old,
            x.mod_add_mul(y, z, m),
            m_old
        );
    }
}

fn demo_natural_mod_add_mul_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, m) in natural_quadruple_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).mod_add_mul(&{}, &{}, &{}) = {}",
            x,
            y,
            z,
            m,
            (&x).mod_add_mul(&y, &z, &m)
        );
    }
}

fn demo_natural_mod_add_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, m) in natural_quadruple_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mod_add_mul_assign(&y, &z, &m);
        println!("x := {x_old}; x.mod_add_mul_assign(&{y}, &{z}, &{m}); x = {x}");
    }
}

fn benchmark_natural_mod_add_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_add_mul(Natural, Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_natural_max_bit_bucketer("x", "y", "z", "m"),
        &mut [
            (
                "Natural.mod_add_mul(Natural, Natural, Natural)",
                &mut |(x, y, z, m)| {
                    no_out!(x.mod_add_mul(y, z, m));
                },
            ),
            (
                "Natural.mod_add_mul(Natural, Natural, &Natural)",
                &mut |(x, y, z, m)| {
                    no_out!(x.mod_add_mul(y, z, &m));
                },
            ),
            (
                "Natural.mod_add_mul(Natural, &Natural, Natural)",
                &mut |(x, y, z, m)| {
                    no_out!(x.mod_add_mul(y, &z, m));
                },
            ),
            (
                "Natural.mod_add_mul(Natural, &Natural, &Natural)",
                &mut |(x, y, z, m)| {
                    no_out!(x.mod_add_mul(y, &z, &m));
                },
            ),
            (
                "Natural.mod_add_mul(&Natural, Natural, Natural)",
                &mut |(x, y, z, m)| {
                    no_out!(x.mod_add_mul(&y, z, m));
                },
            ),
            (
                "Natural.mod_add_mul(&Natural, Natural, &Natural)",
                &mut |(x, y, z, m)| {
                    no_out!(x.mod_add_mul(&y, z, &m));
                },
            ),
            (
                "Natural.mod_add_mul(&Natural, &Natural, Natural)",
                &mut |(x, y, z, m)| {
                    no_out!(x.mod_add_mul(&y, &z, m));
                },
            ),
            (
                "Natural.mod_add_mul(&Natural, &Natural, &Natural)",
                &mut |(x, y, z, m)| {
                    no_out!(x.mod_add_mul(&y, &z, &m));
                },
            ),
            (
                "(&Natural).mod_add_mul(&Natural, &Natural, &Natural)",
                &mut |(x, y, z, m)| {
                    no_out!((&x).mod_add_mul(&y, &z, &m));
                },
            ),
            (
                "Natural.mod_add_mul_assign(Natural, Natural, Natural)",
                &mut |(mut x, y, z, m)| {
                    x.mod_add_mul_assign(y, z, m);
                },
            ),
            (
                "Natural.mod_add_mul_assign(Natural, Natural, &Natural)",
                &mut |(mut x, y, z, m)| {
                    x.mod_add_mul_assign(y, z, &m);
                },
            ),
            (
                "Natural.mod_add_mul_assign(Natural, &Natural, Natural)",
                &mut |(mut x, y, z, m)| {
                    x.mod_add_mul_assign(y, &z, m);
                },
            ),
            (
                "Natural.mod_add_mul_assign(Natural, &Natural, &Natural)",
                &mut |(mut x, y, z, m)| {
                    x.mod_add_mul_assign(y, &z, &m);
                },
            ),
            (
                "Natural.mod_add_mul_assign(&Natural, Natural, Natural)",
                &mut |(mut x, y, z, m)| {
                    x.mod_add_mul_assign(&y, z, m);
                },
            ),
            (
                "Natural.mod_add_mul_assign(&Natural, Natural, &Natural)",
                &mut |(mut x, y, z, m)| {
                    x.mod_add_mul_assign(&y, z, &m);
                },
            ),
            (
                "Natural.mod_add_mul_assign(&Natural, &Natural, Natural)",
                &mut |(mut x, y, z, m)| {
                    x.mod_add_mul_assign(&y, &z, m);
                },
            ),
            (
                "Natural.mod_add_mul_assign(&Natural, &Natural, &Natural)",
                &mut |(mut x, y, z, m)| {
                    x.mod_add_mul_assign(&y, &z, &m);
                },
            ),
        ],
    );
}

fn benchmark_natural_mod_add_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_add_mul(Natural, Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_natural_max_bit_bucketer("x", "y", "z", "m"),
        &mut [
            ("default", &mut |(x, y, z, m)| {
                no_out!(x.mod_add_mul(y, z, m));
            }),
            ("unfused", &mut |(x, y, z, m)| {
                no_out!(x.mod_add(y.mod_mul(z, &m), m));
            }),
            ("naive", &mut |(x, y, z, m)| {
                no_out!(mod_add_mul_naive(&x, &y, &z, &m));
            }),
        ],
    );
}
