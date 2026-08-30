// Copyright © 2026 Mikhail Hogrefe
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
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_gen, signed_gen_var_10, unsigned_gen_var_21,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_abs_squared_unsigned);
    register_signed_unsigned_match_demos!(runner, demo_abs_squared_signed);
    register_primitive_float_demos!(runner, demo_abs_squared_primitive_float);
    register_unsigned_demos!(runner, demo_abs_squared_assign_unsigned);
    register_signed_unsigned_match_demos!(runner, demo_abs_squared_assign_signed);
    register_primitive_float_demos!(runner, demo_abs_squared_assign_primitive_float);

    register_unsigned_benches!(runner, benchmark_abs_squared_unsigned);
    register_signed_unsigned_match_benches!(runner, benchmark_abs_squared_signed);
    register_primitive_float_benches!(runner, benchmark_abs_squared_primitive_float);
}

fn demo_abs_squared_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_21::<T>().get(gm, config).take(limit) {
        println!("{}.abs_squared() = {}", u, u.abs_squared());
    }
}

fn demo_abs_squared_signed<
    S: PrimitiveSigned + WrappingFrom<U>,
    U: PrimitiveUnsigned + WrappingFrom<S>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for i in signed_gen_var_10::<U, S>().get(gm, config).take(limit) {
        println!("{}.abs_squared() = {}", i, i.abs_squared());
    }
}

fn demo_abs_squared_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "{}.abs_squared() = {}",
            NiceFloat(f),
            NiceFloat(f.abs_squared())
        );
    }
}

fn demo_abs_squared_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut u in unsigned_gen_var_21::<T>().get(gm, config).take(limit) {
        let old_u = u;
        u.abs_squared_assign();
        println!("u := {old_u}; u.abs_squared_assign(); u = {u}");
    }
}

fn demo_abs_squared_assign_signed<
    S: PrimitiveSigned + WrappingFrom<U>,
    U: PrimitiveUnsigned + WrappingFrom<S>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut i in signed_gen_var_10::<U, S>().get(gm, config).take(limit) {
        let old_i = i;
        i.abs_squared_assign();
        println!("i := {old_i}; i.abs_squared_assign(); i = {i}");
    }
}

fn demo_abs_squared_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        let old_f = NiceFloat(f);
        f.abs_squared_assign();
        println!("f := {old_f}; f.abs_squared_assign(); f = {}", NiceFloat(f));
    }
}

fn benchmark_abs_squared_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.abs_squared()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_21::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.abs_squared()))],
    );
}

fn benchmark_abs_squared_signed<
    S: PrimitiveSigned + WrappingFrom<U>,
    U: PrimitiveUnsigned + WrappingFrom<S>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.abs_squared()", S::NAME),
        BenchmarkType::Single,
        signed_gen_var_10::<U, S>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |i| no_out!(i.abs_squared()))],
    );
}

fn benchmark_abs_squared_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.abs_squared()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(f.abs_squared()))],
    );
}
