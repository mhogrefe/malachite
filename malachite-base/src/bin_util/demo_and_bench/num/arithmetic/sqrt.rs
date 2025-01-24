// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::sqrt::{
    ceiling_sqrt_binary, checked_sqrt_binary, floor_sqrt_binary, sqrt_rem_binary, sqrt_rem_newton,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    primitive_float_bucketer, signed_bit_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_gen, signed_gen_var_2, unsigned_gen, unsigned_gen_var_17,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_floor_sqrt_unsigned);
    register_signed_demos!(runner, demo_floor_sqrt_signed);
    register_unsigned_demos!(runner, demo_floor_sqrt_assign_unsigned);
    register_signed_demos!(runner, demo_floor_sqrt_assign_signed);
    register_unsigned_demos!(runner, demo_ceiling_sqrt_unsigned);
    register_signed_demos!(runner, demo_ceiling_sqrt_signed);
    register_unsigned_demos!(runner, demo_ceiling_sqrt_assign_unsigned);
    register_signed_demos!(runner, demo_ceiling_sqrt_assign_signed);
    register_unsigned_demos!(runner, demo_checked_sqrt_unsigned);
    register_signed_demos!(runner, demo_checked_sqrt_signed);
    register_unsigned_demos!(runner, demo_sqrt_rem);
    register_unsigned_demos!(runner, demo_sqrt_assign_rem);
    register_primitive_float_demos!(runner, demo_sqrt_assign);

    register_unsigned_benches!(runner, benchmark_floor_sqrt_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_floor_sqrt_signed);
    register_unsigned_benches!(runner, benchmark_floor_sqrt_assign_unsigned);
    register_signed_benches!(runner, benchmark_floor_sqrt_assign_signed);
    register_unsigned_benches!(runner, benchmark_ceiling_sqrt_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_ceiling_sqrt_signed);
    register_unsigned_benches!(runner, benchmark_ceiling_sqrt_assign_unsigned);
    register_signed_benches!(runner, benchmark_ceiling_sqrt_assign_signed);
    register_unsigned_benches!(runner, benchmark_checked_sqrt_algorithms_unsigned);
    register_signed_benches!(runner, benchmark_checked_sqrt_signed);
    register_unsigned_benches!(runner, benchmark_sqrt_rem_algorithms);
    register_generic_benches_2_only_first_in_key!(
        runner,
        benchmark_sqrt_rem_algorithms_2,
        [u32, i32],
        [u64, i64]
    );
    register_unsigned_benches!(runner, benchmark_sqrt_assign_rem);
    register_primitive_float_benches!(runner, benchmark_sqrt_assign);
}

fn demo_floor_sqrt_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("floor_sqrt({}) = {}", n, n.floor_sqrt());
    }
}

fn demo_floor_sqrt_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in signed_gen_var_2::<T>().get(gm, config).take(limit) {
        println!("floor_sqrt({}) = {}", n, n.floor_sqrt());
    }
}

fn demo_floor_sqrt_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut n in unsigned_gen::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.floor_sqrt_assign();
        println!("n := {old_n}; n.floor_sqrt_assign(); n = {n}");
    }
}

fn demo_floor_sqrt_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut n in signed_gen_var_2::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.floor_sqrt_assign();
        println!("n := {old_n}; n.floor_sqrt_assign(); n = {n}");
    }
}

fn demo_ceiling_sqrt_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("ceiling_sqrt({}) = {}", n, n.ceiling_sqrt());
    }
}

fn demo_ceiling_sqrt_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in signed_gen_var_2::<T>().get(gm, config).take(limit) {
        println!("ceiling_sqrt({}) = {}", n, n.ceiling_sqrt());
    }
}

fn demo_ceiling_sqrt_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut n in unsigned_gen::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.ceiling_sqrt_assign();
        println!("n := {old_n}; n.ceiling_sqrt_assign(); n = {n}");
    }
}

fn demo_ceiling_sqrt_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut n in signed_gen_var_2::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.ceiling_sqrt_assign();
        println!("n := {old_n}; n.ceiling_sqrt_assign(); n = {n}");
    }
}

fn demo_checked_sqrt_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("checked_sqrt({}) = {:?}", n, n.checked_sqrt());
    }
}

fn demo_checked_sqrt_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in signed_gen_var_2::<T>().get(gm, config).take(limit) {
        println!("checked_sqrt({}) = {:?}", n, n.checked_sqrt());
    }
}

fn demo_sqrt_rem<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen::<T>().get(gm, config).take(limit) {
        let (sqrt, rem) = n.sqrt_rem();
        println!("{n} = {sqrt} ^ 2 + {rem}");
    }
}

fn demo_sqrt_assign_rem<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in unsigned_gen::<T>().get(gm, config).take(limit) {
        let old_n = n;
        let rem = n.sqrt_assign_rem();
        println!("n := {old_n}; n.sqrt_assign() = {rem}; n = {n}");
    }
}

fn demo_sqrt_assign<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        let old_f = f;
        f.sqrt_assign();
        println!(
            "i := {}; i.sqrt_assign(); i = {}",
            NiceFloat(old_f),
            NiceFloat(f)
        );
    }
}

fn benchmark_floor_sqrt_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_sqrt()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.floor_sqrt())),
            ("binary", &mut |n| no_out!(floor_sqrt_binary(n))),
        ],
    );
}

fn benchmark_floor_sqrt_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_sqrt()", T::NAME),
        BenchmarkType::Algorithms,
        signed_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.floor_sqrt()))],
    );
}

fn benchmark_floor_sqrt_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_sqrt_assign()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.floor_sqrt_assign())],
    );
}

fn benchmark_floor_sqrt_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.floor_sqrt_assign()", T::NAME),
        BenchmarkType::Single,
        signed_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.floor_sqrt_assign())],
    );
}

fn benchmark_ceiling_sqrt_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_sqrt()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.ceiling_sqrt())),
            ("binary", &mut |n| no_out!(ceiling_sqrt_binary(n))),
        ],
    );
}

fn benchmark_ceiling_sqrt_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_sqrt()", T::NAME),
        BenchmarkType::Single,
        signed_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.ceiling_sqrt()))],
    );
}

fn benchmark_ceiling_sqrt_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_sqrt_assign()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.ceiling_sqrt_assign())],
    );
}

fn benchmark_ceiling_sqrt_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_sqrt_assign()", T::NAME),
        BenchmarkType::Single,
        signed_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.ceiling_sqrt_assign())],
    );
}

fn benchmark_checked_sqrt_algorithms_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_sqrt()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.checked_sqrt())),
            ("binary", &mut |n| no_out!(checked_sqrt_binary(n))),
        ],
    );
}

fn benchmark_checked_sqrt_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.checked_sqrt()", T::NAME),
        BenchmarkType::Single,
        signed_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(n.checked_sqrt()))],
    );
}

fn benchmark_sqrt_rem_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sqrt_rem()", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(n.sqrt_rem())),
            ("binary", &mut |n| no_out!(sqrt_rem_binary(n))),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_sqrt_rem_algorithms_2<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sqrt_assign_rem()", U::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_17::<U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |n| {
                for _ in 0..10 {
                    n.sqrt_rem().0;
                }
            }),
            ("Newton's method", &mut |n| {
                for _ in 0..10 {
                    sqrt_rem_newton::<U, S>(n).0;
                }
            }),
        ],
    );
}

fn benchmark_sqrt_assign_rem<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sqrt_assign_rem()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| no_out!(n.sqrt_assign_rem()))],
    );
}

fn benchmark_sqrt_assign<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sqrt_assign()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |mut f| f.sqrt_assign())],
    );
}
