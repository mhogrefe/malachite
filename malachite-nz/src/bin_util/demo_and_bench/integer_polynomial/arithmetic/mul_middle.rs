// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::quadruple_1_2_vec_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::classical::*;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::mul_middle_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::tiny::{
    mul_middle_to_out_tiny_1, mul_middle_to_out_tiny_2,
};
use malachite_nz::test_util::generators::{
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1,
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_2,
    integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_3,
    integer_vec_unsigned_unsigned_triple_gen_var_1, integer_vec_unsigned_unsigned_triple_gen_var_2,
    integer_vec_unsigned_unsigned_triple_gen_var_3,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_mul_middle_to_out_classical);
    register_demo!(runner, demo_mul_middle_to_out_tiny_1);
    register_demo!(runner, demo_mul_middle_to_out_tiny_2);
    register_demo!(runner, demo_mul_middle_to_out);
    register_demo!(runner, demo_mul_middle_to_out_classical_square);
    register_demo!(runner, demo_mul_middle_to_out_tiny_1_square);
    register_demo!(runner, demo_mul_middle_to_out_tiny_2_square);

    register_bench!(runner, benchmark_mul_middle_to_out_algorithms);
    register_bench!(runner, benchmark_mul_middle_to_out_tiny_1_algorithms);
    register_bench!(runner, benchmark_mul_middle_to_out_tiny_2_algorithms);
}

fn demo_mul_middle_to_out_classical(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, nlo, nhi) in integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_classical(
            &mut out,
            &xs,
            &ys,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_classical(_, {xs:?}, {ys:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_tiny_1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, nlo, nhi) in integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_tiny_1(
            &mut out,
            &xs,
            &ys,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_tiny_1(_, {xs:?}, {ys:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_tiny_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, nlo, nhi) in integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_tiny_2(
            &mut out,
            &xs,
            &ys,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_tiny_2(_, {xs:?}, {ys:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, nlo, nhi) in integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out(
            &mut out,
            &xs,
            &ys,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out(_, {xs:?}, {ys:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_classical_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, nlo, nhi) in integer_vec_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_classical(
            &mut out,
            &xs,
            &xs,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_classical(_, {xs:?}, {xs:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_tiny_1_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, nlo, nhi) in integer_vec_unsigned_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_tiny_1(
            &mut out,
            &xs,
            &xs,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_tiny_1(_, {xs:?}, {xs:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn demo_mul_middle_to_out_tiny_2_square(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, nlo, nhi) in integer_vec_unsigned_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
        mul_middle_to_out_tiny_2(
            &mut out,
            &xs,
            &xs,
            usize::exact_from(nlo),
            usize::exact_from(nhi),
        );
        println!("mul_middle_to_out_tiny_2(_, {xs:?}, {xs:?}, {nlo}, {nhi}) = {out:?}");
    }
}

fn benchmark_mul_middle_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_middle_to_out(&mut [Integer], &[Integer], &[Integer], usize, usize)",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("classical", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_classical(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("default", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("naive", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                out.clone_from_slice(
                    &integers_mul_naive(&xs, &ys)[usize::exact_from(nlo)..usize::exact_from(nhi)],
                );
            }),
        ],
    );
}

fn benchmark_mul_middle_to_out_tiny_1_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_middle_to_out_tiny_1(&mut [Integer], &[Integer], &[Integer], usize, usize)",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("tiny 1", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_tiny_1(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("tiny 2", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_tiny_2(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("classical", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_classical(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("naive", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                out.clone_from_slice(
                    &integers_mul_naive(&xs, &ys)[usize::exact_from(nlo)..usize::exact_from(nhi)],
                );
            }),
        ],
    );
}

fn benchmark_mul_middle_to_out_tiny_2_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_middle_to_out_tiny_2(&mut [Integer], &[Integer], &[Integer], usize, usize)",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_unsigned_unsigned_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("tiny 2", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_tiny_2(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("classical", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                mul_middle_to_out_classical(
                    &mut out,
                    &xs,
                    &ys,
                    usize::exact_from(nlo),
                    usize::exact_from(nhi),
                );
            }),
            ("naive", &mut |(xs, ys, nlo, nhi)| {
                let mut out = vec![Integer::ZERO; usize::exact_from(nhi - nlo)];
                out.clone_from_slice(
                    &integers_mul_naive(&xs, &ys)[usize::exact_from(nlo)..usize::exact_from(nhi)],
                );
            }),
        ],
    );
}
