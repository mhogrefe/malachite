// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::big_exponent_float_complexity_bucketer;
use malachite_float::test_util::generators::big_exponent_float_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_neg);
    register_demo!(runner, demo_big_exponent_float_neg_ref);
    register_demo!(runner, demo_big_exponent_float_neg_assign);
    register_demo!(runner, demo_big_exponent_float_neg_debug);
    register_demo!(runner, demo_big_exponent_float_neg_ref_debug);
    register_demo!(runner, demo_big_exponent_float_neg_assign_debug);
    register_bench!(runner, benchmark_big_exponent_float_neg_evaluation_strategy);
}

fn demo_big_exponent_float_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("-{x} = {}", -&x);
    }
}

fn demo_big_exponent_float_neg_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("-{x:#x} = {:#x}", -&x);
    }
}

fn demo_big_exponent_float_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("-(&{n}) = {}", -&n);
    }
}

fn demo_big_exponent_float_neg_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("-(&{:#x}) = {:#x}", &n, -&n);
    }
}

fn demo_big_exponent_float_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in big_exponent_float_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.neg_assign();
        println!("n := {n_old}; n.neg_assign(); n = {n}");
    }
}

fn demo_big_exponent_float_neg_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in big_exponent_float_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.neg_assign();
        println!("n := {n_old:#x}; n.neg_assign(); n = {n:#x}");
    }
}

fn benchmark_big_exponent_float_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "-BigExponentFloat",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("-BigExponentFloat", &mut |x| {
                no_out!(-x);
            }),
            ("-&BigExponentFloat", &mut |x| {
                no_out!(-&x);
            }),
            ("BigExponentFloat.neg_assign()", &mut |mut x| {
                x.neg_assign();
            }),
        ],
    );
}
