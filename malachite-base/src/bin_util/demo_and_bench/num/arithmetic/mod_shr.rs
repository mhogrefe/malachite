// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModShr, ModShrAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::bench::bucketers::triple_2_3_product_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_signed_unsigned_triple_gen_var_2;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_signed_match_demos!(runner, demo_mod_shr);
    register_unsigned_unsigned_signed_match_demos!(runner, demo_mod_shr_assign);

    register_unsigned_unsigned_signed_match_benches!(runner, benchmark_mod_shr);
    register_unsigned_unsigned_signed_match_benches!(runner, benchmark_mod_shr_assign);
}

fn demo_mod_shr<
    T: ModShr<S, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, u, m) in unsigned_signed_unsigned_triple_gen_var_2::<T, U, S>()
        .get(gm, config)
        .take(limit)
    {
        println!("{} >> {} ≡ {} mod {}", x, u, x.mod_shr(u, m), m);
    }
}

fn demo_mod_shr_assign<
    T: ModShrAssign<S> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, u, m) in unsigned_signed_unsigned_triple_gen_var_2::<T, U, S>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.mod_shr_assign(u, m);
        println!("x := {old_x}; x.mod_shr_assign({u}, {m}); x = {x}");
    }
}

fn benchmark_mod_shr<
    T: ModShr<S, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_shr({}, {})", T::NAME, S::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_signed_unsigned_triple_gen_var_2::<T, U, S>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_product_bit_bucketer("u", "m"),
        &mut [("Malachite", &mut |(x, u, m)| no_out!(x.mod_shr(u, m)))],
    );
}

fn benchmark_mod_shr_assign<
    T: ModShrAssign<S> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_shr_assign({}, u64)", T::NAME, S::NAME),
        BenchmarkType::Single,
        unsigned_signed_unsigned_triple_gen_var_2::<T, U, S>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_product_bit_bucketer("u", "m"),
        &mut [("Malachite", &mut |(mut x, u, m)| x.mod_shr_assign(u, m))],
    );
}
