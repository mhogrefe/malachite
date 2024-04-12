// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

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
    primitive_float_gen, signed_gen_var_10, unsigned_gen_var_21,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_square_unsigned);
    register_unsigned_demos!(runner, demo_square_assign_unsigned);
    register_signed_unsigned_match_demos!(runner, demo_square_signed);
    register_signed_unsigned_match_demos!(runner, demo_square_assign_signed);
    register_primitive_float_demos!(runner, demo_square_primitive_float);
    register_primitive_float_demos!(runner, demo_square_assign_primitive_float);

    register_unsigned_benches!(runner, benchmark_square_unsigned);
    register_unsigned_benches!(runner, benchmark_square_assign_unsigned);
    register_signed_unsigned_match_benches!(runner, benchmark_square_signed);
    register_signed_unsigned_match_benches!(runner, benchmark_square_assign_signed);
    register_primitive_float_benches!(runner, benchmark_square_primitive_float);
    register_primitive_float_benches!(runner, benchmark_square_assign_primitive_float);
}

fn demo_square_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_21::<T>().get(gm, config).take(limit) {
        println!("{}.square() = {}", u, u.square());
    }
}

fn demo_square_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut u in unsigned_gen_var_21::<T>().get(gm, config).take(limit) {
        let old_u = u;
        u.square_assign();
        println!("u := {old_u}; u.square_assign(); u = {u}");
    }
}

fn demo_square_signed<
    S: PrimitiveSigned + WrappingFrom<U>,
    U: PrimitiveUnsigned + WrappingFrom<S>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for i in signed_gen_var_10::<U, S>().get(gm, config).take(limit) {
        println!("{}.square() = {}", i, i.square());
    }
}

fn demo_square_assign_signed<
    S: PrimitiveSigned + WrappingFrom<U>,
    U: PrimitiveUnsigned + WrappingFrom<S>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut i in signed_gen_var_10::<U, S>().get(gm, config).take(limit) {
        let old_i = i;
        i.square_assign();
        println!("i := {old_i}; i.square_assign(); i = {i}");
    }
}

fn demo_square_primitive_float<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!("({}).square() = {}", NiceFloat(f), NiceFloat(f.square()));
    }
}

fn demo_square_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        let old_f = f;
        f.square_assign();
        println!(
            "f := {}; f.square_assign(); x = {}",
            NiceFloat(old_f),
            NiceFloat(f)
        );
    }
}

fn benchmark_square_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.square()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_21::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.square()))],
    );
}

fn benchmark_square_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.square_assign()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_21::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut u| u.square_assign())],
    );
}

fn benchmark_square_signed<
    S: PrimitiveSigned + WrappingFrom<U>,
    U: PrimitiveUnsigned + WrappingFrom<S>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.square()", S::NAME),
        BenchmarkType::Single,
        signed_gen_var_10::<U, S>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |i| no_out!(i.square()))],
    );
}

fn benchmark_square_assign_signed<
    S: PrimitiveSigned + WrappingFrom<U>,
    U: PrimitiveUnsigned + WrappingFrom<S>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.square_assign()", S::NAME),
        BenchmarkType::Single,
        signed_gen_var_10::<U, S>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |mut i| i.square_assign())],
    );
}

fn benchmark_square_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.square()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(f.square()))],
    );
}

fn benchmark_square_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.square_assign()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |mut f| f.square_assign())],
    );
}
