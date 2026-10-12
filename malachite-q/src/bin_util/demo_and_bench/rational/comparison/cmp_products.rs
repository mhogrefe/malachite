// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::comparison::traits::{OrdProducts, OrdProductsNeg};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q::test_util::bench::bucketers::quadruple_rational_max_bit_bucketer;
use malachite_q::test_util::generators::{rational_quadruple_gen, rational_triple_gen};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_cmp_products);
    register_demo!(runner, demo_rational_cmp_products_neg);
    register_bench!(runner, benchmark_rational_cmp_products_algorithms);
    register_bench!(runner, benchmark_rational_cmp_products_near_algorithms);
    register_bench!(runner, benchmark_rational_cmp_products_neg_algorithms);
}

fn demo_rational_cmp_products(gm: GenMode, config: &GenConfig, limit: usize) {
    for (w, x, y, z) in rational_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "Rational::cmp_products(&{w}, &{x}, &{y}, &{z}) = {:?}",
            Rational::cmp_products(&w, &x, &y, &z)
        );
    }
}

fn demo_rational_cmp_products_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for (w, x, y, z) in rational_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "Rational::cmp_products_neg(&{w}, &{x}, &{y}, &{z}) = {:?}",
            Rational::cmp_products_neg(&w, &x, &y, &z)
        );
    }
}

fn benchmark_rational_cmp_products_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::cmp_products(&Rational, &Rational, &Rational, &Rational)",
        BenchmarkType::Algorithms,
        rational_quadruple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_rational_max_bit_bucketer("w", "x", "y", "z"),
        &mut [
            ("default", &mut |(w, x, y, z)| {
                no_out!(Rational::cmp_products(&w, &x, &y, &z));
            }),
            ("multiplying first", &mut |(w, x, y, z)| {
                no_out!((&w * &x).cmp(&(&y * &z)));
            }),
        ],
    );
}

// Quadruples `(w, x, y, w * x / y)`, whose products are equal, so that every screen fails and the
// products must be formed.
fn benchmark_rational_cmp_products_near_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::cmp_products(..), near the product",
        BenchmarkType::Algorithms,
        rational_triple_gen()
            .get(gm, config)
            .filter_map(|(w, x, y)| {
                if y == 0u32 {
                    return None;
                }
                let z = &w * &x / &y;
                Some((w, x, y, z))
            }),
        gm.name(),
        limit,
        file_name,
        &quadruple_rational_max_bit_bucketer("w", "x", "y", "z"),
        &mut [
            ("default", &mut |(w, x, y, z)| {
                no_out!(Rational::cmp_products(&w, &x, &y, &z));
            }),
            ("multiplying first", &mut |(w, x, y, z)| {
                no_out!((&w * &x).cmp(&(&y * &z)));
            }),
        ],
    );
}

fn benchmark_rational_cmp_products_neg_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::cmp_products_neg(&Rational, &Rational, &Rational, &Rational)",
        BenchmarkType::Algorithms,
        rational_quadruple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_rational_max_bit_bucketer("w", "x", "y", "z"),
        &mut [
            ("default", &mut |(w, x, y, z)| {
                no_out!(Rational::cmp_products_neg(&w, &x, &y, &z));
            }),
            ("multiplying and negating first", &mut |(w, x, y, z)| {
                no_out!((&w * &x).cmp(&-(&y * &z)));
            }),
        ],
    );
}
