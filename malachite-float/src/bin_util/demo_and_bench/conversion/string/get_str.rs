// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::bench::bucketers::Bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloatRef;
use malachite_float::Float;
use malachite_float::conversion::string::get_str::get_str;
use malachite_float::test_util::generators::{
    float_signed_unsigned_rounding_mode_quadruple_gen_var_9,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_10_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_get_str);
    register_bench!(runner, benchmark_get_str);
    register_bench!(runner, benchmark_get_str_library_comparison);
}

fn demo_get_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, b0, m, rnd) in float_signed_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let cx = ComparableFloatRef(&x);
        match get_str(&x, b0, m, rnd) {
            Some((s, e, o)) => println!(
                "get_str({cx:x}, {b0}, {m}, {rnd}) = ({:?}, {e}, {o:?})",
                String::from_utf8_lossy(&s)
            ),
            None => println!("get_str({cx:x}, {b0}, {m}, {rnd}) = None"),
        }
    }
}

fn benchmark_get_str(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "get_str(&Float, i64, usize, RoundingMode)",
        BenchmarkType::Single,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &Bucketer {
            bucketing_function: &|(x, _, _, _): &(Float, i64, usize, RoundingMode)| {
                usize::try_from(x.get_prec().unwrap_or(0)).unwrap()
            },
            bucketing_label: "x.prec".to_string(),
        },
        &mut [("Malachite", &mut |(x, b0, m, rnd)| {
            no_out!(get_str(&x, b0, m, rnd));
        })],
    );
}

fn benchmark_get_str_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "get_str(&Float, i64, usize, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_10_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &Bucketer {
            bucketing_function: &|(_, (x, _, _, _)): &(
                (rug::Float, i64, usize, rug::float::Round),
                (Float, i64, usize, RoundingMode),
            )| {
                usize::try_from(x.get_prec().unwrap_or(0)).unwrap()
            },
            bucketing_label: "x.prec".to_string(),
        },
        &mut [
            ("Malachite", &mut |(_, (x, b0, m, rnd))| {
                no_out!(get_str(&x, b0, m, rnd));
            }),
            ("rug", &mut |((x, b0, m, round), _)| {
                no_out!(x.to_sign_string_exp_round(
                    i32::exact_from(b0),
                    if m == 0 { None } else { Some(m) },
                    round,
                ));
            }),
        ],
    );
}
