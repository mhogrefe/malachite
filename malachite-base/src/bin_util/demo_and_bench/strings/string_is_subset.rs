// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::string_is_subset;
use malachite_base::test_util::bench::bucketers::pair_string_max_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{string_pair_gen, string_pair_gen_var_1};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_string_is_subset);
    register_demo!(runner, demo_string_is_subset_ascii);
    register_bench!(runner, benchmark_string_is_subset);
}

fn demo_string_is_subset(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, t) in string_pair_gen().get(gm, config).take(limit) {
        println!(
            "{:?} is {}a subset of {:?}",
            s,
            if string_is_subset(&s, &t) { "" } else { "not " },
            t
        );
    }
}

fn demo_string_is_subset_ascii(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, t) in string_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "{:?} is {}a subset of {:?}",
            s,
            if string_is_subset(&s, &t) { "" } else { "not " },
            t
        );
    }
}

fn benchmark_string_is_subset(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "string_is_subset(&str, &str)",
        BenchmarkType::Single,
        string_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_string_max_len_bucketer("s", "t"),
        &mut [("Malachite", &mut |(s, t)| no_out!(string_is_subset(&s, &t)))],
    );
}
