// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::exhaustive::exhaustive_natural_inclusive_range;
use malachite_nz::natural::Natural;
use malachite_q::arithmetic::traits::{Approximate, ApproximateAssign};
use malachite_q::test_util::arithmetic::approximate::approximate_naive;
use malachite_q::test_util::bench::bucketers::pair_1_rational_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_gen_var_7, rational_natural_pair_gen_var_3, rational_natural_pair_gen_var_4,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_approximate_assign);
    register_demo!(runner, demo_rational_approximate);
    register_demo!(runner, demo_rational_approximate_ref);
    register_demo!(runner, demo_rational_approximate_2);

    register_bench!(runner, benchmark_rational_approximate_assign);
    register_bench!(runner, benchmark_rational_approximate_algorithms);
    register_bench!(runner, benchmark_rational_approximate_evaluation_strategy);
}

fn demo_rational_approximate_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rational_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.approximate_assign(&y);
        println!("x := {x_old}; x.approximate_assign({y}); x = {x}");
    }
}

fn demo_rational_approximate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("({}).approximate({}) = {}", x.clone(), y, x.approximate(&y));
    }
}

fn demo_rational_approximate_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).approximate({}) = {}", x, y, (&x).approximate(&y));
    }
}

fn demo_rational_approximate_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen_var_7().get(gm, config).take(limit) {
        println!("{x}");
        for d in exhaustive_natural_inclusive_range(Natural::ONE, x.to_denominator()) {
            let a = (&x).approximate(&d);
            println!("    {}: {} ≈ {}", d, a, NiceFloat(f64::exact_from(&a)));
        }
    }
}

fn benchmark_rational_approximate_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.approximate_assign(&Natural)",
        BenchmarkType::Single,
        rational_natural_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut x, y)| x.approximate_assign(&y))],
    );
}

fn benchmark_rational_approximate_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.approximate(&Natural)",
        BenchmarkType::Algorithms,
        rational_natural_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("n"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.approximate(&y))),
            ("naive", &mut |(x, y)| no_out!(approximate_naive(&x, &y))),
        ],
    );
}

fn benchmark_rational_approximate_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.approximate(&Natural)",
        BenchmarkType::EvaluationStrategy,
        rational_natural_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("n"),
        &mut [
            ("Rational.approximate(&Natural)", &mut |(x, y)| {
                no_out!(x.approximate(&y))
            }),
            ("(&Rational).approximate(&Natural)", &mut |(x, y)| {
                no_out!((&x).approximate(&y))
            }),
        ],
    );
}
