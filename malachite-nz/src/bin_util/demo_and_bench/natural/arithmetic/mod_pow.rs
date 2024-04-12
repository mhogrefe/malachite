// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPow, ModPowAssign};
use malachite_base::test_util::bench::bucketers::quadruple_4_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mod_pow::{
    limbs_mod_pow, limbs_mod_pow_odd, limbs_mod_pow_odd_scratch_len,
};
use malachite_nz::test_util::bench::bucketers::{
    triple_1_3_prod_natural_bits_bucketer, triple_3_triple_1_3_prod_natural_bits_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_triple_gen_var_5, natural_triple_gen_var_5_nrm, unsigned_vec_quadruple_gen_var_6,
    unsigned_vec_quadruple_gen_var_7,
};
use malachite_nz::test_util::natural::arithmetic::mod_pow::simple_binary_mod_pow;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_mod_pow_odd);
    register_demo!(runner, demo_limbs_mod_pow);
    register_demo!(runner, demo_natural_mod_pow_assign);
    register_demo!(runner, demo_natural_mod_pow_assign_val_ref);
    register_demo!(runner, demo_natural_mod_pow_assign_ref_val);
    register_demo!(runner, demo_natural_mod_pow_assign_ref_ref);
    register_demo!(runner, demo_natural_mod_pow);
    register_demo!(runner, demo_natural_mod_pow_val_val_ref);
    register_demo!(runner, demo_natural_mod_pow_val_ref_val);
    register_demo!(runner, demo_natural_mod_pow_val_ref_ref);
    register_demo!(runner, demo_natural_mod_pow_ref_val_val);
    register_demo!(runner, demo_natural_mod_pow_ref_val_ref);
    register_demo!(runner, demo_natural_mod_pow_ref_ref_val);
    register_demo!(runner, demo_natural_mod_pow_ref_ref_ref);

    register_bench!(runner, benchmark_limbs_mod_pow_odd);
    register_bench!(runner, benchmark_limbs_mod_pow);
    register_bench!(runner, benchmark_natural_mod_pow_assign_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_pow_algorithms);
    register_bench!(runner, benchmark_natural_mod_pow_library_comparison);
    register_bench!(runner, benchmark_natural_mod_pow_evaluation_strategy);
}

fn demo_limbs_mod_pow_odd(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, es, ms) in unsigned_vec_quadruple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let out_old = out.clone();
        let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(ms.len())];
        limbs_mod_pow_odd(&mut out, &xs, &es, &ms, &mut scratch);
        println!(
            "out := {out_old:?}; \
            limbs_mod_pow_odd(&mut out, {xs:?}, {es:?}, {ms:?}, &mut scratch); out = {out:?}",
        );
    }
}

fn demo_limbs_mod_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut out, xs, es, ms) in unsigned_vec_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let out_old = out.clone();
        limbs_mod_pow(&mut out, &xs, &es, &ms);
        println!(
            "out := {out_old:?}; limbs_mod_pow(&mut out, {xs:?}, {es:?}, {ms:?}); out = {out:?}",
        );
    }
}

fn demo_natural_mod_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let exp_old = exp.clone();
        let m_old = m.clone();
        x.mod_pow_assign(exp, m);
        println!("x := {x_old}; x.mod_pow_assign({exp_old}, {m_old}); x = {x}");
    }
}

fn demo_natural_mod_pow_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        let m_old = m.clone();
        let exp_old = exp.clone();
        x.mod_pow_assign(exp, &m);
        println!("x := {x}; x.mod_pow_assign({exp_old}, &{m_old}); x = {x}");
    }
}

fn demo_natural_mod_pow_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        x.mod_pow_assign(&exp, m);
        println!("x := {x_old}; x.mod_pow_assign(&{exp}, {m_old}); x = {x}");
    }
}

fn demo_natural_mod_pow_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.mod_pow_assign(&exp, &m);
        println!("x := {x_old}; x.mod_pow_assign(&{exp}, &{m}); x = {x}");
    }
}

fn demo_natural_mod_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let exp_old = exp.clone();
        let m_old = m.clone();
        println!(
            "{}.pow({}) ≡ {} mod {}",
            x_old,
            exp_old,
            x.mod_pow(exp, m),
            m_old
        );
    }
}

fn demo_natural_mod_pow_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let exp_old = exp.clone();
        println!(
            "{}.pow({}) ≡ {} mod {}",
            x_old,
            exp_old,
            x.mod_pow(exp, &m),
            m
        );
    }
}

fn demo_natural_mod_pow_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        println!(
            "{}.pow({}) ≡ {} mod {}",
            x_old,
            exp,
            x.mod_pow(&exp, m),
            m_old
        );
    }
}

fn demo_natural_mod_pow_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.pow({}) ≡ {} mod {}", x_old, exp, x.mod_pow(&exp, &m), m);
    }
}

fn demo_natural_mod_pow_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        let exp_old = exp.clone();
        let m_old = m.clone();
        println!(
            "{}.pow({}) ≡ {} mod {}",
            x,
            exp_old,
            (&x).mod_pow(exp, m),
            m_old
        );
    }
}

fn demo_natural_mod_pow_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        let exp_old = exp.clone();
        println!(
            "{}.pow({}) ≡ {} mod {}",
            x,
            exp_old,
            (&x).mod_pow(exp, &m),
            m
        );
    }
}

fn demo_natural_mod_pow_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        let m_old = m.clone();
        println!(
            "{}.pow({}) ≡ {} mod {}",
            x,
            exp,
            (&x).mod_pow(&exp, m),
            m_old
        );
    }
}

fn demo_natural_mod_pow_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, exp, m) in natural_triple_gen_var_5().get(gm, config).take(limit) {
        println!("{}.pow({}) ≡ {} mod {}", x, exp, (&x).mod_pow(&exp, &m), m);
    }
}

fn benchmark_limbs_mod_pow_odd(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mod_pow_odd(&mut [Limb], &[Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_quadruple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_4_vec_len_bucketer("ms"),
        &mut [("Malachite", &mut |(mut out, xs, es, ms)| {
            let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(ms.len())];
            limbs_mod_pow_odd(&mut out, &xs, &es, &ms, &mut scratch);
        })],
    );
}

fn benchmark_limbs_mod_pow(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mod_pow(&mut [Limb], &[Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_quadruple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_4_vec_len_bucketer("ms"),
        &mut [("Malachite", &mut |(mut out, xs, es, ms)| {
            limbs_mod_pow(&mut out, &xs, &es, &ms);
        })],
    );
}

fn benchmark_natural_mod_pow_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_pow_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_3_prod_natural_bits_bucketer("exp", "m"),
        &mut [
            (
                "Natural.mod_pow_assign(Natural, Natural)",
                &mut |(mut x, exp, m)| no_out!(x.mod_pow_assign(exp, m)),
            ),
            (
                "Natural.mod_pow_assign(Natural, &Natural)",
                &mut |(mut x, exp, m)| no_out!(x.mod_pow_assign(exp, &m)),
            ),
            (
                "Natural.mod_pow_assign(&Natural, Natural)",
                &mut |(mut x, exp, m)| no_out!(x.mod_pow_assign(&exp, m)),
            ),
            (
                "Natural.mod_pow_assign(&Natural, &Natural)",
                &mut |(mut x, exp, m)| no_out!(x.mod_pow_assign(&exp, &m)),
            ),
        ],
    );
}

fn benchmark_natural_mod_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_pow(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_3_prod_natural_bits_bucketer("exp", "m"),
        &mut [
            ("default", &mut |(x, exp, m)| no_out!(x.mod_pow(exp, m))),
            ("simple binary", &mut |(x, exp, m)| {
                no_out!(simple_binary_mod_pow(&x, &exp, &m))
            }),
        ],
    );
}

fn benchmark_natural_mod_pow_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_pow(Natural, Natural)",
        BenchmarkType::LibraryComparison,
        natural_triple_gen_var_5_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_triple_1_3_prod_natural_bits_bucketer("exp", "m"),
        &mut [
            ("Malachite", &mut |(_, _, (x, exp, m))| {
                no_out!(x.mod_pow(exp, m))
            }),
            (
                "num",
                &mut |((x, exp, m), _, _)| no_out!(x.modpow(&exp, &m)),
            ),
            ("rug", &mut |(_, (x, exp, m), _)| {
                no_out!(x.pow_mod(&exp, &m).unwrap())
            }),
        ],
    );
}

fn benchmark_natural_mod_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_pow(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_3_prod_natural_bits_bucketer("exp", "m"),
        &mut [
            ("Natural.mod_pow(Natural, Natural)", &mut |(x, exp, m)| {
                no_out!(x.mod_pow(exp, m))
            }),
            ("Natural.mod_pow(Natural, &Natural)", &mut |(x, exp, m)| {
                no_out!(x.mod_pow(exp, &m))
            }),
            ("Natural.mod_pow(&Natural, Natural)", &mut |(x, exp, m)| {
                no_out!(x.mod_pow(&exp, m))
            }),
            ("Natural.mod_pow(&Natural, &Natural)", &mut |(x, exp, m)| {
                no_out!(x.mod_pow(&exp, &m))
            }),
            ("(&Natural).mod_pow(Natural, Natural)", &mut |(
                x,
                exp,
                m,
            )| {
                no_out!((&x).mod_pow(exp, m))
            }),
            (
                "(&Natural).mod_pow(Natural, &Natural)",
                &mut |(x, exp, m)| no_out!((&x).mod_pow(exp, &m)),
            ),
            (
                "(&Natural).mod_pow(&Natural, Natural)",
                &mut |(x, exp, m)| no_out!((&x).mod_pow(&exp, m)),
            ),
            (
                "(&Natural).mod_pow(&Natural, &Natural)",
                &mut |(x, exp, m)| no_out!((&x).mod_pow(&exp, &m)),
            ),
        ],
    );
}
