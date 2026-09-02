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
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    primitive_float_bucketer, signed_bit_bucketer, unsigned_bit_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{primitive_float_gen, signed_gen, unsigned_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_canonicalize_unit_unsigned);
    register_signed_demos!(runner, demo_canonicalize_unit_signed);
    register_primitive_float_demos!(runner, demo_canonicalize_unit_primitive_float);
    register_unsigned_demos!(runner, demo_canonicalize_unit_assign_unsigned);
    register_signed_demos!(runner, demo_canonicalize_unit_assign_signed);
    register_primitive_float_demos!(runner, demo_canonicalize_unit_assign_primitive_float);

    register_unsigned_benches!(runner, benchmark_canonicalize_unit_unsigned);
    register_signed_benches!(runner, benchmark_canonicalize_unit_signed);
    register_primitive_float_benches!(runner, benchmark_canonicalize_unit_primitive_float);
}

fn demo_canonicalize_unit_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("{}.canonicalize_unit() = {}", u, u.canonicalize_unit());
    }
}

fn demo_canonicalize_unit_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        println!("({}).canonicalize_unit() = {}", i, i.canonicalize_unit());
    }
}

fn demo_canonicalize_unit_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "({}).canonicalize_unit() = {}",
            NiceFloat(f),
            NiceFloat(f.canonicalize_unit())
        );
    }
}

fn demo_canonicalize_unit_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut u in unsigned_gen::<T>().get(gm, config).take(limit) {
        let old_u = u;
        u.canonicalize_unit_assign();
        println!("u := {old_u}; u.canonicalize_unit_assign(); u = {u}");
    }
}

fn demo_canonicalize_unit_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut i in signed_gen::<T>().get(gm, config).take(limit) {
        let old_i = i;
        i.canonicalize_unit_assign();
        println!("i := {old_i}; i.canonicalize_unit_assign(); i = {i}");
    }
}

fn demo_canonicalize_unit_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for mut f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        let old_f = NiceFloat(f);
        f.canonicalize_unit_assign();
        println!(
            "f := {old_f}; f.canonicalize_unit_assign(); f = {}",
            NiceFloat(f)
        );
    }
}

fn benchmark_canonicalize_unit_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.canonicalize_unit()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.canonicalize_unit()))],
    );
}

fn benchmark_canonicalize_unit_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.canonicalize_unit()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |i| no_out!(i.canonicalize_unit()))],
    );
}

fn benchmark_canonicalize_unit_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.canonicalize_unit()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(f.canonicalize_unit()))],
    );
}
