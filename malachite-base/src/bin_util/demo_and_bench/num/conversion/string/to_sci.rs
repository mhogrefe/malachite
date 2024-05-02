// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{
    pair_1_bit_bucketer, signed_bit_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_gen, signed_to_sci_options_pair_gen, signed_to_sci_options_pair_gen_var_1, unsigned_gen,
    unsigned_to_sci_options_pair_gen, unsigned_to_sci_options_pair_gen_var_1,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_to_sci_unsigned);
    register_signed_demos!(runner, demo_to_sci_signed);
    register_unsigned_demos!(runner, demo_fmt_sci_valid_unsigned);
    register_signed_demos!(runner, demo_fmt_sci_valid_signed);
    register_unsigned_demos!(runner, demo_to_sci_with_options_unsigned);
    register_signed_demos!(runner, demo_to_sci_with_options_signed);

    register_unsigned_benches!(runner, benchmark_to_sci_unsigned);
    register_signed_benches!(runner, benchmark_to_sci_signed);
    register_unsigned_benches!(runner, benchmark_fmt_sci_valid_unsigned);
    register_signed_benches!(runner, benchmark_fmt_sci_valid_signed);
    register_unsigned_benches!(runner, benchmark_to_sci_with_options_unsigned);
    register_signed_benches!(runner, benchmark_to_sci_with_options_signed);
}

fn demo_to_sci_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("{}.to_sci() = {}", x, x.to_sci());
    }
}

fn demo_to_sci_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in signed_gen::<T>().get(gm, config).take(limit) {
        println!("{}.to_sci() = {}", x, x.to_sci());
    }
}

fn demo_fmt_sci_valid_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, options) in unsigned_to_sci_options_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.fmt_sci_valid(options) {
            println!("{x} can be converted to sci using {options:?}");
        } else {
            println!("{x} cannot be converted to sci using {options:?}");
        }
    }
}

fn demo_fmt_sci_valid_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in signed_to_sci_options_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.fmt_sci_valid(options) {
            println!("{x} can be converted to sci using {options:?}");
        } else {
            println!("{x} cannot be converted to sci using {options:?}");
        }
    }
}

fn demo_to_sci_with_options_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, options) in unsigned_to_sci_options_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_sci_with_options({}, {:?}) = {}",
            x,
            options,
            x.to_sci_with_options(options)
        );
    }
}

fn demo_to_sci_with_options_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, options) in signed_to_sci_options_pair_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_sci_with_options({}, {:?}) = {}",
            x,
            options,
            x.to_sci_with_options(options)
        );
    }
}

fn benchmark_to_sci_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_sci()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(x.to_sci().to_string()))],
    );
}

fn benchmark_to_sci_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_sci()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(x.to_sci().to_string()))],
    );
}

fn benchmark_fmt_sci_valid_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.fmt_sci_valid(ToSciOptions)", T::NAME),
        BenchmarkType::Single,
        unsigned_to_sci_options_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("u"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(x.fmt_sci_valid(options))
        })],
    );
}

fn benchmark_fmt_sci_valid_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.fmt_sci_valid(ToSciOptions)", T::NAME),
        BenchmarkType::Single,
        signed_to_sci_options_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("i"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(x.fmt_sci_valid(options))
        })],
    );
}

fn benchmark_to_sci_with_options_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_sci_with_options(ToSciOptions)", T::NAME),
        BenchmarkType::Single,
        unsigned_to_sci_options_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("u"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(x.to_sci_with_options(options).to_string())
        })],
    );
}

fn benchmark_to_sci_with_options_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_sci_with_options(ToSciOptions)", T::NAME),
        BenchmarkType::Single,
        signed_to_sci_options_pair_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("u"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(x.to_sci_with_options(options).to_string())
        })],
    );
}
