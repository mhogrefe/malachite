// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::kronecker_symbol::{
    jacobi_symbol_unsigned_double_fast_2, jacobi_symbol_unsigned_simple,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{HasHalf, JoinHalves, WrappingFrom};
use malachite_base::test_util::bench::bucketers::{
    pair_max_bit_bucketer, quadruple_max_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_pair_gen, signed_pair_gen_var_8, unsigned_pair_gen_var_27, unsigned_pair_gen_var_40,
    unsigned_quadruple_gen_var_12,
};
use malachite_base::test_util::num::arithmetic::kronecker_symbol::{
    jacobi_symbol_unsigned_double_fast_1, jacobi_symbol_unsigned_double_simple,
    jacobi_symbol_unsigned_fast_1, jacobi_symbol_unsigned_fast_2_1,
    jacobi_symbol_unsigned_fast_2_2, jacobi_symbol_unsigned_fast_2_3,
    jacobi_symbol_unsigned_fast_2_4,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_jacobi_symbol_unsigned_double_fast_1);
    register_unsigned_demos!(runner, demo_jacobi_symbol_unsigned_double_fast_2);
    register_unsigned_demos!(runner, demo_jacobi_symbol_unsigned);
    register_unsigned_signed_match_demos!(runner, demo_jacobi_symbol_signed);
    register_unsigned_demos!(runner, demo_kronecker_symbol_unsigned);
    register_signed_demos!(runner, demo_kronecker_symbol_signed);

    register_generic_benches_2!(
        runner,
        benchmark_jacobi_symbol_unsigned_double_algorithms,
        [u8, u16],
        [u16, u32],
        [u32, u64],
        [u64, u128]
    );
    register_unsigned_benches!(runner, benchmark_jacobi_symbol_unsigned_algorithms);
    register_unsigned_signed_match_benches!(runner, benchmark_jacobi_symbol_signed);
    register_unsigned_benches!(runner, benchmark_kronecker_symbol_unsigned);
    register_signed_benches!(runner, benchmark_kronecker_symbol_signed);
}

fn demo_jacobi_symbol_unsigned_double_fast_1<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x1, x0, y1, y0) in unsigned_quadruple_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "jacobi_symbol_unsigned_double_fast_1({}, {}, {}, {}) = {}",
            x1,
            x0,
            y1,
            y0,
            jacobi_symbol_unsigned_double_fast_1(x1, x0, y1, y0)
        );
    }
}

fn demo_jacobi_symbol_unsigned_double_fast_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x1, x0, y1, y0) in unsigned_quadruple_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "jacobi_symbol_unsigned_double_fast_1({}, {}, {}, {}) = {}",
            x1,
            x0,
            y1,
            y0,
            jacobi_symbol_unsigned_double_fast_2(x1, x0, y1, y0)
        );
    }
}

fn demo_jacobi_symbol_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y) in unsigned_pair_gen_var_40::<T>().get(gm, config).take(limit) {
        println!("{}.jacobi_symbol({}) = {}", x, y, x.jacobi_symbol(y));
    }
}

fn demo_jacobi_symbol_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y) in signed_pair_gen_var_8::<U, S>().get(gm, config).take(limit) {
        println!("({}).jacobi_symbol({}) = {}", x, y, x.jacobi_symbol(y));
    }
}

fn demo_kronecker_symbol_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y) in unsigned_pair_gen_var_27::<T>().get(gm, config).take(limit) {
        println!("{}.kronecker_symbol({}) = {}", x, y, x.kronecker_symbol(y));
    }
}

fn demo_kronecker_symbol_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen::<T>().get(gm, config).take(limit) {
        println!(
            "({}).kronecker_symbol({}) = {}",
            x,
            y,
            x.kronecker_symbol(y)
        );
    }
}

fn benchmark_jacobi_symbol_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.jacobi_symbol({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_40::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.jacobi_symbol(y))),
            ("simple", &mut |(x, y)| {
                no_out!(jacobi_symbol_unsigned_simple(x, y))
            }),
            ("fast 1", &mut |(x, y)| {
                no_out!(jacobi_symbol_unsigned_fast_1(x, y))
            }),
            ("fast 2.1", &mut |(x, y)| {
                no_out!(jacobi_symbol_unsigned_fast_2_1(x, y))
            }),
            ("fast 2.2", &mut |(x, y)| {
                no_out!(jacobi_symbol_unsigned_fast_2_2(x, y))
            }),
            ("fast 2.3", &mut |(x, y)| {
                no_out!(jacobi_symbol_unsigned_fast_2_3(x, y))
            }),
            ("fast 2.4", &mut |(x, y)| {
                no_out!(jacobi_symbol_unsigned_fast_2_4(x, y))
            }),
        ],
    );
}

fn benchmark_jacobi_symbol_unsigned_double_algorithms<
    T: PrimitiveUnsigned,
    D: HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "jacobi_symbol_unsigned_double({}, {}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_quadruple_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_max_bit_bucketer("x1", "x0", "y1", "y0"),
        &mut [
            ("simple", &mut |(x1, x0, y1, y0)| {
                no_out!(jacobi_symbol_unsigned_double_simple::<T, D>(x1, x0, y1, y0))
            }),
            ("fast 1", &mut |(x1, x0, y1, y0)| {
                no_out!(jacobi_symbol_unsigned_double_fast_1(x1, x0, y1, y0))
            }),
            ("fast 2", &mut |(x1, x0, y1, y0)| {
                no_out!(jacobi_symbol_unsigned_double_fast_2(x1, x0, y1, y0))
            }),
        ],
    );
}

fn benchmark_jacobi_symbol_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.jacobi_symbol({})", S::NAME, S::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_8::<U, S>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("default", &mut |(x, y)| no_out!(x.jacobi_symbol(y)))],
    );
}

fn benchmark_kronecker_symbol_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.kronecker_symbol({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_27::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.kronecker_symbol(y)))],
    );
}

fn benchmark_kronecker_symbol_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.kronecker_symbol({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("default", &mut |(x, y)| no_out!(x.kronecker_symbol(y)))],
    );
}
