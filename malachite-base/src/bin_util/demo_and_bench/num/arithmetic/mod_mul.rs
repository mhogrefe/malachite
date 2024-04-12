// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::mod_mul::{
    fast_mod_mul, limbs_invert_limb_u32, limbs_invert_limb_u64, limbs_mod_preinverted,
    naive_mod_mul,
};
use malachite_base::num::arithmetic::traits::ModMulPrecomputed;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{HasHalf, JoinHalves, SplitInHalf};
use malachite_base::test_util::bench::bucketers::{
    ignore_highest_bit_unsigned_bit_bucketer, quadruple_1_2_bit_bucketer, triple_3_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_gen_var_12, unsigned_quadruple_gen_var_5, unsigned_triple_gen_var_12,
};
use malachite_base::test_util::num::arithmetic::mod_mul::limbs_invert_limb_naive;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_invert_limb_u32);
    register_demo!(runner, demo_limbs_invert_limb_u64);
    register_generic_demos_2!(
        runner,
        demo_limbs_mod_preinverted,
        [u8, u16],
        [u16, u32],
        [u32, u64],
        [u64, u128]
    );
    register_unsigned_demos!(runner, demo_mod_mul);
    register_unsigned_demos!(runner, demo_mod_mul_assign);

    register_bench!(runner, benchmark_limbs_invert_limb_u32_algorithms);
    register_bench!(runner, benchmark_limbs_invert_limb_u64_algorithms);
    register_generic_benches_2!(
        runner,
        benchmark_limbs_mod_preinverted_algorithms,
        [u8, u16],
        [u16, u32],
        [u32, u64],
        [u64, u128]
    );
    register_generic_benches!(runner, benchmark_mod_mul_algorithms, u8, u16, u128, usize);
    register_generic_benches_2!(
        runner,
        benchmark_mod_mul_algorithms_with_fast,
        [u32, u64],
        [u64, u128]
    );
    register_unsigned_benches!(runner, benchmark_mod_mul_assign);
    register_unsigned_benches!(runner, benchmark_mod_mul_precomputed_algorithms);
}

fn demo_limbs_invert_limb_u32(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen_var_12().get(gm, config).take(limit) {
        println!(
            "limbs_invert_limb_u32({}) = {}",
            x,
            limbs_invert_limb_u32(x)
        );
    }
}

fn demo_limbs_invert_limb_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen_var_12().get(gm, config).take(limit) {
        println!(
            "limbs_invert_limb_u64({}) = {}",
            x,
            limbs_invert_limb_u64(x)
        );
    }
}

fn demo_limbs_mod_preinverted<
    T: TryFrom<DT> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x_1, x_0, m, inv) in unsigned_quadruple_gen_var_5::<T, DT>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_preinverted({}, {}, {}, {}) = {}",
            x_1,
            x_0,
            m,
            inv,
            limbs_mod_preinverted::<T, DT>(x_1, x_0, m, inv)
        );
    }
}

fn demo_mod_mul<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in unsigned_triple_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{} * {} ≡ {} mod {}", x, y, x.mod_mul(y, m), m);
    }
}

fn demo_mod_mul_assign<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, m) in unsigned_triple_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.mod_mul_assign(y, m);
        println!("x := {old_x}; x.mod_mul_assign({y}, {m}); x = {x}");
    }
}

fn benchmark_limbs_invert_limb_u32_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_invert_limb_u32(u32)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &ignore_highest_bit_unsigned_bit_bucketer("m"),
        &mut [
            ("default", &mut |x| no_out!(limbs_invert_limb_u32(x))),
            ("naive", &mut |x| {
                no_out!(limbs_invert_limb_naive::<u32, u64>(x))
            }),
        ],
    );
}

fn benchmark_limbs_invert_limb_u64_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_invert_limb_u64(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &ignore_highest_bit_unsigned_bit_bucketer("m"),
        &mut [
            ("default", &mut |x| no_out!(limbs_invert_limb_u64(x))),
            ("naive", &mut |x| {
                no_out!(limbs_invert_limb_naive::<u64, u128>(x))
            }),
        ],
    );
}

fn benchmark_limbs_mod_preinverted_algorithms<
    T: TryFrom<DT> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "limbs_mod_preinverted({}, {}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_quadruple_gen_var_5::<T, DT>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x_1, x_0, d, d_inv)| {
                no_out!(limbs_mod_preinverted::<T, DT>(x_1, x_0, d, d_inv))
            }),
            ("naive", &mut |(x_1, x_0, d, _)| {
                no_out!(T::exact_from(DT::join_halves(x_1, x_0) % DT::from(d)))
            }),
        ],
    );
}

fn benchmark_mod_mul_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_triple_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, y, m)| no_out!(x.mod_mul(y, m))),
            ("naive", &mut |(x, y, m)| no_out!(naive_mod_mul(x, y, m))),
        ],
    );
}

fn benchmark_mod_mul_algorithms_with_fast<
    T: ModMulPrecomputed<Data = T> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_triple_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, y, m)| no_out!(x.mod_mul(y, m))),
            ("naive", &mut |(x, y, m)| no_out!(naive_mod_mul(x, y, m))),
            ("fast", &mut |(x, y, m)| {
                no_out!(fast_mod_mul::<T, DT>(
                    x,
                    y,
                    m,
                    T::precompute_mod_mul_data(&m)
                ))
            }),
        ],
    );
}

fn benchmark_mod_mul_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [("Malachite", &mut |(mut x, y, m)| x.mod_mul_assign(y, m))],
    );
}

fn benchmark_mod_mul_precomputed_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_triple_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [
            ("default", &mut |(x, y, m)| {
                for _ in 0..10 {
                    x.mod_mul(y, m);
                }
            }),
            ("precomputed", &mut |(x, y, m)| {
                let data = T::precompute_mod_mul_data(&m);
                for _ in 0..10 {
                    x.mod_mul_precomputed(y, m, &data);
                }
            }),
        ],
    );
}
