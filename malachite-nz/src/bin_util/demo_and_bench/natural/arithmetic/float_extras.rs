// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::triple_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::float_extras::limbs_float_exp;
use malachite_nz::test_util::generators::unsigned_vec_unsigned_unsigned_triple_gen_var_17;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_float_exp);
    register_bench!(runner, benchmark_limbs_float_exp);
}

fn demo_limbs_float_exp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, b, e) in unsigned_vec_unsigned_unsigned_triple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let n = out.len();
        let (exp, err) = limbs_float_exp(&mut out, b, i64::exact_from(e));
        println!(
            "limbs_float_exp(&mut out[len = {n}], {b}, {e}) = (exp = {exp}, err = {err}); \
            out = {out:?}"
        );
    }
}

fn benchmark_limbs_float_exp(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_float_exp(&mut [Limb], u64, i64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_17().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("out"),
        &mut [("Malachite", &mut |(mut out, b, e)| {
            no_out!(limbs_float_exp(&mut out, b, i64::exact_from(e)));
        })],
    );
}
