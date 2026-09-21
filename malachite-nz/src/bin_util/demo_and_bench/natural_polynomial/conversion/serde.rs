// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::string_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::string_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::bench::bucketers::natural_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::natural_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_serialize_json);
    register_demo!(runner, demo_natural_polynomial_deserialize_json);
    register_demo!(runner, demo_natural_polynomial_deserialize_json_targeted);

    register_bench!(runner, benchmark_natural_polynomial_serialize_json);
    register_bench!(runner, benchmark_natural_polynomial_deserialize_json);
}

fn demo_natural_polynomial_serialize_json(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "serde_json::to_string({}) = {}",
            p,
            serde_json::to_string(&p).unwrap()
        );
    }
}

fn demo_natural_polynomial_deserialize_json(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        let p: Result<NaturalPolynomial, _> = serde_json::from_str(&s);
        println!("serde_json::from_str({s}) = {p:?}");
    }
}

// An arbitrary string is almost never an encoding of a polynomial, so this demo feeds back what
// serializing produces, which is what a caller will actually be deserializing.
fn demo_natural_polynomial_deserialize_json_targeted(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        let s = serde_json::to_string(&p).unwrap();
        let q: NaturalPolynomial = serde_json::from_str(&s).unwrap();
        println!("serde_json::from_str({s}) = {q}");
    }
}

fn benchmark_natural_polynomial_serialize_json(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "serde_json::to_string(&NaturalPolynomial)",
        BenchmarkType::Single,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| {
            no_out!(serde_json::to_string(&p).unwrap());
        })],
    );
}

fn benchmark_natural_polynomial_deserialize_json(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "serde_json::from_str(&str)",
        BenchmarkType::Single,
        Box::new(
            natural_polynomial_gen()
                .get(gm, config)
                .map(|p| serde_json::to_string(&p).unwrap()),
        ),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| {
            let _p: NaturalPolynomial = serde_json::from_str(&s).unwrap();
        })],
    );
}
