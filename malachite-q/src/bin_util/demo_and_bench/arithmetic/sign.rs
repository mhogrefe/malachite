// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::arithmetic::sign::num_sign;
use malachite_q::test_util::bench::bucketers::triple_3_rational_bit_bucketer;
use malachite_q::test_util::generators::{rational_gen, rational_gen_nrm};
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_sign);
    register_bench!(runner, benchmark_integer_sign_library_comparison);
}

fn demo_integer_sign(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_gen().get(gm, config).take(limit) {
        match x.sign() {
            Less => println!("{x} is negative"),
            Equal => println!("{x} is zero"),
            Greater => println!("{x} is positive"),
        }
    }
}

fn benchmark_integer_sign_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.sign()",
        BenchmarkType::LibraryComparison,
        rational_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| no_out!(x.sign())),
            ("num", &mut |(x, _, _)| no_out!(num_sign(&x))),
            ("rug", &mut |(_, x, _)| no_out!(x.cmp0())),
        ],
    );
}
