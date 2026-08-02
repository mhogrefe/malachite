// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::quadruple_max_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_quadruple_gen, unsigned_quadruple_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_overflowing_mul_add_mul_unsigned);
    register_signed_demos!(runner, demo_overflowing_mul_add_mul_signed);
    register_unsigned_demos!(runner, demo_overflowing_mul_add_mul_assign_unsigned);
    register_signed_demos!(runner, demo_overflowing_mul_add_mul_assign_signed);
    register_unsigned_benches!(runner, benchmark_overflowing_mul_add_mul_unsigned);
    register_signed_benches!(runner, benchmark_overflowing_mul_add_mul_signed);
}

fn demo_overflowing_mul_add_mul_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, w) in unsigned_quadruple_gen::<T>().get(gm, config).take(limit) {
        println!(
            "{}.overflowing_mul_add_mul({}, {}, {}) = {:?}",
            x,
            y,
            z,
            w,
            x.overflowing_mul_add_mul(y, z, w)
        );
    }
}

fn demo_overflowing_mul_add_mul_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, w) in signed_quadruple_gen::<T>().get(gm, config).take(limit) {
        println!(
            "({}).overflowing_mul_add_mul({}, {}, {}) = {:?}",
            x,
            y,
            z,
            w,
            x.overflowing_mul_add_mul(y, z, w)
        );
    }
}

fn demo_overflowing_mul_add_mul_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, z, w) in unsigned_quadruple_gen::<T>().get(gm, config).take(limit) {
        let old_x = x;
        let overflow = x.overflowing_mul_add_mul_assign(y, z, w);
        println!(
            "x := {old_x}; x.overflowing_mul_add_mul_assign({y}, {z}, {w}) = {overflow}; x = {x}"
        );
    }
}

fn demo_overflowing_mul_add_mul_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, z, w) in signed_quadruple_gen::<T>().get(gm, config).take(limit) {
        let old_x = x;
        let overflow = x.overflowing_mul_add_mul_assign(y, z, w);
        println!(
            "x := {old_x}; x.overflowing_mul_add_mul_assign({y}, {z}, {w}) = {overflow}; x = {x}"
        );
    }
}

fn benchmark_overflowing_mul_add_mul_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.overflowing_mul_add_mul({}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_quadruple_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_max_bit_bucketer("x", "y", "z", "w"),
        &mut [("Malachite", &mut |(x, y, z, w)| {
            no_out!(x.overflowing_mul_add_mul(y, z, w));
        })],
    );
}

fn benchmark_overflowing_mul_add_mul_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.overflowing_mul_add_mul({}, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        signed_quadruple_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_max_bit_bucketer("x", "y", "z", "w"),
        &mut [("Malachite", &mut |(x, y, z, w)| {
            no_out!(x.overflowing_mul_add_mul(y, z, w));
        })],
    );
}
