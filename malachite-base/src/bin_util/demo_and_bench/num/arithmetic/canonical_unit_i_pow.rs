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
    register_unsigned_demos!(runner, demo_canonical_unit_i_pow_unsigned);
    register_signed_demos!(runner, demo_canonical_unit_i_pow_signed);
    register_primitive_float_demos!(runner, demo_canonical_unit_i_pow_primitive_float);

    register_unsigned_benches!(runner, benchmark_canonical_unit_i_pow_unsigned);
    register_signed_benches!(runner, benchmark_canonical_unit_i_pow_signed);
    register_primitive_float_benches!(runner, benchmark_canonical_unit_i_pow_primitive_float);
}

fn demo_canonical_unit_i_pow_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!(
            "{}.canonical_unit_i_pow() = {}",
            u,
            u.canonical_unit_i_pow()
        );
    }
}

fn demo_canonical_unit_i_pow_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for i in signed_gen::<T>().get(gm, config).take(limit) {
        println!(
            "({}).canonical_unit_i_pow() = {}",
            i,
            i.canonical_unit_i_pow()
        );
    }
}

fn demo_canonical_unit_i_pow_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "({}).canonical_unit_i_pow() = {}",
            NiceFloat(f),
            f.canonical_unit_i_pow()
        );
    }
}

fn benchmark_canonical_unit_i_pow_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.canonical_unit_i_pow()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.canonical_unit_i_pow()))],
    );
}

fn benchmark_canonical_unit_i_pow_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.canonical_unit_i_pow()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |i| no_out!(i.canonical_unit_i_pow()))],
    );
}

fn benchmark_canonical_unit_i_pow_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.canonical_unit_i_pow()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(f.canonical_unit_i_pow()))],
    );
}
