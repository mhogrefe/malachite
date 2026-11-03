// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::{ModEvaluate, ModEvaluateMany};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_many::*;
use malachite_base::unsigned_polynomial::modular::mod_evaluate_many::mod_evaluate_many_in_place;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::{Modulus, evaluate_many_fast};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_evaluate_many);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_evaluate_many_algorithms
    );
}

fn demo_unsigned_polynomial_mod_evaluate_many(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, xs, m) in unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_evaluate_many(&{xs:?}, {m}) = {:?}",
            (&p).mod_evaluate_many(&xs, m)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_evaluate_many_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedPolynomial<u64>).mod_evaluate_many(&[u64], u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, xs, m)| {
                no_out!((&p).mod_evaluate_many(&xs, m));
            }),
            ("one at a time", &mut |(p, xs, m)| {
                no_out!(
                    xs.iter()
                        .map(|&x| (&p).mod_evaluate(x, m))
                        .collect::<Vec<_>>()
                );
            }),
            ("Horner's rule in blocks", &mut |(p, xs, m)| {
                let mut values = xs;
                mod_evaluate_many_in_place(p.coefficients_asc(), &mut values, m);
                no_out!(values);
            }),
            ("subproduct tree", &mut |(p, xs, m)| {
                if m != 1 {
                    no_out!(evaluate_many_fast(p.coefficients_asc(), &xs, &Modulus(m)));
                }
            }),
            ("naive", &mut |(p, xs, m)| {
                no_out!(mod_evaluate_many_naive(&p, &xs, m));
            }),
        ],
    );
}
