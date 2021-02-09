use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::string::to_string::{
    digit_to_display_byte_lower, digit_to_display_byte_upper,
};
use malachite_base::num::conversion::string::BaseFmtWrapper;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base_test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    signed_unsigned_pair_gen_var_5, unsigned_gen_var_7, unsigned_pair_gen_var_8,
};
use malachite_base_test_util::num::conversion::string::to_string::{
    _to_string_base_signed_naive, _to_string_base_unsigned_naive,
};
use malachite_base_test_util::runner::Runner;
use std::fmt::Display;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_digit_to_display_byte_lower);
    register_demo!(runner, demo_digit_to_display_byte_upper);
    register_unsigned_demos!(runner, demo_to_string_base_unsigned);
    register_signed_demos!(runner, demo_to_string_base_signed);
    register_unsigned_demos!(runner, demo_to_string_base_upper_unsigned);
    register_signed_demos!(runner, demo_to_string_base_upper_signed);
    register_unsigned_demos!(runner, demo_base_fmt_wrapper_fmt_unsigned);
    register_signed_demos!(runner, demo_base_fmt_wrapper_fmt_signed);
    register_unsigned_demos!(runner, demo_base_fmt_wrapper_fmt_upper_unsigned);
    register_signed_demos!(runner, demo_base_fmt_wrapper_fmt_upper_signed);

    register_unsigned_benches!(runner, benchmark_to_string_base_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_to_string_base_algorithms_signed);
    register_unsigned_benches!(runner, benchmark_to_string_base_upper_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_to_string_base_upper_algorithms_signed);
}

fn demo_digit_to_display_byte_lower(gm: GenMode, config: GenConfig, limit: usize) {
    for b in unsigned_gen_var_7().get(gm, &config).take(limit) {
        println!(
            "digit_to_display_byte_lower({}) = {}",
            b,
            digit_to_display_byte_lower(b)
        );
    }
}

fn demo_digit_to_display_byte_upper(gm: GenMode, config: GenConfig, limit: usize) {
    for b in unsigned_gen_var_7().get(gm, &config).take(limit) {
        println!(
            "digit_to_display_byte_upper({}) = {}",
            b,
            digit_to_display_byte_upper(b)
        );
    }
}

fn demo_to_string_base_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, base) in unsigned_pair_gen_var_8::<T, u64>()
        .get(gm, &config)
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

fn demo_to_string_base_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, base) in signed_unsigned_pair_gen_var_5::<T, u64>()
        .get(gm, &config)
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
    config: GenConfig,
    limit: usize,
) {
    for (x, base) in unsigned_pair_gen_var_8::<T, u64>()
        .get(gm, &config)
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
    config: GenConfig,
    limit: usize,
) {
    for (x, base) in signed_unsigned_pair_gen_var_5::<T, u64>()
        .get(gm, &config)
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

fn demo_base_fmt_wrapper_fmt_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base) in unsigned_pair_gen_var_8::<T, u64>()
        .get(gm, &config)
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

fn demo_base_fmt_wrapper_fmt_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base) in signed_unsigned_pair_gen_var_5::<T, u64>()
        .get(gm, &config)
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

fn demo_base_fmt_wrapper_fmt_upper_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base) in unsigned_pair_gen_var_8::<T, u64>()
        .get(gm, &config)
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

fn demo_base_fmt_wrapper_fmt_upper_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    BaseFmtWrapper<T>: Display,
{
    for (x, base) in signed_unsigned_pair_gen_var_5::<T, u64>()
        .get(gm, &config)
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

fn benchmark_to_string_base_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
    u8: WrappingFrom<T>,
{
    run_benchmark(
        &format!("{}.to_string_base(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_8::<T, u64>().get(gm, &config),
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
                no_out!(_to_string_base_unsigned_naive(x, base))
            }),
        ],
    );
}

fn benchmark_to_string_base_algorithms_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
    u8: WrappingFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    run_benchmark(
        &format!("{}.to_string_base(u64)", T::NAME),
        BenchmarkType::Algorithms,
        signed_unsigned_pair_gen_var_5::<T, u64>().get(gm, &config),
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
                no_out!(_to_string_base_signed_naive(x, base))
            }),
        ],
    );
}

fn benchmark_to_string_base_upper_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
{
    run_benchmark(
        &format!("{}.to_string_base_upper(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_8::<T, u64>().get(gm, &config),
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

fn benchmark_to_string_base_upper_algorithms_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    BaseFmtWrapper<T>: Display,
{
    run_benchmark(
        &format!("{}.to_string_base_upper(u64)", T::NAME),
        BenchmarkType::Algorithms,
        signed_unsigned_pair_gen_var_5::<T, u64>().get(gm, &config),
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
