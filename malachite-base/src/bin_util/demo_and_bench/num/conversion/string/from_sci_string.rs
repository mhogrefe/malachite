// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::test_util::bench::bucketers::{
    pair_1_string_len_bucketer, string_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    string_from_sci_string_options_pair_gen, string_from_sci_string_options_pair_gen_var_1,
    string_gen, string_gen_var_13,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_demos!(runner, demo_primitive_int_from_sci_string);
    register_primitive_int_demos!(runner, demo_primitive_int_from_sci_string_targeted);
    register_primitive_int_demos!(runner, demo_primitive_int_from_sci_string_with_options);
    register_primitive_int_demos!(
        runner,
        demo_primitive_int_from_sci_string_with_options_targeted
    );

    register_primitive_int_benches!(runner, benchmark_primitive_int_from_sci_string);
    register_primitive_int_benches!(runner, benchmark_primitive_int_from_sci_string_with_options);
}

fn demo_primitive_int_from_sci_string<T: PrimitiveInt>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for s in string_gen().get(gm, config).take(limit) {
        println!(
            "{}::from_sci_string({}) = {:?}",
            T::NAME,
            s,
            T::from_sci_string(&s)
        );
    }
}

fn demo_primitive_int_from_sci_string_targeted<T: PrimitiveInt>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for s in string_gen_var_13().get(gm, config).take(limit) {
        println!(
            "{}::from_sci_string({}) = {:?}",
            T::NAME,
            s,
            T::from_sci_string(&s)
        );
    }
}

fn demo_primitive_int_from_sci_string_with_options<T: PrimitiveInt>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (s, options) in string_from_sci_string_options_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_sci_string_with_options({}, {:?}) = {:?}",
            T::NAME,
            s,
            options,
            T::from_sci_string_with_options(&s, options)
        );
    }
}

fn demo_primitive_int_from_sci_string_with_options_targeted<T: PrimitiveInt>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (s, options) in string_from_sci_string_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}::from_sci_string_with_options({}, {:?}) = {:?}",
            T::NAME,
            s,
            options,
            T::from_sci_string_with_options(&s, options)
        );
    }
}

fn benchmark_primitive_int_from_sci_string<T: PrimitiveInt>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_sci_string(&str)", T::NAME),
        BenchmarkType::Single,
        string_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &string_len_bucketer(),
        &mut [("Malachite", &mut |s| no_out!(T::from_sci_string(&s)))],
    );
}

fn benchmark_primitive_int_from_sci_string_with_options<T: PrimitiveInt>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}::from_sci_string_with_options(&str, FromSciStringOptions)",
            T::NAME
        ),
        BenchmarkType::Single,
        string_from_sci_string_options_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_string_len_bucketer("s"),
        &mut [("Malachite", &mut |(s, options)| {
            no_out!(T::from_sci_string_with_options(&s, options))
        })],
    );
}
