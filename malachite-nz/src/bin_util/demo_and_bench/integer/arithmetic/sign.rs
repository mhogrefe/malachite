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
use malachite_nz::test_util::bench::bucketers::triple_3_integer_bit_bucketer;
use malachite_nz::test_util::generators::{integer_gen, integer_gen_nrm};
use malachite_nz::test_util::integer::arithmetic::sign::num_sign;
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_sign);
    register_bench!(runner, benchmark_integer_sign_library_comparison);
}

fn demo_integer_sign(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        match n.sign() {
            Less => println!("{n} is negative"),
            Equal => println!("{n} is zero"),
            Greater => println!("{n} is positive"),
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
        "Integer.sign()",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, _, n)| no_out!(n.sign())),
            ("num", &mut |(n, _, _)| no_out!(num_sign(&n))),
            ("rug", &mut |(_, n, _)| no_out!(n.cmp0())),
        ],
    );
}
