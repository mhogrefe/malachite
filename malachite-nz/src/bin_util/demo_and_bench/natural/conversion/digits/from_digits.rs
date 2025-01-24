// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, Digits, ExactFrom, PowerOf2Digits, SaturatingFrom, WrappingFrom,
};
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_1_vec_len_times_pair_2_bits_bucketer, triple_2_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_unsigned_pair_gen_var_12, unsigned_vec_unsigned_pair_gen_var_5,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::conversion::digits::general_digits::{
    from_digits_asc_large, from_digits_asc_limb, from_digits_desc_basecase, from_digits_desc_large,
    from_digits_desc_limb, from_digits_desc_naive, from_digits_desc_naive_primitive,
    limbs_from_digits_small_base, limbs_from_digits_small_base_basecase,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::bench::bucketers::pair_1_vec_len_times_pair_2_natural_bits_bucketer;
use malachite_nz::test_util::generators::{
    natural_vec_natural_pair_gen_var_1, natural_vec_natural_pair_gen_var_2,
    natural_vec_natural_pair_gen_var_3, natural_vec_natural_pair_gen_var_4,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_3,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_limbs_from_digits_small_base_basecase);
    register_unsigned_demos!(runner, demo_limbs_from_digits_small_base_basecase_targeted);
    register_unsigned_demos!(runner, demo_limbs_from_digits_small_base);
    register_unsigned_demos!(runner, demo_limbs_from_digits_small_base_targeted);
    register_unsigned_demos!(runner, demo_from_digits_desc_basecase);
    register_unsigned_demos!(runner, demo_from_digits_desc_basecase_targeted);
    register_unsigned_demos!(runner, demo_from_digits_asc_limb);
    register_unsigned_demos!(runner, demo_from_digits_desc_limb);
    register_unsigned_demos!(runner, demo_from_digits_asc_limb_targeted);
    register_unsigned_demos!(runner, demo_from_digits_desc_limb_targeted);
    register_demo!(runner, demo_from_digits_asc_large);
    register_demo!(runner, demo_from_digits_desc_large);
    register_demo!(runner, demo_from_digits_asc_large_targeted);
    register_demo!(runner, demo_from_digits_desc_large_targeted);
    register_unsigned_demos!(runner, demo_from_digits_asc_unsigned);
    register_unsigned_demos!(runner, demo_from_digits_desc_unsigned);
    register_unsigned_demos!(runner, demo_from_digits_asc_unsigned_targeted);
    register_unsigned_demos!(runner, demo_from_digits_desc_unsigned_targeted);
    register_demo!(runner, demo_from_digits_asc);
    register_demo!(runner, demo_from_digits_desc);
    register_demo!(runner, demo_from_digits_asc_targeted);
    register_demo!(runner, demo_from_digits_desc_targeted);

    register_unsigned_benches!(
        runner,
        benchmark_limbs_from_digits_small_base_basecase_algorithms
    );
    register_unsigned_benches!(runner, benchmark_limbs_from_digits_small_base_algorithms);
    register_unsigned_benches!(runner, benchmark_from_digits_desc_basecase_algorithms);
    register_unsigned_benches!(runner, benchmark_from_digits_asc_limb);
    register_unsigned_benches!(runner, benchmark_from_digits_desc_limb_algorithms);
    register_bench!(runner, benchmark_from_digits_asc_large);
    register_bench!(runner, benchmark_from_digits_desc_large_algorithms);
    register_unsigned_benches!(runner, benchmark_from_digits_asc_unsigned);
    register_unsigned_benches!(runner, benchmark_from_digits_desc_unsigned_algorithms);
    register_bench!(runner, benchmark_from_digits_asc);
    register_bench!(runner, benchmark_from_digits_desc_algorithms);
}

fn demo_limbs_from_digits_small_base_basecase<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: WrappingFrom<T>,
{
    for (mut out, xs, base) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_3::<T, Limb>()
        .get(gm, config)
        .take(limit)
    {
        let old_out = out.to_vec();
        let out_len = limbs_from_digits_small_base_basecase(&mut out, &xs, base);
        println!(
            "out := {old_out:?}; \
            limbs_from_digits_small_base_basecase(&mut out, {xs:?}, {base}) = {out_len:?}; \
            out = {out:?}",
        );
    }
}

fn demo_limbs_from_digits_small_base_basecase_targeted<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: WrappingFrom<T>,
{
    for (mut out, xs, base) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2::<T, Limb>()
        .get(gm, config)
        .take(limit)
    {
        let old_out = out.to_vec();
        let out_len = limbs_from_digits_small_base_basecase(&mut out, &xs, base).unwrap();
        println!(
            "out := {old_out:?}; \
            limbs_from_digits_small_base_basecase(&mut out, {xs:?}, {base}) = {out_len}; \
            out = {out:?}",
        );
    }
}

fn demo_limbs_from_digits_small_base<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: WrappingFrom<T>,
{
    for (mut out, xs, base) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_3::<T, Limb>()
        .get(gm, config)
        .take(limit)
    {
        let old_out = out.to_vec();
        let out_len = limbs_from_digits_small_base(&mut out, &xs, base);
        println!(
            "out := {old_out:?}; \
            limbs_from_digits_small_base(&mut out, {xs:?}, {base}) = {out_len:?}; out = {out:?}",
        );
    }
}

fn demo_limbs_from_digits_small_base_targeted<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: WrappingFrom<T>,
{
    for (mut out, xs, base) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2::<T, Limb>()
        .get(gm, config)
        .take(limit)
    {
        let old_out = out.to_vec();
        let out_len = limbs_from_digits_small_base(&mut out, &xs, base).unwrap();
        println!(
            "out := {old_out:?}; \
            limbs_from_digits_small_base(&mut out, {xs:?}, {base}) = {out_len}; \
            out = {out:?}",
        );
    }
}

fn demo_from_digits_desc_basecase<T: ConvertibleFrom<Limb> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: SaturatingFrom<T> + WrappingFrom<T>,
{
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_12::<T, Limb>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_digits_desc_basecase(&{:?}, {}) = {:?}",
            xs,
            base,
            from_digits_desc_basecase(&xs, base)
        );
    }
}

fn demo_from_digits_desc_basecase_targeted<T: ConvertibleFrom<Limb> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: SaturatingFrom<T> + WrappingFrom<T>,
{
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_5::<T, Limb>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_digits_desc_basecase(&{:?}, {}) = {}",
            xs,
            base,
            from_digits_desc_basecase(&xs, base).unwrap()
        );
    }
}

fn demo_from_digits_asc_limb<T: ConvertibleFrom<Limb> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: ExactFrom<T> + SaturatingFrom<T> + WrappingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_12::<T, Limb>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_digits_asc_limb(&{:?}, {}) = {:?}",
            xs.clone(),
            base,
            from_digits_asc_limb(xs.into_iter(), base)
        );
    }
}

fn demo_from_digits_desc_limb<T: ConvertibleFrom<Limb> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: ExactFrom<T> + SaturatingFrom<T> + WrappingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_12::<T, Limb>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_digits_desc_limb(&{:?}, {}) = {:?}",
            xs.clone(),
            base,
            from_digits_desc_limb(xs.into_iter(), base)
        );
    }
}

fn demo_from_digits_asc_limb_targeted<T: ConvertibleFrom<Limb> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: ExactFrom<T> + SaturatingFrom<T> + WrappingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_5::<T, Limb>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_digits_asc_limb(&{:?}, {}) = {}",
            xs.clone(),
            base,
            from_digits_asc_limb(xs.into_iter(), base).unwrap()
        );
    }
}

fn demo_from_digits_desc_limb_targeted<T: ConvertibleFrom<Limb> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: ExactFrom<T> + SaturatingFrom<T> + WrappingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_12::<T, Limb>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_digits_desc_limb(&{:?}, {}) = {}",
            xs.clone(),
            base,
            from_digits_desc_limb(xs.into_iter(), base).unwrap()
        );
    }
}

fn demo_from_digits_asc_large(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, base) in natural_vec_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_digits_asc_large(&{:?}, {}) = {:?}",
            xs.clone(),
            base,
            from_digits_asc_large(xs.into_iter(), &base)
        );
    }
}

fn demo_from_digits_desc_large(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, base) in natural_vec_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_digits_desc_large(&{:?}, {}) = {:?}",
            xs.clone(),
            base,
            from_digits_desc_large(xs.into_iter(), &base)
        );
    }
}

fn demo_from_digits_asc_large_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, base) in natural_vec_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_digits_asc_large(&{:?}, {}) = {}",
            xs.clone(),
            base,
            from_digits_asc_large(xs.into_iter(), &base).unwrap()
        );
    }
}

fn demo_from_digits_desc_large_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, base) in natural_vec_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "from_digits_desc_large(&{:?}, {}) = {}",
            xs.clone(),
            base,
            from_digits_desc_large(xs.into_iter(), &base).unwrap()
        );
    }
}

fn demo_from_digits_asc_unsigned<T: PrimitiveUnsigned + SaturatingFrom<T> + WrappingFrom<T>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: SaturatingFrom<T>,
    Natural: Digits<T>,
{
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_digits_asc({}, &{:?}) = {:?}",
            base,
            xs.clone(),
            Natural::from_digits_asc(&base, xs.into_iter())
        );
    }
}

fn demo_from_digits_desc_unsigned<T: PrimitiveUnsigned + SaturatingFrom<T> + WrappingFrom<T>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: SaturatingFrom<T>,
    Natural: Digits<T>,
{
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_digits_desc({}, &{:?}) = {:?}",
            base,
            xs.clone(),
            Natural::from_digits_desc(&base, xs.into_iter())
        );
    }
}

fn demo_from_digits_asc_unsigned_targeted<
    T: PrimitiveUnsigned + SaturatingFrom<T> + WrappingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: SaturatingFrom<T>,
    Natural: Digits<T>,
{
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_5::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_digits_asc({}, &{:?}) = {}",
            base,
            xs.clone(),
            Natural::from_digits_asc(&base, xs.into_iter()).unwrap()
        );
    }
}

fn demo_from_digits_desc_unsigned_targeted<
    T: PrimitiveUnsigned + SaturatingFrom<T> + WrappingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: SaturatingFrom<T>,
    Natural: Digits<T>,
{
    for (xs, base) in unsigned_vec_unsigned_pair_gen_var_5::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_digits_desc({}, &{:?}) = {}",
            base,
            xs.clone(),
            Natural::from_digits_desc(&base, xs.into_iter()).unwrap()
        );
    }
}

fn demo_from_digits_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, base) in natural_vec_natural_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_digits_asc({}, &{:?}) = {:?}",
            base,
            xs.clone(),
            Natural::from_digits_asc(&base, xs.into_iter())
        );
    }
}

fn demo_from_digits_desc(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, base) in natural_vec_natural_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_digits_desc({}, &{:?}) = {:?}",
            base,
            xs.clone(),
            Natural::from_digits_desc(&base, xs.into_iter())
        );
    }
}

fn demo_from_digits_asc_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, base) in natural_vec_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_digits_asc({}, &{:?}) = {}",
            base,
            xs.clone(),
            Natural::from_digits_asc(&base, xs.into_iter()).unwrap()
        );
    }
}

fn demo_from_digits_desc_targeted(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, base) in natural_vec_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::from_digits_desc({}, &{:?}) = {}",
            base,
            xs.clone(),
            Natural::from_digits_desc(&base, xs.into_iter()).unwrap()
        );
    }
}

fn benchmark_limbs_from_digits_small_base_basecase_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Limb: WrappingFrom<T>,
    Natural: From<T>,
{
    run_benchmark(
        &format!(
            "limbs_from_digits_small_base_basecase(&mut [Limb], &[{}], u64)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2::<T, Limb>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs, base)| {
                no_out!(limbs_from_digits_small_base_basecase(&mut out, &xs, base).unwrap())
            }),
            ("naive", &mut |(_, xs, base)| {
                from_digits_desc_naive_primitive(&xs, T::exact_from(base))
                    .unwrap()
                    .into_limbs_asc();
            }),
        ],
    );
}

fn benchmark_limbs_from_digits_small_base_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Limb: WrappingFrom<T>,
    Natural: From<T>,
{
    run_benchmark(
        &format!(
            "limbs_from_digits_small_base(&mut [Limb], &[{}], u64)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2::<T, Limb>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("full", &mut |(mut out, xs, base)| {
                no_out!(limbs_from_digits_small_base(&mut out, &xs, base))
            }),
            ("basecase", &mut |(mut out, xs, base)| {
                no_out!(limbs_from_digits_small_base_basecase(&mut out, &xs, base))
            }),
        ],
    );
}

fn benchmark_from_digits_desc_basecase_algorithms<T: ConvertibleFrom<Limb> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Limb: SaturatingFrom<T> + WrappingFrom<T>,
    Natural: From<T>,
{
    run_benchmark(
        &format!("from_digits_desc_basecase(&[{}], u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_5::<T, Limb>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("default", &mut |(xs, base)| {
                no_out!(from_digits_desc_basecase(&xs, base).unwrap())
            }),
            ("naive", &mut |(xs, base)| {
                from_digits_desc_naive_primitive(&xs, T::exact_from(base))
                    .unwrap()
                    .into_limbs_asc();
            }),
        ],
    );
}

fn benchmark_from_digits_asc_limb<T: ConvertibleFrom<Limb> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Limb: ExactFrom<T> + SaturatingFrom<T> + WrappingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    run_benchmark(
        &format!("from_digits_asc_limb(&[{}], Limb)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_5::<T, Limb>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_times_pair_2_bits_bucketer("xs", "base"),
        &mut [("Malachite", &mut |(xs, base)| {
            no_out!(from_digits_asc_limb(xs.into_iter(), base))
        })],
    );
}

fn benchmark_from_digits_desc_limb_algorithms<T: ConvertibleFrom<Limb> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Limb: ExactFrom<T> + SaturatingFrom<T> + WrappingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    run_benchmark(
        &format!("from_digits_desc_limb(&[{}], Limb)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_5::<T, Limb>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_times_pair_2_bits_bucketer("xs", "base"),
        &mut [
            ("full", &mut |(xs, base)| {
                no_out!(from_digits_desc_limb(xs.into_iter(), base))
            }),
            ("basecase", &mut |(xs, base)| {
                no_out!(from_digits_desc_basecase(&xs, base))
            }),
        ],
    );
}

fn benchmark_from_digits_asc_large(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "from_digits_asc_large(&[Natural], Natural)",
        BenchmarkType::Single,
        natural_vec_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_times_pair_2_natural_bits_bucketer("xs", "base"),
        &mut [("Malachite", &mut |(xs, base)| {
            no_out!(from_digits_desc_large(xs.into_iter(), &base))
        })],
    );
}

fn benchmark_from_digits_desc_large_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "from_digits_desc_large(&[Natural], Natural)",
        BenchmarkType::Algorithms,
        natural_vec_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_times_pair_2_natural_bits_bucketer("xs", "base"),
        &mut [
            ("large", &mut |(xs, base)| {
                no_out!(from_digits_desc_large(xs.into_iter(), &base))
            }),
            ("naive", &mut |(xs, base)| {
                no_out!(from_digits_desc_naive(&xs, &base))
            }),
        ],
    );
}

fn benchmark_from_digits_asc_unsigned<T: PrimitiveUnsigned + SaturatingFrom<T> + WrappingFrom<T>>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: Digits<T>,
{
    run_benchmark(
        &format!(
            "Natural::from_digits_asc({}, Iterator<Item = {}>)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_5::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, base)| {
            no_out!(Natural::from_digits_asc(&base, xs.into_iter()))
        })],
    );
}

fn benchmark_from_digits_desc_unsigned_algorithms<
    T: PrimitiveUnsigned + SaturatingFrom<T> + WrappingFrom<T>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: Digits<T> + From<T>,
{
    run_benchmark(
        &format!(
            "Natural::from_digits_desc({}, Iterator<Item = {}>)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_5::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("default", &mut |(xs, base)| {
                no_out!(Natural::from_digits_desc(&base, xs.into_iter()))
            }),
            ("naive", &mut |(xs, base)| {
                from_digits_desc_naive_primitive(&xs, T::exact_from(base));
            }),
        ],
    );
}

fn benchmark_from_digits_asc(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural::from_digits_asc(&Natural, Iterator<Item = Natural>)",
        BenchmarkType::Single,
        natural_vec_natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_times_pair_2_natural_bits_bucketer("xs", "base"),
        &mut [("Malachite", &mut |(xs, base)| {
            no_out!(Natural::from_digits_asc(&base, xs.into_iter()))
        })],
    );
}

fn benchmark_from_digits_desc_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_digits_desc(&Natural, Iterator<Item = Natural>)",
        BenchmarkType::Algorithms,
        natural_vec_natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_times_pair_2_natural_bits_bucketer("xs", "base"),
        &mut [
            ("default", &mut |(xs, base)| {
                no_out!(Natural::from_digits_desc(&base, xs.into_iter()))
            }),
            ("naive", &mut |(xs, base)| {
                from_digits_desc_naive(&xs, &base);
            }),
        ],
    );
}
