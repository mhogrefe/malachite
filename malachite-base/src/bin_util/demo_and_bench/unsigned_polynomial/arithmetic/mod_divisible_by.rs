// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::ModDivisibleBy;
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_divisible_by);
    register_unsigned_demos!(runner, demo_unsigned_polynomial_mod_divisible_by_ref_ref);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_polynomial_mod_divisible_by_evaluation_strategy
    );
}

fn demo_unsigned_polynomial_mod_divisible_by<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        let q_old = q.clone();
        println!(
            "({p_old}).mod_divisible_by({q_old}, {m}) = {}",
            p.mod_divisible_by(q, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_divisible_by_ref_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, q, m) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_divisible_by(&({q}), {m}) = {}",
            (&p).mod_divisible_by(&q, m)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_divisible_by_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedPolynomial<{}>.mod_divisible_by(UnsignedPolynomial<{}>, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [
            ("p.mod_divisible_by(q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_divisible_by(q, m));
            }),
            ("p.mod_divisible_by(&q, m)", &mut |(p, q, m)| {
                no_out!(p.mod_divisible_by(&q, m));
            }),
            ("(&p).mod_divisible_by(q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_divisible_by(q, m));
            }),
            ("(&p).mod_divisible_by(&q, m)", &mut |(p, q, m)| {
                no_out!((&p).mod_divisible_by(&q, m));
            }),
        ],
    );
}
