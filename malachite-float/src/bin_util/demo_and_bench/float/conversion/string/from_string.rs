// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::float_complexity_bucketer;
use malachite_float::test_util::generators::float_gen;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::str::FromStr;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_from_str);
    register_demo!(runner, demo_comparable_float_from_str);
    register_demo!(runner, demo_float_from_string_base);

    register_bench!(runner, benchmark_float_from_str);
    register_bench!(runner, benchmark_float_from_string_base);
}

// The four bases a `ComparableFloat` can be written in.
const BASES: [u8; 4] = [2, 8, 10, 16];

// Renders `x` in `base` the way `ComparableFloat` does, which is what these readers invert.
fn render(x: &Float, base: u8) -> String {
    let c = ComparableFloatRef(x);
    match base {
        2 => format!("{c:#b}"),
        8 => format!("{c:#o}"),
        16 => format!("{c:#x}"),
        _ => format!("{c}"),
    }
}

fn demo_float_from_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let s = format!("{}", ComparableFloatRef(&x));
        println!(
            "Float::from_str({s:?}) = {:?}",
            Float::from_str(&s).map(|x| format!("{:#x}", ComparableFloat(x)))
        );
    }
}

fn demo_comparable_float_from_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let s = format!("{}", ComparableFloatRef(&x));
        println!(
            "ComparableFloat::from_str({s:?}) = {:?}",
            ComparableFloat::from_str(&s).map(|x| format!("{x:#x}"))
        );
    }
}

fn demo_float_from_string_base(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        for base in BASES {
            let s = render(&x, base);
            println!(
                "Float::from_string_base({base}, {s:?}) = {:?}",
                Float::from_string_base(base, &s).map(|x| format!("{:#x}", ComparableFloat(x)))
            );
        }
    }
}

fn benchmark_float_from_str(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float::from_str(&str)",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            let s = format!("{}", ComparableFloatRef(&x));
            no_out!(Float::from_str(&s).ok());
        })],
    );
}

fn benchmark_float_from_string_base(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_string_base(u8, &str)",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            // The hexadecimal form is the one the tests use as their canonical label.
            let s = render(&x, 16);
            no_out!(Float::from_string_base(16, &s));
        })],
    );
}
