// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::bench::bucketers::{Bucketer, triple_1_vec_len_bucketer};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::float_extras::{limbs_float_exp, limbs_get_str_aux};
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    large_type_gen_var_28, unsigned_vec_unsigned_unsigned_triple_gen_var_17,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_float_exp);
    register_demo!(runner, demo_limbs_get_str_aux);
    register_bench!(runner, benchmark_limbs_float_exp);
    register_bench!(runner, benchmark_limbs_get_str_aux);
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

fn demo_limbs_get_str_aux(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut r, f, e, b0, m, rnd) in large_type_gen_var_28().get(gm, config).take(limit) {
        let r_old = r.clone();
        let mut out = vec![0; m];
        let (dir, exp) = limbs_get_str_aux(&mut out, &mut r, f, e, b0, m, rnd);
        println!(
            "limbs_get_str_aux(out, {r_old:?}, {f}, {e}, {b0}, {m}, {rnd}) = (dir = {dir}, \
             exp = {exp}); str = {:?}",
            String::from_utf8_lossy(&out)
        );
    }
}

fn benchmark_limbs_get_str_aux(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_get_str_aux(&mut [u8], &mut [Limb], i64, i64, i64, usize, RoundingMode)",
        BenchmarkType::Single,
        large_type_gen_var_28().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &Bucketer {
            bucketing_function: &|(r, _, _, _, _, _): &(
                Vec<Limb>,
                i64,
                i64,
                i64,
                usize,
                RoundingMode,
            )| r.len(),
            bucketing_label: "r.len()".to_string(),
        },
        &mut [("Malachite", &mut |(mut r, f, e, b0, m, rnd)| {
            no_out!(limbs_get_str_aux(&mut vec![0; m], &mut r, f, e, b0, m, rnd));
        })],
    );
}
