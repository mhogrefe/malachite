// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::FromSciString;
use malachite_base::test_util::bench::bucketers::{
    pair_1_string_len_bucketer, string_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    string_gen_var_14, string_gen_var_15, string_unsigned_pair_gen_var_1,
    string_unsigned_pair_gen_var_2,
};
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_from_sci_string);
    register_demo!(runner, demo_rational_from_sci_string_targeted);
    register_demo!(runner, demo_rational_from_sci_string_with_options);
    register_demo!(runner, demo_rational_from_sci_string_with_options_targeted);
    register_demo!(runner, demo_rational_from_sci_string_simplest);
    register_demo!(runner, demo_rational_from_sci_string_simplest_targeted);
    register_demo!(runner, demo_rational_from_sci_string_simplest_with_options);
    register_demo!(
        runner,
        demo_rational_from_sci_string_simplest_with_options_targeted
    );

    register_bench!(runner, benchmark_rational_from_sci_string);
    register_bench!(runner, benchmark_rational_from_sci_string_with_options);
    register_bench!(runner, benchmark_rational_from_sci_string_simplest);
    register_bench!(
        runner,
        benchmark_rational_from_sci_string_simplest_with_options
    );
}

fn demo_rational_from_sci_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen_var_14().get(gm, config).take(limit) {
        println!(
            "Rational::from_sci_string({}) = {:?}",
            s,
            Rational::from_sci_string(&s)
        );
    }
}

fn demo_rational_from_sci_string_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen_var_15().get(gm, config).take(limit) {
        println!(
            "Rational::from_sci_string({}) = {:?}",
            s,
            Rational::from_sci_string(&s)
        );
    }
}

fn demo_rational_from_sci_string_with_options(gm: GenMode, config: &GenConfig, limit: usize) {
    for (s, base) in string_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let mut options = FromSciStringOptions::default();
        options.set_base(base);
        println!(
            "Rational::from_sci_string({}, {}) = {:?}",
            s,
            base,
            Rational::from_sci_string_with_options(&s, options)
        );
    }
}

fn demo_rational_from_sci_string_with_options_targeted(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (s, base) in string_unsigned_pair_gen_var_2().get(gm, config).take(limit) {
        let mut options = FromSciStringOptions::default();
        options.set_base(base);
        println!(
            "Rational::from_sci_string({}, {}) = {:?}",
            s,
            base,
            Rational::from_sci_string_with_options(&s, options)
        );
    }
}

fn demo_rational_from_sci_string_simplest(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen_var_14().get(gm, config).take(limit) {
        println!(
            "Rational::from_sci_string_simplest({}) = {:?}",
            s,
            Rational::from_sci_string_simplest(&s)
        );
    }
}

fn demo_rational_from_sci_string_simplest_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for s in string_gen_var_15().get(gm, config).take(limit) {
        println!(
            "Rational::from_sci_string_simplest({}) = {:?}",
            s,
            Rational::from_sci_string_simplest(&s)
        );
    }
}

fn demo_rational_from_sci_string_simplest_with_options(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (s, base) in string_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let mut options = FromSciStringOptions::default();
        options.set_base(base);
        println!(
            "Rational::from_sci_string_simplest({}, {}) = {:?}",
            s,
            base,
            Rational::from_sci_string_simplest_with_options(&s, options)
        );
    }
}

fn demo_rational_from_sci_string_simplest_with_options_targeted(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (s, base) in string_unsigned_pair_gen_var_2().get(gm, config).take(limit) {
        let mut options = FromSciStringOptions::default();
        options.set_base(base);
        println!(
            "Rational::from_sci_string_simplest({}, {}) = {:?}",
            s,
            base,
            Rational::from_sci_string_simplest_with_options(&s, options)
        );
    }
}

fn benchmark_rational_from_sci_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_sci_string(&str)",
        BenchmarkType::Single,
        string_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| no_out!(Rational::from_sci_string(&s)))],
    );
}

fn benchmark_rational_from_sci_string_with_options(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_sci_string_with_options(&str, FromSciStrOptions)",
        BenchmarkType::Single,
        string_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_string_len_bucketer("s"),
        &mut [("Malachite", &mut |(s, base)| {
            no_out!({
                let mut options = FromSciStringOptions::default();
                options.set_base(base);
                Rational::from_sci_string_with_options(&s, options)
            })
        })],
    );
}

fn benchmark_rational_from_sci_string_simplest(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_sci_string_simplest(&str)",
        BenchmarkType::Single,
        string_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| {
            no_out!(Rational::from_sci_string_simplest(&s))
        })],
    );
}

fn benchmark_rational_from_sci_string_simplest_with_options(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::from_sci_string_simplest_with_options(&str, FromSciStrOptions)",
        BenchmarkType::Single,
        string_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_string_len_bucketer("s"),
        &mut [("Malachite", &mut |(s, base)| {
            no_out!({
                let mut options = FromSciStringOptions::default();
                options.set_base(base);
                Rational::from_sci_string_simplest_with_options(&s, options)
            })
        })],
    );
}
