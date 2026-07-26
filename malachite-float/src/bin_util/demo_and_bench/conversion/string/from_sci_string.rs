// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{ExactFrom, FromSciString};
use malachite_base::test_util::bench::bucketers::Bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloatRef;
use malachite_float::Float;
use malachite_float::test_util::generators::{
    string_from_sci_string_options_unsigned_triple_gen_var_1,
    string_from_sci_string_options_unsigned_triple_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_from_sci_string);
    register_demo!(runner, demo_float_from_sci_string_targeted);
    register_demo!(runner, demo_float_from_sci_string_prec);
    register_demo!(runner, demo_float_from_sci_string_with_options);
    register_demo!(runner, demo_float_from_sci_string_with_options_prec);
    register_demo!(
        runner,
        demo_float_from_sci_string_with_options_prec_targeted
    );

    register_bench!(runner, benchmark_float_from_sci_string_with_options);
    register_bench!(runner, benchmark_float_from_sci_string_with_options_prec);
}

// The work is driven by whichever is larger, the number of digits read or the number of bits
// produced, so that is what the buckets measure.
fn sci_string_bucketer<'a>() -> Bucketer<'a, (String, FromSciStringOptions, u64)> {
    Bucketer {
        bucketing_function: &|(s, _, prec)| usize::max(s.len(), usize::exact_from(*prec)),
        bucketing_label: "max(s.len(), prec)".to_string(),
    }
}

fn demo_float_from_sci_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, _, _) in string_from_sci_string_options_unsigned_triple_gen_var_2()
        .get(gm, config)
        .filter(|(_, options, _)| options.get_base() == 10)
        .take(limit)
    {
        println!(
            "Float::from_sci_string({s:?}) = {:?}",
            Float::from_sci_string(&s).map(|x| format!("{:#x}", ComparableFloatRef(&x)))
        );
    }
}

fn demo_float_from_sci_string_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, _, _) in string_from_sci_string_options_unsigned_triple_gen_var_1()
        .get(gm, config)
        .filter(|(_, options, _)| options.get_base() == 10)
        .take(limit)
    {
        println!(
            "Float::from_sci_string({s:?}) = {:?}",
            Float::from_sci_string(&s).map(|x| format!("{:#x}", ComparableFloatRef(&x)))
        );
    }
}

fn demo_float_from_sci_string_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, _, prec) in string_from_sci_string_options_unsigned_triple_gen_var_1()
        .get(gm, config)
        .filter(|(_, options, _)| options.get_base() == 10)
        .take(limit)
    {
        println!(
            "Float::from_sci_string_prec({s:?}, {prec}) = {:?}",
            Float::from_sci_string_prec(&s, prec)
                .map(|(x, o)| (format!("{:#x}", ComparableFloatRef(&x)), o))
        );
    }
}

fn demo_float_from_sci_string_with_options(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, options, _) in string_from_sci_string_options_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_sci_string_with_options({s:?}, {}) = {:?}",
            options.get_base(),
            Float::from_sci_string_with_options(&s, options)
                .map(|x| format!("{:#x}", ComparableFloatRef(&x)))
        );
    }
}

fn demo_float_from_sci_string_with_options_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, options, prec) in string_from_sci_string_options_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_sci_string_with_options_prec({s:?}, {}, {}, {prec}) = {:?}",
            options.get_base(),
            options.get_rounding_mode(),
            Float::from_sci_string_with_options_prec(&s, options, prec)
                .map(|(x, o)| (format!("{:#x}", ComparableFloatRef(&x)), o))
        );
    }
}

fn demo_float_from_sci_string_with_options_prec_targeted(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (s, options, prec) in string_from_sci_string_options_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::from_sci_string_with_options_prec({s:?}, {}, {}, {prec}) = {:?}",
            options.get_base(),
            options.get_rounding_mode(),
            Float::from_sci_string_with_options_prec(&s, options, prec)
                .map(|(x, o)| (format!("{:#x}", ComparableFloatRef(&x)), o))
        );
    }
}

fn benchmark_float_from_sci_string_with_options(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_sci_string_with_options(&str, FromSciStringOptions)",
        BenchmarkType::Single,
        string_from_sci_string_options_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &sci_string_bucketer(),
        &mut [("Malachite", &mut |(s, options, _)| {
            no_out!(Float::from_sci_string_with_options(&s, options));
        })],
    );
}

fn benchmark_float_from_sci_string_with_options_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_sci_string_with_options_prec(&str, FromSciStringOptions, u64)",
        BenchmarkType::Single,
        string_from_sci_string_options_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &sci_string_bucketer(),
        &mut [("Malachite", &mut |(s, options, prec)| {
            no_out!(Float::from_sci_string_with_options_prec(&s, options, prec));
        })],
    );
}
