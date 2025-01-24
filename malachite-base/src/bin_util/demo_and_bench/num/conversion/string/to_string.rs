// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::string::to_string::{
    digit_to_display_byte_lower, digit_to_display_byte_upper, BaseFmtWrapper,
};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::bench::bucketers::{pair_1_bit_bucketer, triple_1_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_5, signed_unsigned_unsigned_triple_gen_var_3, unsigned_gen,
    unsigned_gen_var_7, unsigned_pair_gen_var_8, unsigned_triple_gen_var_6,
};
use malachite_base::test_util::num::conversion::string::to_string::{
    to_string_base_signed_naive, to_string_base_unsigned_naive,
};
use malachite_base::test_util::runner::Runner;
use std::fmt::Display;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_digit_to_display_byte_lower);
    register_demo!(runner, demo_digit_to_display_byte_upper);
    register_demo!(runner, demo_digit_to_display_byte_lower_targeted);
    register_demo!(runner, demo_digit_to_display_byte_upper_targeted);
    register_unsigned_demos!(runner, demo_to_string_base_unsigned);
    register_signed_demos!(runner, demo_to_string_base_signed);
    register_unsigned_demos!(runner, demo_to_string_base_upper_unsigned);
    register_signed_demos!(runner, demo_to_string_base_upper_signed);
    register_unsigned_demos!(runner, demo_base_fmt_wrapper_fmt_unsigned);
    register_unsigned_demos!(runner, demo_base_fmt_wrapper_fmt_with_width_unsigned);
    register_signed_demos!(runner, demo_base_fmt_wrapper_fmt_signed);
    register_signed_demos!(runner, demo_base_fmt_wrapper_fmt_with_width_signed);
    register_unsigned_demos!(runner, demo_base_fmt_wrapper_fmt_upper_unsigned);
    register_unsigned_demos!(runner, demo_base_fmt_wrapper_fmt_upper_with_width_unsigned);
    register_signed_demos!(runner, demo_base_fmt_wrapper_fmt_upper_signed);
    register_signed_demos!(runner, demo_base_fmt_wrapper_fmt_upper_with_width_signed);

    register_unsigned_benches!(runner, benchmark_to_string_base_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_to_string_base_algorithms_signed);
    register_unsigned_benches!(runner, benchmark_to_string_base_upper_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_to_string_base_upper_algorithms_signed);
    register_unsigned_benches!(runner, benchmark_base_fmt_wrapper_fmt_with_width_unsigned);
    register_signed_benches!(runner, benchmark_base_fmt_wrapper_fmt_with_width_signed);
    register_unsigned_benches!(
        runner,
        benchmark_base_fmt_wrapper_fmt_upper_with_width_unsigned
    );
    register_signed_benches!(
        runner,
        benchmark_base_fmt_wrapper_fmt_upper_with_width_signed
    );
}

fn demo_digit_to_display_byte_lower(gm: GenMode, config: &GenConfig, limit: usize) {
    for b in unsigned_gen().get(gm, config).take(limit) {
        println!(
            "digit_to_display_byte_lower({}) = {:?}",
            b,
            digit_to_display_byte_lower(b)
        );
    }
}

fn demo_digit_to_display_byte_upper(gm: GenMode, config: &GenConfig, limit: usize) {
    for b in unsigned_gen().get(gm, config).take(limit) {
        println!(
            "digit_to_display_byte_upper({}) = {:?}",
            b,
            digit_to_display_byte_upper(b)
        );
    }
}

fn demo_digit_to_display_byte_lower_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for b in unsigned_gen_var_7().get(gm, config).take(limit) {
        println!(
            "digit_to_display_byte_lower({}) = {}",
            b,
            digit_to_display_byte_lower(b).unwrap()
        );
    }
}

fn demo_digit_to_display_byte_upper_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for b in unsigned_gen_var_7().get(gm, config).take(limit) {
        println!(
            "digit_to_display_byte_upper({}) = {}",
            b,
            digit_to_display_byte_upper(b).unwrap()
        );
    }
}

fn demo_to_string_base_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base) in unsigned_pair_gen_var_8::<T, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.to_string_base({}) = {}",
            x,
            base,
            x.to_string_base(base)
        );
    }
}

fn demo_to_string_base_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base) in signed_unsigned_pair_gen_var_5::<T, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).to_string_base({}) = {}",
            x,
            base,
            x.to_string_base(base)
        );
    }
}

fn demo_to_string_base_upper_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base) in unsigned_pair_gen_var_8::<T, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.to_string_base_upper({}) = {}",
            x,
            base,
            x.to_string_base_upper(base)
        );
    }
}

fn demo_to_string_base_upper_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base) in signed_unsigned_pair_gen_var_5::<T, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).to_string_base_upper({}) = {}",
            x,
            base,
            x.to_string_base_upper(base)
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_base_fmt_wrapper_fmt_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base) in unsigned_pair_gen_var_8::<T, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "format!(\"{{}}\", BaseFmtWrapper::new({}, {})) = {}",
            x,
            base,
            format!("{}", BaseFmtWrapper::new(x, base))
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_base_fmt_wrapper_fmt_with_width_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base, width) in unsigned_triple_gen_var_6::<T, u8, usize>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "format!(\"{{:0{}}}\", BaseFmtWrapper::new({}, {})) = {}",
            width,
            x,
            base,
            format!("{:0width$}", BaseFmtWrapper::new(x, base), width = width)
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_base_fmt_wrapper_fmt_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base) in signed_unsigned_pair_gen_var_5::<T, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "format!(\"{{}}\", BaseFmtWrapper::new({}, {})) = {}",
            x,
            base,
            format!("{}", BaseFmtWrapper::new(x, base))
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_base_fmt_wrapper_fmt_with_width_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base, width) in signed_unsigned_unsigned_triple_gen_var_3::<T, u8, usize>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "format!(\"{{:0{}}}\", BaseFmtWrapper::new({}, {})) = {}",
            width,
            x,
            base,
            format!("{:0width$}", BaseFmtWrapper::new(x, base), width = width)
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_base_fmt_wrapper_fmt_upper_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base) in unsigned_pair_gen_var_8::<T, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "format!(\"{{:#}}\", BaseFmtWrapper::new({}, {})) = {}",
            x,
            base,
            format!("{:#}", BaseFmtWrapper::new(x, base))
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_base_fmt_wrapper_fmt_upper_with_width_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base, width) in unsigned_triple_gen_var_6::<T, u8, usize>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "format!(\"{{:#0{}}}\", BaseFmtWrapper::new({}, {})) = {}",
            width,
            x,
            base,
            format!("{:#0width$}", BaseFmtWrapper::new(x, base), width = width)
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_base_fmt_wrapper_fmt_upper_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base) in signed_unsigned_pair_gen_var_5::<T, u8>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "format!(\"{{:#}}\", BaseFmtWrapper::new({}, {})) = {}",
            x,
            base,
            format!("{:#}", BaseFmtWrapper::new(x, base))
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_base_fmt_wrapper_fmt_upper_with_width_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base, width) in signed_unsigned_unsigned_triple_gen_var_3::<T, u8, usize>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "format!(\"{{:#0{}}}\", BaseFmtWrapper::new({}, {})) = {}",
            width,
            x,
            base,
            format!("{:#0width$}", BaseFmtWrapper::new(x, base), width = width)
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_to_string_base_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
    u8: WrappingFrom<T>,
{
    run_benchmark(
        &format!("{}.to_string_base(u8)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_8::<T, u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            (
                "to_string",
                &mut |(x, base)| no_out!(x.to_string_base(base)),
            ),
            ("using fmt", &mut |(x, base)| {
                no_out!(format!("{}", BaseFmtWrapper::new(x, base)))
            }),
            ("naive", &mut |(x, base)| {
                no_out!(to_string_base_unsigned_naive(x, base))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_to_string_base_algorithms_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
    u8: WrappingFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    run_benchmark(
        &format!("{}.to_string_base(u8)", T::NAME),
        BenchmarkType::Algorithms,
        signed_unsigned_pair_gen_var_5::<T, u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            (
                "to_string",
                &mut |(x, base)| no_out!(x.to_string_base(base)),
            ),
            ("using fmt", &mut |(x, base)| {
                no_out!(format!("{}", BaseFmtWrapper::new(x, base)))
            }),
            ("naive", &mut |(x, base)| {
                no_out!(to_string_base_signed_naive(x, base))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_to_string_base_upper_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
{
    run_benchmark(
        &format!("{}.to_string_base_upper(u8)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_8::<T, u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("to_string", &mut |(x, base)| {
                no_out!(x.to_string_base_upper(base))
            }),
            ("using fmt", &mut |(x, base)| {
                no_out!(format!("{:#}", BaseFmtWrapper::new(x, base)))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_to_string_base_upper_algorithms_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
{
    run_benchmark(
        &format!("{}.to_string_base_upper(u8)", T::NAME),
        BenchmarkType::Algorithms,
        signed_unsigned_pair_gen_var_5::<T, u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            (
                "to_string",
                &mut |(x, base)| no_out!(x.to_string_base(base)),
            ),
            ("using fmt", &mut |(x, base)| {
                no_out!(format!("{}", BaseFmtWrapper::new(x, base)))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_base_fmt_wrapper_fmt_with_width_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
{
    run_benchmark(
        &format!(
            "format!(\"{{:0usize}}\", BaseFmtWrapper::new({}, u8))",
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_triple_gen_var_6::<T, u8, usize>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base, width)| {
            no_out!(format!(
                "{:0width$}",
                BaseFmtWrapper::new(x, base),
                width = width
            ))
        })],
    );
}

#[allow(unused_must_use)]
fn benchmark_base_fmt_wrapper_fmt_with_width_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
{
    run_benchmark(
        &format!(
            "format!(\"{{:0usize}}\", BaseFmtWrapper::new({}, u8))",
            T::NAME
        ),
        BenchmarkType::Single,
        signed_unsigned_unsigned_triple_gen_var_3::<T, u8, usize>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base, width)| {
            no_out!(format!(
                "{:0width$}",
                BaseFmtWrapper::new(x, base),
                width = width
            ))
        })],
    );
}

#[allow(unused_must_use)]
fn benchmark_base_fmt_wrapper_fmt_upper_with_width_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
{
    run_benchmark(
        &format!(
            "format!(\"{{:#0usize}}\", BaseFmtWrapper::new({}, u8))",
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_triple_gen_var_6::<T, u8, usize>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base, width)| {
            no_out!(format!(
                "{:#0width$}",
                BaseFmtWrapper::new(x, base),
                width = width
            ))
        })],
    );
}

#[allow(unused_must_use)]
fn benchmark_base_fmt_wrapper_fmt_upper_with_width_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
{
    run_benchmark(
        &format!(
            "format!(\"{{:#0usize}}\", BaseFmtWrapper::new({}, u8))",
            T::NAME
        ),
        BenchmarkType::Single,
        signed_unsigned_unsigned_triple_gen_var_3::<T, u8, usize>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base, width)| {
            no_out!(format!(
                "{:#0width$}",
                BaseFmtWrapper::new(x, base),
                width = width
            ))
        })],
    );
}
