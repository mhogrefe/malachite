// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::modular::traits::{ModPowerOf2Evaluate, ModPowerOf2EvaluateMany};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_polynomial_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_evaluate_many::*;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::{
    PowerOf2Modulus, evaluate_many_fast,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(
        runner,
        demo_unsigned_polynomial_mod_power_of_2_evaluate_many
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_power_of_2_evaluate_many_algorithms
    );
}

fn demo_unsigned_polynomial_mod_power_of_2_evaluate_many<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, xs, pow) in unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_evaluate_many(&{xs:?}, {pow}) = {:?}",
            (&p).mod_power_of_2_evaluate_many(&xs, pow)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_power_of_2_evaluate_many_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "(&UnsignedPolynomial<{}>).mod_power_of_2_evaluate_many(&[{}], u64)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_vec_unsigned_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_polynomial_len_bucketer("p"),
        &mut [
            ("default", &mut |(p, xs, pow)| {
                no_out!((&p).mod_power_of_2_evaluate_many(&xs, pow));
            }),
            ("one at a time", &mut |(p, xs, pow)| {
                no_out!(
                    xs.iter()
                        .map(|&x| (&p).mod_power_of_2_evaluate(x, pow))
                        .collect::<Vec<_>>()
                );
            }),
            ("subproduct tree", &mut |(p, xs, pow)| {
                if pow != 0 {
                    no_out!(evaluate_many_fast(
                        p.coefficients_asc(),
                        &xs,
                        &PowerOf2Modulus(pow)
                    ));
                }
            }),
            ("naive", &mut |(p, xs, pow)| {
                no_out!(mod_power_of_2_evaluate_many_naive(&p, &xs, pow));
            }),
        ],
    );
}
