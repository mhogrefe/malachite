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
use malachite_float::conversion::string::strtofr::{set_str, strtofr};
use malachite_float::test_util::generators::{
    string_unsigned_unsigned_rounding_mode_quadruple_gen_var_1,
    string_unsigned_unsigned_rounding_mode_quadruple_gen_var_2_rm,
    string_unsigned_unsigned_rounding_mode_quadruple_gen_var_3,
};
use rug::ops::CompleteRound;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_strtofr);
    register_demo!(runner, demo_strtofr_targeted);
    register_demo!(runner, demo_set_str);
    register_demo!(runner, demo_set_str_targeted);

    register_bench!(runner, benchmark_strtofr);
    register_bench!(runner, benchmark_set_str);
    register_bench!(runner, benchmark_set_str_library_comparison);
}

// The work is driven by whichever is larger, the number of digits read or the number of bits
// produced, so that is what the buckets measure.
fn strtofr_bucketer<'a>() -> Bucketer<'a, (String, u8, u64, RoundingMode)> {
    Bucketer {
        bucketing_function: &|(s, _, prec, _)| usize::max(s.len(), usize::exact_from(*prec)),
        bucketing_label: "max(s.len(), prec)".to_string(),
    }
}

fn strtofr_rm_bucketer<'a>() -> Bucketer<
    'a,
    (
        (String, i32, u32, rug::float::Round),
        (String, u8, u64, RoundingMode),
    ),
> {
    Bucketer {
        bucketing_function: &|(_, (s, _, prec, _))| usize::max(s.len(), usize::exact_from(*prec)),
        bucketing_label: "max(s.len(), prec)".to_string(),
    }
}

fn demo_strtofr(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, base, prec, rm) in string_unsigned_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (x, o, len) = strtofr(&s, base, prec, rm);
        println!(
            "strtofr({s:?}, {base}, {prec}, {rm}) = ({:#x}, {o:?}, {len})",
            ComparableFloatRef(&x)
        );
    }
}

fn demo_strtofr_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, base, prec, rm) in string_unsigned_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (x, o, len) = strtofr(&s, base, prec, rm);
        println!(
            "strtofr({s:?}, {base}, {prec}, {rm}) = ({:#x}, {o:?}, {len})",
            ComparableFloatRef(&x)
        );
    }
}

fn demo_set_str(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, base, prec, rm) in string_unsigned_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        match set_str(&s, base, prec, rm) {
            Some((x, o)) => println!(
                "set_str({s:?}, {base}, {prec}, {rm}) = Some(({:#x}, {o:?}))",
                ComparableFloatRef(&x)
            ),
            None => println!("set_str({s:?}, {base}, {prec}, {rm}) = None"),
        }
    }
}

fn demo_set_str_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, base, prec, rm) in string_unsigned_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        match set_str(&s, base, prec, rm) {
            Some((x, o)) => println!(
                "set_str({s:?}, {base}, {prec}, {rm}) = Some(({:#x}, {o:?}))",
                ComparableFloatRef(&x)
            ),
            None => println!("set_str({s:?}, {base}, {prec}, {rm}) = None"),
        }
    }
}

fn benchmark_strtofr(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "strtofr(&str, u8, u64, RoundingMode)",
        BenchmarkType::Single,
        string_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &strtofr_bucketer(),
        &mut [("Malachite", &mut |(s, base, prec, rm)| {
            no_out!(strtofr(&s, base, prec, rm));
        })],
    );
}

fn benchmark_set_str(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "set_str(&str, u8, u64, RoundingMode)",
        BenchmarkType::Single,
        string_unsigned_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &strtofr_bucketer(),
        &mut [("Malachite", &mut |(s, base, prec, rm)| {
            no_out!(set_str(&s, base, prec, rm));
        })],
    );
}

fn benchmark_set_str_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "set_str(&str, u8, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        string_unsigned_unsigned_rounding_mode_quadruple_gen_var_2_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &strtofr_rm_bucketer(),
        &mut [
            ("Malachite", &mut |(_, (s, base, prec, rm))| {
                no_out!(set_str(&s, base, prec, rm));
            }),
            ("rug", &mut |((s, base, prec, round), _)| {
                no_out!(
                    rug::Float::parse_radix(&s, base)
                        .unwrap()
                        .complete_round(prec, round)
                );
            }),
        ],
    );
}
