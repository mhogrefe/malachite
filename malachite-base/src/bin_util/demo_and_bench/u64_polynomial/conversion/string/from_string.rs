// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::u64_polynomial_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{string_gen, u64_polynomial_gen};
use malachite_base::test_util::runner::Runner;
use malachite_base::u64_polynomial::U64Polynomial;
use malachite_base::vars::VarScheme;
use malachite_base::vars::greek::GreekVars;
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_u64_polynomial_from_str);
    register_demo!(runner, demo_u64_polynomial_from_str_targeted);
    register_demo!(runner, demo_u64_polynomial_from_string_with);
    register_bench!(runner, benchmark_u64_polynomial_from_str);
}

// Almost no random string is a polynomial, so this mostly shows what is rejected.
fn demo_u64_polynomial_from_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen().get(gm, config).take(limit) {
        println!(
            "U64Polynomial::from_str({:?}) = {:?}",
            s,
            U64Polynomial::from_str(&s)
        );
    }
}

// Strings that really are polynomials, so that the reading side is exercised.
fn demo_u64_polynomial_from_str_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in u64_polynomial_gen().get(gm, config).take(limit) {
        let s = p.to_string();
        println!(
            "U64Polynomial::from_str({:?}) = {:?}",
            s,
            U64Polynomial::from_str(&s)
        );
    }
}

fn demo_u64_polynomial_from_string_with(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in u64_polynomial_gen().get(gm, config).take(limit) {
        let s = p.to_string_with(GreekVars.var(0));
        println!(
            "U64Polynomial::from_string_with(α, {:?}) = {:?}",
            s,
            U64Polynomial::from_string_with(GreekVars.var(0), &s)
        );
    }
}

fn benchmark_u64_polynomial_from_str(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "U64Polynomial::from_str(&str)",
        BenchmarkType::Single,
        u64_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &u64_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| {
            no_out!(U64Polynomial::from_str(&p.to_string()).unwrap());
        })],
    );
}
