// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::{OrdProduct, OrdProductNeg};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q::test_util::bench::bucketers::triple_rational_max_bit_bucketer;
use malachite_q::test_util::generators::{rational_pair_gen, rational_triple_gen};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_cmp_product);
    register_demo!(runner, demo_rational_cmp_product_neg);
    register_bench!(runner, benchmark_rational_cmp_product_algorithms);
    register_bench!(runner, benchmark_rational_cmp_product_near_algorithms);
    register_bench!(runner, benchmark_rational_cmp_product_neg_algorithms);
}

fn demo_rational_cmp_product(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in rational_triple_gen().get(gm, config).take(limit) {
        println!(
            "Rational::cmp_product(&{x}, &{y}, &{z}) = {:?}",
            Rational::cmp_product(&x, &y, &z)
        );
    }
}

fn demo_rational_cmp_product_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in rational_triple_gen().get(gm, config).take(limit) {
        println!(
            "Rational::cmp_product_neg(&{x}, &{y}, &{z}) = {:?}",
            Rational::cmp_product_neg(&x, &y, &z)
        );
    }
}

fn benchmark_rational_cmp_product_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::cmp_product(&Rational, &Rational, &Rational)",
        BenchmarkType::Algorithms,
        rational_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_rational_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("default", &mut |(x, y, z)| {
                no_out!(Rational::cmp_product(&x, &y, &z));
            }),
            ("multiplying first", &mut |(x, y, z)| {
                no_out!((&x * &y).cmp(&z));
            }),
        ],
    );
}

// Triples `(x, y, x * y)`, where every screen fails and the products must be formed.
fn benchmark_rational_cmp_product_near_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::cmp_product(..), near the product",
        BenchmarkType::Algorithms,
        rational_pair_gen().get(gm, config).map(|(x, y)| {
            let z = &x * &y;
            (x, y, z)
        }),
        gm.name(),
        limit,
        file_name,
        &triple_rational_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("default", &mut |(x, y, z)| {
                no_out!(Rational::cmp_product(&x, &y, &z));
            }),
            ("multiplying first", &mut |(x, y, z)| {
                no_out!((&x * &y).cmp(&z));
            }),
        ],
    );
}

fn benchmark_rational_cmp_product_neg_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::cmp_product_neg(&Rational, &Rational, &Rational)",
        BenchmarkType::Algorithms,
        rational_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_rational_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("default", &mut |(x, y, z)| {
                no_out!(Rational::cmp_product_neg(&x, &y, &z));
            }),
            ("negating, then multiplying", &mut |(x, y, z)| {
                no_out!((&x * &y).cmp(&-z));
            }),
        ],
    );
}
