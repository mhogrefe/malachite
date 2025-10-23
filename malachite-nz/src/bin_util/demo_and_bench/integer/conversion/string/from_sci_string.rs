// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::FromSciString;
use malachite_base::test_util::bench::bucketers::{
    pair_1_string_len_bucketer, string_len_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    string_from_sci_string_options_pair_gen_var_2, string_from_sci_string_options_pair_gen_var_3,
    string_gen_var_14, string_gen_var_15,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_from_sci_string);
    register_demo!(runner, demo_integer_from_sci_string_targeted);
    register_demo!(runner, demo_integer_from_sci_string_with_options);
    register_demo!(runner, demo_integer_from_sci_string_with_options_targeted);

    register_bench!(runner, benchmark_integer_from_sci_string);
    register_bench!(runner, benchmark_integer_from_sci_string_with_options);
}

fn demo_integer_from_sci_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen_var_14().get(gm, config).take(limit) {
        println!(
            "Integer::from_sci_string({}) = {:?}",
            s,
            Integer::from_sci_string(&s)
        );
    }
}

fn demo_integer_from_sci_string_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen_var_15().get(gm, config).take(limit) {
        println!(
            "Integer::from_sci_string({}) = {:?}",
            s,
            Integer::from_sci_string(&s)
        );
    }
}

fn demo_integer_from_sci_string_with_options(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, options) in string_from_sci_string_options_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Integer::from_sci_string_with_options({}, {:?}) = {:?}",
            s,
            options,
            Integer::from_sci_string_with_options(&s, options)
        );
    }
}

fn demo_integer_from_sci_string_with_options_targeted(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (s, options) in string_from_sci_string_options_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Integer::from_sci_string_with_options({}, {:?}) = {:?}",
            s,
            options,
            Integer::from_sci_string_with_options(&s, options)
        );
    }
}

fn benchmark_integer_from_sci_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from_sci_string(&str)",
        BenchmarkType::Single,
        string_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| no_out!(Integer::from_sci_string(&s)))],
    );
}

fn benchmark_integer_from_sci_string_with_options(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::from_sci_string_with_options(&str, FromSciStringOptions)",
        BenchmarkType::Single,
        string_from_sci_string_options_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_string_len_bucketer("s"),
        &mut [("Malachite", &mut |(s, options)| {
            no_out!(Integer::from_sci_string_with_options(&s, options));
        })],
    );
}
