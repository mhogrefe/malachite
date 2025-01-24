// Copyright © 2025 Mikhail Hogrefe
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
    triple_max_bit_bucketer, triple_max_primitive_float_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_triple_gen, signed_triple_gen_var_1, unsigned_triple_gen_var_1,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_add_mul_unsigned);
    register_signed_demos!(runner, demo_add_mul_signed);
    register_primitive_float_demos!(runner, demo_add_mul_primitive_float);
    register_unsigned_demos!(runner, demo_add_mul_assign_unsigned);
    register_signed_demos!(runner, demo_add_mul_assign_signed);
    register_primitive_float_demos!(runner, demo_add_mul_assign_primitive_float);

    register_unsigned_benches!(runner, benchmark_add_mul_unsigned);
    register_signed_benches!(runner, benchmark_add_mul_signed);
    register_primitive_float_benches!(runner, benchmark_add_mul_primitive_float);
    register_unsigned_benches!(runner, benchmark_add_mul_assign_unsigned);
    register_signed_benches!(runner, benchmark_add_mul_assign_signed);
    register_primitive_float_benches!(runner, benchmark_add_mul_assign_primitive_float);
}

fn demo_add_mul_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in unsigned_triple_gen_var_1::<T>().get(gm, config).take(limit) {
        println!("{}.add_mul({}, {}) = {}", x, y, z, x.add_mul(y, z));
    }
}

fn demo_add_mul_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in signed_triple_gen_var_1::<T>().get(gm, config).take(limit) {
        println!("({}).add_mul({}, {}) = {}", x, y, z, x.add_mul(y, z));
    }
}

fn demo_add_mul_primitive_float<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in primitive_float_triple_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).add_mul({}, {}) = {}",
            NiceFloat(x),
            NiceFloat(y),
            NiceFloat(z),
            NiceFloat(x.add_mul(y, z))
        );
    }
}

fn demo_add_mul_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, z) in unsigned_triple_gen_var_1::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.add_mul_assign(y, z);
        println!("x := {old_x}; x.add_mul_assign({y}, {z}); x = {x}");
    }
}

fn demo_add_mul_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in signed_triple_gen_var_1::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.add_mul_assign(y, z);
        println!("x := {old_x}; x.add_mul_assign({y}, {z}); x = {x}");
    }
}

fn demo_add_mul_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, z) in primitive_float_triple_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.add_mul_assign(y, z);
        println!(
            "x := {}; x.add_mul_assign({}, {}); x = {}",
            NiceFloat(old_x),
            NiceFloat(y),
            NiceFloat(z),
            NiceFloat(x)
        );
    }
}

fn benchmark_add_mul_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.add_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(x.add_mul(y, z)))],
    );
}

fn benchmark_add_mul_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.add_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(x.add_mul(y, z)))],
    );
}

fn benchmark_add_mul_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.add_mul({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        primitive_float_triple_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_primitive_float_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(x.add_mul(y, z)))],
    );
}

fn benchmark_add_mul_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.add_mul_assign({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(mut x, y, z)| x.add_mul_assign(y, z))],
    );
}

fn benchmark_add_mul_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.add_mul_assign({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(mut x, y, z)| x.add_mul_assign(y, z))],
    );
}

fn benchmark_add_mul_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.add_mul_assign({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        primitive_float_triple_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_primitive_float_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(mut x, y, z)| x.add_mul_assign(y, z))],
    );
}
