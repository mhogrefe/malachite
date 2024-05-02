// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, SaturatingFrom};
use malachite_base::test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_gen, signed_gen_var_2, unsigned_gen};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::conversion::from_primitive_int::NaturalFromSignedError;
use malachite_nz::natural::Natural;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_from_unsigned);
    register_signed_demos!(runner, demo_natural_try_from_signed);
    register_signed_demos!(runner, demo_natural_exact_from_signed);
    register_signed_demos!(runner, demo_natural_saturating_from_signed);
    register_signed_demos!(runner, demo_natural_convertible_from_signed);
    register_demo!(runner, demo_natural_const_from);

    register_unsigned_benches!(runner, benchmark_natural_from_unsigned);
    register_signed_benches!(runner, benchmark_natural_try_from_signed);
    register_signed_benches!(runner, benchmark_natural_exact_from_signed);
    register_signed_benches!(runner, benchmark_natural_saturating_from_signed);
    register_signed_benches!(runner, benchmark_natural_convertible_from_signed);
    register_bench!(runner, benchmark_natural_const_from);
}

fn demo_natural_from_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: From<T>,
{
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("Natural::from({}) = {}", u, Natural::from(u));
    }
}

fn demo_natural_try_from_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: TryFrom<T, Error = NaturalFromSignedError>,
{
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        println!("Natural::try_from({}) = {:?}", i, Natural::try_from(i));
    }
}

natural_signed_single_arg_demo_with_trait!(
    demo_natural_exact_from_signed,
    exact_from,
    signed_gen_var_2,
    ExactFrom
);
natural_signed_single_arg_demo_with_trait!(
    demo_natural_saturating_from_signed,
    saturating_from,
    signed_gen,
    SaturatingFrom
);

fn demo_natural_convertible_from_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Natural: ConvertibleFrom<T>,
{
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a Limb",
            i,
            if Natural::convertible_from(i) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_natural_const_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen().get(gm, config).take(limit) {
        println!("Natural::const_from({}) = {}", u, Natural::const_from(u));
    }
}

#[allow(unused_must_use)]
fn benchmark_natural_from_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T>,
{
    run_benchmark(
        &format!("Natural::from({})", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(Natural::from(u)))],
    );
}

fn benchmark_natural_try_from_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: TryFrom<T>,
{
    run_benchmark(
        &format!(concat!("Natural::try_from({})"), T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |x| no_out!(Natural::try_from(x).ok()))],
    );
}

natural_signed_single_arg_bench_with_trait!(
    benchmark_natural_exact_from_signed,
    exact_from,
    signed_gen_var_2,
    ExactFrom
);
natural_signed_single_arg_bench_with_trait!(
    benchmark_natural_saturating_from_signed,
    saturating_from,
    signed_gen,
    SaturatingFrom
);
natural_signed_single_arg_bench_with_trait!(
    benchmark_natural_convertible_from_signed,
    convertible_from,
    signed_gen,
    ConvertibleFrom
);

fn benchmark_natural_const_from(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural::const_from(Limb)",
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(Natural::const_from(u)))],
    );
}
