// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::named::Named;
use malachite_base::num::arithmetic::root::{
    cbrt_chebyshev_approx_u32, cbrt_chebyshev_approx_u64, fast_ceiling_root_u32,
    fast_ceiling_root_u64, fast_checked_root_u32, fast_checked_root_u64, fast_floor_cbrt_u32,
    fast_floor_cbrt_u64, fast_floor_root_u32, fast_floor_root_u64, fast_root_rem_u32,
    fast_root_rem_u64, floor_root_approx_and_refine,
};
use malachite_base::num::arithmetic::root::{
    ceiling_root_binary, checked_root_binary, floor_root_binary, root_rem_binary,
};
use malachite_base::num::arithmetic::traits::{CeilingRoot, CheckedRoot, FloorRoot, RootRem};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::{
    pair_1_bit_bucketer, signed_bit_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_gen, signed_unsigned_pair_gen_var_18, unsigned_gen, unsigned_gen_var_1,
    unsigned_pair_gen_var_32,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_floor_cbrt_unsigned);
    register_signed_demos!(runner, demo_floor_cbrt_signed);
    register_unsigned_demos!(runner, demo_floor_cbrt_assign_unsigned);
    register_signed_demos!(runner, demo_floor_cbrt_assign_signed);
    register_unsigned_demos!(runner, demo_ceiling_cbrt_unsigned);
    register_signed_demos!(runner, demo_ceiling_cbrt_signed);
    register_unsigned_demos!(runner, demo_ceiling_cbrt_assign_unsigned);
    register_signed_demos!(runner, demo_ceiling_cbrt_assign_signed);
    register_unsigned_demos!(runner, demo_checked_cbrt_unsigned);
    register_signed_demos!(runner, demo_checked_cbrt_signed);
    register_unsigned_demos!(runner, demo_cbrt_rem);
    register_unsigned_demos!(runner, demo_cbrt_assign_rem);

    register_unsigned_demos!(runner, demo_floor_root_unsigned);
    register_signed_demos!(runner, demo_floor_root_signed);
    register_unsigned_demos!(runner, demo_floor_root_assign_unsigned);
    register_signed_demos!(runner, demo_floor_root_assign_signed);
    register_unsigned_demos!(runner, demo_ceiling_root_unsigned);
    register_signed_demos!(runner, demo_ceiling_root_signed);
    register_unsigned_demos!(runner, demo_ceiling_root_assign_unsigned);
    register_signed_demos!(runner, demo_ceiling_root_assign_signed);
    register_unsigned_demos!(runner, demo_checked_root_unsigned);
    register_signed_demos!(runner, demo_checked_root_signed);
    register_unsigned_demos!(runner, demo_root_rem);
    register_unsigned_demos!(runner, demo_root_assign_rem);

    register_bench!(runner, benchmark_floor_cbrt_algorithms_unsigned_u8);
    register_bench!(runner, benchmark_floor_cbrt_algorithms_unsigned_u16);
    register_bench!(runner, benchmark_floor_cbrt_algorithms_unsigned_u32);
    register_bench!(runner, benchmark_floor_cbrt_algorithms_unsigned_u64);
    register_bench!(runner, benchmark_floor_cbrt_algorithms_unsigned_u128);
    register_bench!(runner, benchmark_floor_cbrt_algorithms_unsigned_usize);

    register_signed_benches!(runner, benchmark_floor_cbrt_signed);
    register_unsigned_benches!(runner, benchmark_floor_cbrt_assign_unsigned);
    register_signed_benches!(runner, benchmark_floor_cbrt_assign_signed);
    register_unsigned_benches!(runner, benchmark_ceiling_cbrt_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_ceiling_cbrt_signed);
    register_unsigned_benches!(runner, benchmark_ceiling_cbrt_assign_unsigned);
    register_signed_benches!(runner, benchmark_ceiling_cbrt_assign_signed);
    register_unsigned_benches!(runner, benchmark_checked_cbrt_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_checked_cbrt_signed);
    register_unsigned_benches!(runner, benchmark_cbrt_rem_algorithms);
    register_unsigned_benches!(runner, benchmark_cbrt_assign_rem);

    register_bench!(runner, benchmark_floor_root_algorithms_unsigned_u8);
    register_bench!(runner, benchmark_floor_root_algorithms_unsigned_u16);
    register_bench!(runner, benchmark_floor_root_algorithms_unsigned_u32);
    register_bench!(runner, benchmark_floor_root_algorithms_unsigned_u64);
    register_bench!(runner, benchmark_floor_root_algorithms_unsigned_u128);
    register_bench!(runner, benchmark_floor_root_algorithms_unsigned_usize);

    register_signed_benches!(runner, benchmark_floor_root_signed);
    register_unsigned_benches!(runner, benchmark_floor_root_assign_unsigned);
    register_signed_benches!(runner, benchmark_floor_root_assign_signed);
    register_unsigned_benches!(runner, benchmark_ceiling_root_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_ceiling_root_signed);
    register_unsigned_benches!(runner, benchmark_ceiling_root_assign_unsigned);
    register_signed_benches!(runner, benchmark_ceiling_root_assign_signed);
    register_unsigned_benches!(runner, benchmark_checked_root_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_checked_root_signed);
    register_unsigned_benches!(runner, benchmark_root_rem_algorithms);
    register_unsigned_benches!(runner, benchmark_root_assign_rem);

    register_bench!(runner, benchmark_floor_cbrt_algorithms_2_u32);
    register_bench!(runner, benchmark_floor_cbrt_algorithms_2_u64);
    register_bench!(runner, benchmark_floor_root_algorithms_2_u32);
    register_bench!(runner, benchmark_floor_root_algorithms_2_u64);
    register_bench!(runner, benchmark_ceiling_root_algorithms_2_u32);
    register_bench!(runner, benchmark_ceiling_root_algorithms_2_u64);
    register_bench!(runner, benchmark_checked_root_algorithms_2_u32);
    register_bench!(runner, benchmark_checked_root_algorithms_2_u64);
    register_bench!(runner, benchmark_root_rem_algorithms_2_u32);
    register_bench!(runner, benchmark_root_rem_algorithms_2_u64);
}

fn demo_floor_cbrt_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("floor_root({}, 3) = {}", n, n.floor_root(3));
    }
}

fn demo_floor_cbrt_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in signed_gen::<T>().get(gm, config).take(limit) {
        println!("floor_root({}, 3) = {}", n, n.floor_root(3));
    }
}

fn demo_floor_cbrt_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut n in unsigned_gen::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.floor_root_assign(3);
        println!("n := {old_n}; n.floor_root_assign(3); n = {n}");
    }
}

fn demo_floor_cbrt_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut n in signed_gen::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.floor_root_assign(3);
        println!("n := {old_n}; n.floor_root_assign(3); n = {n}");
    }
}

fn demo_ceiling_cbrt_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("ceiling_root({}, 3) = {}", n, n.ceiling_root(3));
    }
}

fn demo_ceiling_cbrt_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in signed_gen::<T>().get(gm, config).take(limit) {
        println!("ceiling_root({}, 3) = {}", n, n.ceiling_root(3));
    }
}

fn demo_ceiling_cbrt_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut n in unsigned_gen::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.ceiling_root_assign(3);
        println!("n := {old_n}; n.ceiling_root_assign(3); n = {n}");
    }
}

fn demo_ceiling_cbrt_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut n in signed_gen::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.ceiling_root_assign(3);
        println!("n := {old_n}; n.ceiling_root_assign(3); n = {n}");
    }
}

fn demo_checked_cbrt_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("checked_root({}, 3) = {:?}", n, n.checked_root(3));
    }
}

fn demo_checked_cbrt_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in signed_gen::<T>().get(gm, config).take(limit) {
        println!("checked_root({}, 3) = {:?}", n, n.checked_root(3));
    }
}

fn demo_cbrt_rem<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("root_rem({}, 3) = {:?}", n, n.root_rem(3));
    }
}

fn demo_cbrt_assign_rem<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in unsigned_gen::<T>().get(gm, config).take(limit) {
        let old_n = n;
        let rem = n.root_assign_rem(3);
        println!("n := {old_n}; n.root_assign_rem(3) = {rem}; n = {n}");
    }
}

fn demo_floor_root_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("floor_root({}, {}) = {}", n, exp, n.floor_root(exp));
    }
}

fn demo_floor_root_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, exp) in signed_unsigned_pair_gen_var_18::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("floor_root({}, {}) = {}", n, exp, n.floor_root(exp));
    }
}

fn demo_floor_root_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.floor_root_assign(exp);
        println!("n := {old_n}; n.floor_root_assign({exp}); n = {n}");
    }
}

fn demo_floor_root_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, exp) in signed_unsigned_pair_gen_var_18::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.floor_root_assign(exp);
        println!("n := {old_n}; n.floor_root_assign({exp}); n = {n}");
    }
}

fn demo_ceiling_root_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("ceiling_root({}, {}) = {}", n, exp, n.ceiling_root(exp));
    }
}

fn demo_ceiling_root_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, exp) in signed_unsigned_pair_gen_var_18::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("ceiling_root({}, {}) = {}", n, exp, n.ceiling_root(exp));
    }
}

fn demo_ceiling_root_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.floor_root_assign(exp);
        println!("n := {old_n}; n.ceiling_root_assign({exp}); n = {n}");
    }
}

fn demo_ceiling_root_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, exp) in signed_unsigned_pair_gen_var_18::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.floor_root_assign(exp);
        println!("n := {old_n}; n.ceiling_root_assign({exp}); n = {n}");
    }
}

fn demo_checked_root_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("checked_root({}, {}) = {:?}", n, exp, n.checked_root(exp));
    }
}

fn demo_checked_root_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, exp) in signed_unsigned_pair_gen_var_18::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("checked_root({}, {}) = {:?}", n, exp, n.checked_root(exp));
    }
}

fn demo_root_rem<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("root_rem({}, {}) = {:?}", n, exp, n.root_rem(exp));
    }
}

fn demo_root_assign_rem<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, exp) in unsigned_pair_gen_var_32::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        let rem = n.root_assign_rem(exp);
        println!("n := {old_n}; n.root_assign_rem({exp}) = {rem}; n = {n}");
    }
}

macro_rules! benchmark_floor_cbrt_algorithms_unsigned {
    ($t:ident, $f:ident) => {
        fn $f(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
            #[allow(clippy::cast_lossless)]
            run_benchmark(
                &format!("{}.floor_root(3)", $t::NAME),
                BenchmarkType::Algorithms,
                unsigned_gen::<$t>().get(gm, config),
                gm.name(),
                limit,
                file_name,
                &unsigned_bit_bucketer(),
                &mut [
                    ("default", &mut |n| no_out!(n.floor_root(3))),
                    ("binary", &mut |n| no_out!(floor_root_binary(n, 3))),
                    ("approx and refine", &mut |n| {
                        no_out!(floor_root_approx_and_refine(
                            |x| x as f64,
                            |f| f as $t,
                            n,
                            3
                        ))
                    }),
                ],
            );
        }
    };
}
benchmark_floor_cbrt_algorithms_unsigned!(u8, benchmark_floor_cbrt_algorithms_unsigned_u8);
benchmark_floor_cbrt_algorithms_unsigned!(u16, benchmark_floor_cbrt_algorithms_unsigned_u16);
benchmark_floor_cbrt_algorithms_unsigned!(u32, benchmark_floor_cbrt_algorithms_unsigned_u32);
benchmark_floor_cbrt_algorithms_unsigned!(u64, benchmark_floor_cbrt_algorithms_unsigned_u64);
benchmark_floor_cbrt_algorithms_unsigned!(u128, benchmark_floor_cbrt_algorithms_unsigned_u128);
benchmark_floor_cbrt_algorithms_unsigned!(usize, benchmark_floor_cbrt_algorithms_unsigned_usize);

fn benchmark_floor_cbrt_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_root(3)", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.floor_root(3)))],
    );
}

fn benchmark_floor_cbrt_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_root_assign(3)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.floor_root_assign(3))],
    );
}

fn benchmark_floor_cbrt_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_cbrt_assign(3)", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.floor_root_assign(3))],
    );
}

fn benchmark_ceiling_cbrt_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_root(3)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.ceiling_root(3))),
            ("binary", &mut |n| no_out!(ceiling_root_binary(n, 3))),
        ],
    );
}

fn benchmark_ceiling_cbrt_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_root(3)", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.ceiling_root(3)))],
    );
}

fn benchmark_ceiling_cbrt_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_root_assign(3)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.ceiling_root_assign(3))],
    );
}

fn benchmark_ceiling_cbrt_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_cbrt_assign(3)", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.ceiling_root_assign(3))],
    );
}

fn benchmark_checked_cbrt_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_root(3)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.checked_root(3))),
            ("binary", &mut |n| no_out!(checked_root_binary(n, 3))),
        ],
    );
}

fn benchmark_checked_cbrt_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_roor(3)", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.checked_root(3)))],
    );
}

fn benchmark_cbrt_rem_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.root_rem(3)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.root_rem(3))),
            ("binary", &mut |n| no_out!(root_rem_binary(n, 3))),
        ],
    );
}

fn benchmark_cbrt_assign_rem<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.root_assign_rem(3)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| no_out!(n.root_assign_rem(3)))],
    );
}

macro_rules! benchmark_floor_root_algorithms_unsigned {
    ($t:ident, $f:ident) => {
        fn $f(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
            #[allow(clippy::cast_lossless)]
            run_benchmark(
                &format!("{}.floor_root(u64)", $t::NAME),
                BenchmarkType::Algorithms,
                unsigned_pair_gen_var_32::<$t, u64>().get(gm, config),
                gm.name(),
                limit,
                file_name,
                &pair_1_bit_bucketer("x"),
                &mut [
                    ("default", &mut |(n, exp)| no_out!(n.floor_root(exp))),
                    ("binary", &mut |(n, exp)| no_out!(floor_root_binary(n, exp))),
                    ("approx and refine", &mut |(n, exp)| {
                        no_out!(floor_root_approx_and_refine(
                            |x| x as f64,
                            |f| f as $t,
                            n,
                            exp
                        ))
                    }),
                ],
            );
        }
    };
}
benchmark_floor_root_algorithms_unsigned!(u8, benchmark_floor_root_algorithms_unsigned_u8);
benchmark_floor_root_algorithms_unsigned!(u16, benchmark_floor_root_algorithms_unsigned_u16);
benchmark_floor_root_algorithms_unsigned!(u32, benchmark_floor_root_algorithms_unsigned_u32);
benchmark_floor_root_algorithms_unsigned!(u64, benchmark_floor_root_algorithms_unsigned_u64);
benchmark_floor_root_algorithms_unsigned!(u128, benchmark_floor_root_algorithms_unsigned_u128);
benchmark_floor_root_algorithms_unsigned!(usize, benchmark_floor_root_algorithms_unsigned_usize);

fn benchmark_floor_root_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_root(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_18::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(n, exp)| no_out!(n.floor_root(exp)))],
    );
}

fn benchmark_floor_root_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_root_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| n.floor_root_assign(exp))],
    );
}

fn benchmark_floor_root_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_root_assign(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_18::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| n.floor_root_assign(exp))],
    );
}

fn benchmark_ceiling_root_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_root(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.ceiling_root(exp))),
            ("binary", &mut |(n, exp)| {
                no_out!(ceiling_root_binary(n, exp))
            }),
        ],
    );
}

fn benchmark_ceiling_root_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_root(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_18::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(n, exp)| no_out!(n.ceiling_root(exp)))],
    );
}

fn benchmark_ceiling_root_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_root_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| n.ceiling_root_assign(exp))],
    );
}

fn benchmark_ceiling_root_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_root_assign(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_18::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| n.ceiling_root_assign(exp))],
    );
}

fn benchmark_checked_root_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_root(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.checked_root(exp))),
            ("binary", &mut |(n, exp)| {
                no_out!(checked_root_binary(n, exp))
            }),
        ],
    );
}

fn benchmark_checked_root_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_root(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_18::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(n, exp)| no_out!(n.checked_root(exp)))],
    );
}

fn benchmark_root_rem_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.root_rem(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.root_rem(exp))),
            ("binary", &mut |(n, exp)| no_out!(root_rem_binary(n, exp))),
        ],
    );
}

fn benchmark_root_assign_rem<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.root_assign_rem(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_32::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut n, exp)| {
            no_out!(n.root_assign_rem(exp))
        })],
    );
}

fn benchmark_floor_cbrt_algorithms_2_u32(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32.floor_root(3)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.floor_root(3))),
            ("binary", &mut |n| no_out!(floor_root_binary(n, 3))),
            ("fast", &mut |n| no_out!(fast_floor_cbrt_u32(n))),
            ("Chebyshev", &mut |n| no_out!(cbrt_chebyshev_approx_u32(n))),
            ("approx and refine", &mut |n| {
                no_out!(floor_root_approx_and_refine(f64::from, |f| f as u32, n, 3))
            }),
        ],
    );
}

fn benchmark_floor_cbrt_algorithms_2_u64(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64.floor_root(3)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.floor_root(3))),
            ("binary", &mut |n| no_out!(floor_root_binary(n, 3))),
            ("fast", &mut |n| no_out!(fast_floor_cbrt_u64(n))),
            ("Chebyshev", &mut |n| no_out!(cbrt_chebyshev_approx_u64(n))),
            ("approx and refine", &mut |n| {
                no_out!(floor_root_approx_and_refine(
                    |x| x as f64,
                    |f| f as u64,
                    n,
                    3
                ))
            }),
        ],
    );
}

fn benchmark_floor_root_algorithms_2_u32(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32.floor_root(u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<u32, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.floor_root(exp))),
            ("binary", &mut |(n, exp)| no_out!(floor_root_binary(n, exp))),
            ("fast", &mut |(n, exp)| no_out!(fast_floor_root_u32(n, exp))),
            ("approx and refine", &mut |(n, exp)| {
                no_out!(floor_root_approx_and_refine(
                    f64::from,
                    |f| f as u32,
                    n,
                    exp
                ))
            }),
        ],
    );
}

fn benchmark_floor_root_algorithms_2_u64(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64.floor_root(u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.floor_root(exp))),
            ("binary", &mut |(n, exp)| no_out!(floor_root_binary(n, exp))),
            ("fast", &mut |(n, exp)| no_out!(fast_floor_root_u64(n, exp))),
            ("approx and refine", &mut |(n, exp)| {
                no_out!(floor_root_approx_and_refine(
                    |x| x as f64,
                    |f| f as u64,
                    n,
                    exp
                ))
            }),
        ],
    );
}

fn benchmark_ceiling_root_algorithms_2_u32(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32.ceiling_root(u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<u32, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.ceiling_root(exp))),
            ("binary", &mut |(n, exp)| {
                no_out!(ceiling_root_binary(n, exp))
            }),
            ("fast", &mut |(n, exp)| {
                no_out!(fast_ceiling_root_u32(n, exp))
            }),
        ],
    );
}

fn benchmark_ceiling_root_algorithms_2_u64(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64.ceiling_root(u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.ceiling_root(exp))),
            ("binary", &mut |(n, exp)| {
                no_out!(ceiling_root_binary(n, exp))
            }),
            ("fast", &mut |(n, exp)| {
                no_out!(fast_ceiling_root_u64(n, exp))
            }),
        ],
    );
}

fn benchmark_checked_root_algorithms_2_u32(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32.checked_root(u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<u32, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.checked_root(exp))),
            ("binary", &mut |(n, exp)| {
                no_out!(checked_root_binary(n, exp))
            }),
            ("fast", &mut |(n, exp)| {
                no_out!(fast_checked_root_u32(n, exp))
            }),
        ],
    );
}

fn benchmark_checked_root_algorithms_2_u64(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64.checked_root(u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.checked_root(exp))),
            ("binary", &mut |(n, exp)| {
                no_out!(checked_root_binary(n, exp))
            }),
            ("fast", &mut |(n, exp)| {
                no_out!(fast_checked_root_u64(n, exp))
            }),
        ],
    );
}

fn benchmark_root_rem_algorithms_2_u32(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32.root_rem(u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<u32, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.root_rem(exp))),
            ("binary", &mut |(n, exp)| no_out!(root_rem_binary(n, exp))),
            ("fast", &mut |(n, exp)| no_out!(fast_root_rem_u32(n, exp))),
        ],
    );
}

fn benchmark_root_rem_algorithms_2_u64(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64.root_rem(u64)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_32::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [
            ("default", &mut |(n, exp)| no_out!(n.root_rem(exp))),
            ("binary", &mut |(n, exp)| no_out!(root_rem_binary(n, exp))),
            ("fast", &mut |(n, exp)| no_out!(fast_root_rem_u64(n, exp))),
        ],
    );
}
