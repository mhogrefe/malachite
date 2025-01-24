// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    integer_bit_bucketer, triple_1_integer_bit_bucketer, triple_3_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_gen, integer_gen_nrm, integer_integer_natural_triple_gen,
};
use num::Signed;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_abs);
    register_demo!(runner, demo_integer_abs_ref);
    register_demo!(runner, demo_integer_abs_assign);
    register_demo!(runner, demo_integer_unsigned_abs);
    register_demo!(runner, demo_integer_unsigned_abs_ref);
    register_demo!(runner, demo_integer_unsigned_abs_ref_out);
    register_demo!(runner, demo_integer_mutate_unsigned_abs);

    register_bench!(runner, benchmark_integer_abs_library_comparison);
    register_bench!(runner, benchmark_integer_abs_evaluation_strategy);
    register_bench!(runner, benchmark_integer_abs_assign);
    register_bench!(runner, benchmark_integer_unsigned_abs_evaluation_strategy);
    register_bench!(runner, benchmark_integer_mutate_unsigned_abs);
}

fn demo_integer_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("|{}| = {}", n.clone(), n.abs());
    }
}

fn demo_integer_abs_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("|&{}| = {}", n, (&n).abs());
    }
}

fn demo_integer_abs_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in integer_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.abs_assign();
        println!("n := {n_old}; n.abs_assign(); n = {n}");
    }
}

fn demo_integer_unsigned_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("unsigned_abs({}) = {}", n.clone(), n.unsigned_abs());
    }
}

fn demo_integer_unsigned_abs_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("unsigned_abs(&{}) = {}", n, (&n).unsigned_abs());
    }
}

fn demo_integer_unsigned_abs_ref_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("{}.unsigned_abs_ref() = {}", n, n.unsigned_abs_ref());
    }
}

fn demo_integer_mutate_unsigned_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, out, new_abs) in integer_integer_natural_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n.clone();
        let old_out = out.clone();
        let old_new_abs = new_abs.clone();
        let actual_out = n.mutate_unsigned_abs(|x| {
            *x = new_abs;
            out
        });
        println!(
            "n := {old_n}; \
            n.mutate_unsigned_abs(|x| {{ *x = {old_new_abs}; {old_out} }}) = {actual_out}; \
            n = {n}",
        );
    }
}

fn benchmark_integer_abs_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.abs()",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, n)| no_out!(n.abs())),
            ("num", &mut |(n, _, _)| no_out!(n.abs())),
            ("rug", &mut |(_, n, _)| no_out!(n.abs().cmp0())),
        ],
    );
}

fn benchmark_integer_abs_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.abs()",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("Integer.abs()", &mut |n| no_out!(n.abs())),
            ("(&Integer).abs()", &mut |n| no_out!((&n).abs())),
        ],
    );
}

fn benchmark_integer_abs_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.abs_assign()",
        BenchmarkType::Single,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut n| n.abs_assign())],
    );
}

fn benchmark_integer_unsigned_abs_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.unsigned_abs()",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("Integer.unsigned_abs()", &mut |n| no_out!(n.unsigned_abs())),
            ("(&Integer).unsigned_abs()", &mut |n| {
                no_out!((&n).unsigned_abs())
            }),
            ("Integer.unsigned_abs_ref()", &mut |n| {
                no_out!(n.unsigned_abs_ref())
            }),
        ],
    );
}

fn benchmark_integer_mutate_unsigned_abs(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.mutate_unsigned_abs(FnOnce(&mut Natural) -> T)",
        BenchmarkType::Single,
        integer_integer_natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, out, new_abs)| {
            no_out!(n.mutate_unsigned_abs(|x| {
                *x = new_abs;
                out
            }))
        })],
    );
}
