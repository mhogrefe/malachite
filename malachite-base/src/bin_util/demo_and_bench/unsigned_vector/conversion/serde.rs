// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{
    string_len_bucketer, unsigned_vector_dimension_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{string_gen, unsigned_vector_gen};
use malachite_base::test_util::runner::Runner;
use malachite_base::unsigned_vector::UnsignedVector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_serialize_json);
    register_demo!(runner, demo_unsigned_vector_deserialize_json);
    register_demo!(runner, demo_unsigned_vector_deserialize_json_targeted);

    register_bench!(runner, benchmark_unsigned_vector_serialize_json);
    register_bench!(runner, benchmark_unsigned_vector_deserialize_json);
}

fn demo_unsigned_vector_serialize_json(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!(
            "serde_json::to_string({}) = {}",
            v,
            serde_json::to_string(&v).unwrap()
        );
    }
}

fn demo_unsigned_vector_deserialize_json(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        let v: Result<UnsignedVector<u64>, _> = serde_json::from_str(&s);
        println!("serde_json::from_str({s}) = {v:?}");
    }
}

fn demo_unsigned_vector_deserialize_json_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        let s = serde_json::to_string(&v).unwrap();
        let w: UnsignedVector<u64> = serde_json::from_str(&s).unwrap();
        println!("serde_json::from_str({s}) = {w}");
    }
}

fn benchmark_unsigned_vector_serialize_json(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "serde_json::to_string(&UnsignedVector<u64>)",
        BenchmarkType::Single,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_vector_dimension_bucketer("v"),
        &mut [("Malachite", &mut |v| {
            no_out!(serde_json::to_string(&v).unwrap());
        })],
    );
}

fn benchmark_unsigned_vector_deserialize_json(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "serde_json::from_str(&str)",
        BenchmarkType::Single,
        Box::new(
            unsigned_vector_gen()
                .get(gm, config)
                .map(|v| serde_json::to_string(&v).unwrap()),
        ),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| {
            let _v: UnsignedVector<u64> = serde_json::from_str(&s).unwrap();
        })],
    );
}
