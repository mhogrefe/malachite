// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_base::test_util::num::logic::bit_convertible::{
    to_bits_asc_alt, to_bits_asc_signed_naive, to_bits_asc_unsigned_naive, to_bits_desc_alt,
    to_bits_desc_signed_naive, to_bits_desc_unsigned_naive,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_to_bits_asc_unsigned);
    register_signed_demos!(runner, demo_to_bits_asc_signed);
    register_unsigned_demos!(runner, demo_to_bits_desc_unsigned);
    register_signed_demos!(runner, demo_to_bits_desc_signed);

    register_unsigned_benches!(runner, benchmark_to_bits_asc_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_to_bits_asc_algorithms_signed);
    register_unsigned_benches!(runner, benchmark_to_bits_asc_evaluation_strategy_unsigned);
    register_signed_benches!(runner, benchmark_to_bits_asc_evaluation_strategy_signed);
    register_unsigned_benches!(runner, benchmark_to_bits_desc_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_to_bits_desc_algorithms_signed);
    register_unsigned_benches!(runner, benchmark_to_bits_desc_evaluation_strategy_unsigned);
    register_signed_benches!(runner, benchmark_to_bits_desc_evaluation_strategy_signed);
}

fn demo_to_bits_asc_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("{}.to_bits_asc() = {:?}", u, u.to_bits_asc());
    }
}

fn demo_to_bits_asc_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        println!("{}.to_bits_asc() = {:?}", i, i.to_bits_asc());
    }
}

fn demo_to_bits_desc_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("{}.to_bits_desc() = {:?}", u, u.to_bits_desc());
    }
}

fn demo_to_bits_desc_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        println!("{}.to_bits_desc() = {:?}", i, i.to_bits_desc());
    }
}

fn benchmark_to_bits_asc_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_bits_asc()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("Malachite", &mut |u| no_out!(u.to_bits_asc())),
            ("alt", &mut |u| no_out!(to_bits_asc_alt(&u))),
            ("naive", &mut |u| no_out!(to_bits_asc_unsigned_naive(u))),
        ],
    );
}

fn benchmark_to_bits_asc_algorithms_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_bits_asc()", T::NAME),
        BenchmarkType::Algorithms,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [
            ("Malachite", &mut |i| no_out!(i.to_bits_asc())),
            ("alt", &mut |i| no_out!(to_bits_asc_alt(&i))),
            ("naive", &mut |i| no_out!(to_bits_asc_signed_naive(i))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_to_bits_asc_evaluation_strategy_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_bits_asc()", T::NAME),
        BenchmarkType::EvaluationStrategy,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            (&format!("{}.to_bits_asc()", T::NAME), &mut |n| {
                no_out!(n.to_bits_asc())
            }),
            (&format!("{}.bits().collect_vec()", T::NAME), &mut |n| {
                no_out!(n.bits().collect_vec())
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_to_bits_asc_evaluation_strategy_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_bits_asc()", T::NAME),
        BenchmarkType::EvaluationStrategy,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [
            (&format!("{}.to_bits_asc()", T::NAME), &mut |n| {
                no_out!(n.to_bits_asc())
            }),
            (&format!("{}.bits().collect_vec()", T::NAME), &mut |n| {
                no_out!(n.bits().collect_vec())
            }),
        ],
    );
}

fn benchmark_to_bits_desc_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_bits_desc()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("Malachite", &mut |u| no_out!(u.to_bits_desc())),
            ("alt", &mut |u| no_out!(to_bits_desc_alt(&u))),
            ("naive", &mut |u| no_out!(to_bits_desc_unsigned_naive(u))),
        ],
    );
}

fn benchmark_to_bits_desc_algorithms_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_bits_desc()", T::NAME),
        BenchmarkType::Algorithms,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [
            ("Malachite", &mut |i| no_out!(i.to_bits_desc())),
            ("alt", &mut |i| no_out!(to_bits_desc_alt(&i))),
            ("naive", &mut |i| no_out!(to_bits_desc_signed_naive(i))),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_to_bits_desc_evaluation_strategy_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_bits_desc()", T::NAME),
        BenchmarkType::EvaluationStrategy,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            (&format!("{}.to_bits_desc()", T::NAME), &mut |n| {
                no_out!(n.to_bits_desc())
            }),
            (
                &format!("{}.bits().rev().collect_vec()", T::NAME),
                &mut |n| no_out!(n.bits().rev().collect_vec()),
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_to_bits_desc_evaluation_strategy_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_bits_desc()", T::NAME),
        BenchmarkType::EvaluationStrategy,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [
            (&format!("{}.to_bits_desc()", T::NAME), &mut |n| {
                no_out!(n.to_bits_desc())
            }),
            (
                &format!("{}.bits().rev().collect_vec()", T::NAME),
                &mut |n| no_out!(n.bits().rev().collect_vec()),
            ),
        ],
    );
}
