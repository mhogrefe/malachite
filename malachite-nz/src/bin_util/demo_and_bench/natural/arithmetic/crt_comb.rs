// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::crt_comb::CrtComb;
use malachite_nz::test_util::bench::bucketers::pair_2_natural_bit_bucketer;
use malachite_nz::test_util::generators::unsigned_vec_natural_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_crt_comb_reduce);
    register_demo!(runner, demo_natural_crt_comb_combine);
    register_demo!(runner, demo_natural_crt_comb_combine_balanced);

    register_bench!(runner, benchmark_natural_crt_comb_reduce_algorithms);
    register_bench!(runner, benchmark_natural_crt_comb_combine_algorithms);
}

fn demo_natural_crt_comb_reduce(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ms, x) in unsigned_vec_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let comb = CrtComb::new(&ms).unwrap();
        println!("comb_reduce({ms:?}, {x}) = {:?}", comb.reduce(&x));
    }
}

fn demo_natural_crt_comb_combine(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ms, x) in unsigned_vec_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let comb = CrtComb::new(&ms).unwrap();
        let rs = comb.reduce(&x);
        println!("comb_combine({ms:?}, {rs:?}) = {}", comb.combine(&rs));
    }
}

fn demo_natural_crt_comb_combine_balanced(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ms, x) in unsigned_vec_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let comb = CrtComb::new(&ms).unwrap();
        let rs = comb.reduce(&x);
        println!(
            "comb_combine_balanced({ms:?}, {rs:?}) = {}",
            comb.combine_balanced(&rs)
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_natural_crt_comb_reduce_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "CrtComb.reduce(&Natural)",
        BenchmarkType::Algorithms,
        unsigned_vec_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("x"),
        &mut [
            ("comb", &mut |(ms, x)| {
                let comb = CrtComb::new(&ms).unwrap();
                no_out!(comb.reduce(&x));
            }),
            ("one modulus at a time", &mut |(ms, x)| {
                for m in ms {
                    no_out!(&x % Natural::from(m));
                }
            }),
        ],
    );
}

fn benchmark_natural_crt_comb_combine_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "CrtComb.combine(&[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("x"),
        &mut [
            ("comb", &mut |(ms, x)| {
                let comb = CrtComb::new(&ms).unwrap();
                let rs = comb.reduce(&x);
                no_out!(comb.combine(&rs));
            }),
            ("multi_crt", &mut |(ms, x)| {
                let moduli = ms.iter().map(|&m| Natural::from(m)).collect::<Vec<_>>();
                let values = ms
                    .iter()
                    .map(|&m| &x % Natural::from(m))
                    .collect::<Vec<_>>();
                no_out!(Natural::multi_crt(&moduli, &values));
            }),
        ],
    );
}
