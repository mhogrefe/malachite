// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::string_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{string_gen, string_gen_var_8};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    natural_deserialize_bucketer, triple_3_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen, natural_gen_nrm, string_triple_gen_var_1};
use num::BigUint;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_serialize_json);
    register_demo!(runner, demo_natural_deserialize_json);
    register_demo!(runner, demo_natural_deserialize_json_targeted);

    register_bench!(runner, benchmark_natural_serialize_json_library_comparison);
    register_bench!(runner, benchmark_natural_deserialize_json);
    register_bench!(
        runner,
        benchmark_natural_deserialize_json_library_comparison
    );
}

fn demo_natural_serialize_json(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!(
            "serde_json::to_string({}) = {}",
            n,
            serde_json::to_string(&n).unwrap()
        );
    }
}

fn demo_natural_deserialize_json(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        let n: Result<Natural, _> = serde_json::from_str(&s);
        println!("serde_json::from_str({s}) = {n:?}");
    }
}

fn demo_natural_deserialize_json_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen_var_8().get(gm, config).take(limit) {
        let n: Natural = serde_json::from_str(&s).unwrap();
        println!("serde_json::from_str({s}) = {n}");
    }
}

fn benchmark_natural_serialize_json_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "serde_json::to_string(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| {
                no_out!(serde_json::to_string(&x).unwrap())
            }),
            ("num", &mut |(x, _, _)| {
                no_out!(serde_json::to_string(&x).unwrap())
            }),
            ("rug", &mut |(_, x, _)| {
                no_out!(serde_json::to_string(&x).unwrap())
            }),
        ],
    );
}

fn benchmark_natural_deserialize_json(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "serde_json::from_str(&str)",
        BenchmarkType::Single,
        string_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| {
            let _n: Natural = serde_json::from_str(&s).unwrap();
        })],
    );
}

fn benchmark_natural_deserialize_json_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "serde_json::from_str(&str)",
        BenchmarkType::LibraryComparison,
        string_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_deserialize_bucketer(),
        &mut [
            ("Malachite", &mut |(_, _, s)| {
                let _n: Natural = serde_json::from_str(&s).unwrap();
            }),
            ("num", &mut |(s, _, _)| {
                let _n: BigUint = serde_json::from_str(&s).unwrap();
            }),
            ("rug", &mut |(_, s, _)| {
                let _n: rug::Integer = serde_json::from_str(&s).unwrap();
            }),
        ],
    );
}
