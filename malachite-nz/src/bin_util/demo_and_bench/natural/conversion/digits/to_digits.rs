// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, Digits, PowerOf2Digits, SaturatingFrom,
};
use malachite_base::test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, quadruple_3_vec_len_bucketer, triple_3_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::conversion::digits::general_digits::{
    limbs_to_digits_basecase, limbs_to_digits_small_base, limbs_to_digits_small_base_basecase,
    to_digits_asc_large, to_digits_asc_limb, to_digits_asc_naive, to_digits_asc_naive_primitive,
    to_digits_desc_large, to_digits_desc_limb,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_ratio_bucketer, pair_1_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_limbs_to_digits_small_base_basecase);
    register_unsigned_demos!(runner, demo_limbs_to_digits_small_base);
    register_unsigned_demos!(runner, demo_limbs_to_digits_basecase);
    register_unsigned_demos!(runner, demo_to_digits_asc_limb);
    register_unsigned_demos!(runner, demo_to_digits_desc_limb);
    register_demo!(runner, demo_to_digits_asc_large);
    register_demo!(runner, demo_to_digits_desc_large);
    register_unsigned_demos!(runner, demo_to_digits_asc);
    register_unsigned_demos!(runner, demo_to_digits_desc);
    register_demo!(runner, demo_to_digits_asc_natural);
    register_demo!(runner, demo_to_digits_desc_natural);

    register_unsigned_benches!(
        runner,
        benchmark_limbs_to_digits_small_base_basecase_algorithms
    );
    register_unsigned_benches!(
        runner,
        benchmark_limbs_to_digits_small_base_basecase_algorithms_2
    );
    register_unsigned_benches!(runner, benchmark_limbs_to_digits_small_base_algorithms);
    register_unsigned_benches!(runner, benchmark_limbs_to_digits_basecase_algorithms);
    register_unsigned_benches!(runner, benchmark_to_digits_asc_limb);
    register_unsigned_benches!(runner, benchmark_to_digits_desc_limb);
    register_bench!(runner, benchmark_to_digits_asc_large);
    register_bench!(runner, benchmark_to_digits_desc_large);
    register_unsigned_benches!(runner, benchmark_to_digits_asc_algorithms);
    register_unsigned_benches!(runner, benchmark_to_digits_desc);
    register_bench!(runner, benchmark_to_digits_asc_natural_algorithms);
    register_bench!(runner, benchmark_to_digits_desc_natural);
}

fn demo_limbs_to_digits_small_base_basecase<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut out, len, xs, base) in
        unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1::<T>()
            .get(gm, config)
            .take(limit)
    {
        let old_out = out.to_vec();
        let out_len = limbs_to_digits_small_base_basecase(&mut out, len, &xs, base);
        println!(
            "out := {old_out:?}; \
            limbs_to_digits_small_base_basecase(&mut out, {len}, {xs:?}, {base}) = {out_len}; \
            out = {out:?}",
        );
    }
}

fn demo_limbs_to_digits_small_base<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut out, base, mut xs) in unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_out = out.to_vec();
        let out_len = limbs_to_digits_small_base(&mut out, base, &mut xs, None);
        println!(
            "out := {old_out:?}; \
            limbs_to_digits_small_base(&mut out, {base}, {xs:?}) = {out_len}; out = {out:?}",
        );
    }
}

fn demo_limbs_to_digits_basecase<T: ConvertibleFrom<Limb> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: SaturatingFrom<T>,
{
    for (mut xs, base) in unsigned_vec_unsigned_pair_gen_var_4::<Limb, T>()
        .get(gm, config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let mut digits = Vec::new();
        limbs_to_digits_basecase::<T>(&mut digits, &mut xs, base);
        println!(
            "limbs_to_digits_basecase(&mut digits, &{xs_old:?}, {base}); \
            digits = {digits:?}"
        );
    }
}

fn demo_to_digits_asc_limb<
    T: ConvertibleFrom<Limb> + for<'a> TryFrom<&'a Natural> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: Digits<T> + SaturatingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    for (x, base) in natural_unsigned_pair_gen_var_1::<Limb, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_digits_asc_limb({}, {}) = {:?}",
            x,
            base,
            to_digits_asc_limb::<T>(&x, base)
        );
    }
}

fn demo_to_digits_desc_limb<
    T: ConvertibleFrom<Limb> + for<'a> TryFrom<&'a Natural> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Limb: Digits<T> + SaturatingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    for (x, base) in natural_unsigned_pair_gen_var_1::<Limb, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_digits_desc_limb({}, {}) = {:?}",
            x,
            base,
            to_digits_desc_limb::<T>(&x, base)
        );
    }
}

fn demo_to_digits_asc_large(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base) in natural_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "to_digits_asc_large({}, {}) = {:?}",
            x,
            base,
            to_digits_asc_large(&x, &base)
        );
    }
}

fn demo_to_digits_desc_large(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base) in natural_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "to_digits_asc_large({}, {}) = {:?}",
            x,
            base,
            to_digits_desc_large(&x, &base)
        );
    }
}

fn demo_to_digits_asc<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: Digits<T>,
{
    for (x, base) in natural_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_digits_asc({}, {}) = {:?}",
            x,
            base,
            x.to_digits_asc(&base)
        );
    }
}

fn demo_to_digits_desc<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Natural: Digits<T>,
{
    for (x, base) in natural_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_digits_desc({}, {}) = {:?}",
            x,
            base,
            x.to_digits_desc(&base)
        );
    }
}

fn demo_to_digits_asc_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base) in natural_pair_gen_var_2().get(gm, config).take(limit) {
        println!(
            "to_digits_asc({}, {}) = {:?}",
            x,
            base,
            x.to_digits_asc(&base)
        );
    }
}

fn demo_to_digits_desc_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base) in natural_pair_gen_var_2().get(gm, config).take(limit) {
        println!(
            "to_digits_desc({}, {}) = {:?}",
            x,
            base,
            x.to_digits_desc(&base)
        );
    }
}

fn benchmark_limbs_to_digits_small_base_basecase_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "limbs_to_digits_small_base_basecase(&mut [{}], usize, &[Limb], u64)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, len, xs, base)| {
                no_out!(limbs_to_digits_small_base_basecase(
                    &mut out, len, &xs, base
                ))
            }),
            ("naive", &mut |(_, _, xs, base)| {
                let mut digits = Vec::new();
                to_digits_asc_naive_primitive(
                    &mut digits,
                    &Natural::from_owned_limbs_asc(xs),
                    base,
                );
            }),
        ],
    );
}

fn benchmark_limbs_to_digits_small_base_basecase_algorithms_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "limbs_to_digits_small_base_basecase(&mut [{}], usize, &[Limb], u64)",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_vec_len_bucketer("xs"),
        &mut [
            ("limbs_to_digits_small_base_basecase", &mut |(
                mut out,
                _,
                xs,
                base,
            )| {
                no_out!(limbs_to_digits_small_base_basecase(&mut out, 0, &xs, base))
            }),
            ("limbs_to_digits_small_base", &mut |(
                mut out,
                _,
                mut xs,
                base,
            )| {
                no_out!(limbs_to_digits_small_base(&mut out, base, &mut xs, None))
            }),
        ],
    );
}

fn benchmark_limbs_to_digits_small_base_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "limbs_to_digits_small_base(&mut [{}], u64, &[Limb])",
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_vec_len_bucketer("xs"),
        &mut [
            ("default", &mut |(mut out, base, mut xs)| {
                no_out!(limbs_to_digits_small_base(&mut out, base, &mut xs, None))
            }),
            ("naive", &mut |(_, base, xs)| {
                let mut digits = Vec::new();
                to_digits_asc_naive_primitive(
                    &mut digits,
                    &Natural::from_owned_limbs_asc(xs),
                    base,
                );
            }),
        ],
    );
}

fn benchmark_limbs_to_digits_basecase_algorithms<
    T: for<'a> TryFrom<&'a Natural> + ConvertibleFrom<Limb> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Limb: Digits<T> + SaturatingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    run_benchmark(
        "limbs_to_digits_basecase(&mut [Limb], u64)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_pair_gen_var_4::<Limb, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut xs, base)| {
                let mut digits = Vec::new();
                limbs_to_digits_basecase::<T>(&mut digits, &mut xs, base);
            }),
            ("full", &mut |(xs, base)| {
                to_digits_asc_limb::<T>(&Natural::from_owned_limbs_asc(xs), base);
            }),
        ],
    );
}

fn benchmark_to_digits_asc_limb<
    T: for<'a> TryFrom<&'a Natural> + ConvertibleFrom<Limb> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Limb: Digits<T> + SaturatingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    run_benchmark(
        "to_digits_asc_limb(&Natural, Limb)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_1::<Limb, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base)| {
            to_digits_asc_limb::<T>(&x, base);
        })],
    );
}

fn benchmark_to_digits_desc_limb<
    T: for<'a> TryFrom<&'a Natural> + ConvertibleFrom<Limb> + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Limb: Digits<T> + SaturatingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    run_benchmark(
        "to_digits_desc_limb(&Natural, Limb)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_1::<Limb, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base)| {
            to_digits_desc_limb::<T>(&x, base);
        })],
    );
}

fn benchmark_to_digits_asc_large(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "to_digits_asc_large(&Natural, &Natural)",
        BenchmarkType::Single,
        natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_ratio_bucketer("x", "base"),
        &mut [("Malachite", &mut |(x, base)| {
            to_digits_asc_large(&x, &base);
        })],
    );
}

fn benchmark_to_digits_desc_large(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "to_digits_desc_large(&Natural, &Natural)",
        BenchmarkType::Single,
        natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_ratio_bucketer("x", "base"),
        &mut [("Malachite", &mut |(x, base)| {
            to_digits_desc_large(&x, &base);
        })],
    );
}

fn benchmark_to_digits_asc_algorithms<T: for<'a> TryFrom<&'a Natural> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: Digits<T> + From<T>,
{
    run_benchmark(
        &format!("Natural.to_digits_asc(&{})", T::NAME),
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, base)| {
                x.to_digits_asc(&base);
            }),
            ("naive", &mut |(x, base)| {
                let mut digits = Vec::new();
                to_digits_asc_naive_primitive(&mut digits, &x, base);
            }),
        ],
    );
}

fn benchmark_to_digits_desc<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: Digits<T>,
{
    run_benchmark(
        &format!("Natural.to_digits_desc(&{})", T::NAME),
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base)| {
            x.to_digits_desc(&base);
        })],
    );
}

fn benchmark_to_digits_asc_natural_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "to_digits_asc::<Natural, Natural>(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_ratio_bucketer("x", "base"),
        &mut [
            ("default", &mut |(x, base)| no_out!(x.to_digits_asc(&base))),
            ("naive", &mut |(x, base)| {
                let mut digits: Vec<Natural> = Vec::new();
                to_digits_asc_naive(&mut digits, &x, &base);
            }),
        ],
    );
}

fn benchmark_to_digits_desc_natural(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "to_digits_desc::<Natural, Natural>(&Natural, &Natural)",
        BenchmarkType::Single,
        natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_ratio_bucketer("x", "base"),
        &mut [("Malachite", &mut |(x, base)| {
            no_out!(x.to_digits_desc(&base))
        })],
    );
}
