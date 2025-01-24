// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, UnsignedAbs};
use malachite_base::test_util::bench::bucketers::{
    triple_1_2_vec_max_len_bucketer, triple_1_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_vec_triple_gen_var_36, unsigned_vec_unsigned_unsigned_triple_gen_var_5,
    unsigned_vec_unsigned_unsigned_triple_gen_var_7,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::arithmetic::eq_mod::{
    limbs_eq_neg_limb_mod_limb, limbs_pos_eq_neg_limb_mod, limbs_pos_eq_neg_limb_mod_ref,
    limbs_pos_eq_neg_mod, limbs_pos_eq_neg_mod_limb, limbs_pos_eq_neg_mod_ref,
    limbs_pos_limb_eq_neg_limb_mod,
};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_triple_1_2_integer_max_bit_bucketer, triple_1_2_integer_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_integer_natural_triple_gen, integer_integer_natural_triple_gen_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_eq_neg_limb_mod_limb);
    register_demo!(runner, demo_limbs_pos_limb_eq_neg_limb_mod);
    register_demo!(runner, demo_limbs_pos_eq_neg_limb_mod);
    register_demo!(runner, demo_limbs_pos_eq_neg_limb_mod_ref);
    register_demo!(runner, demo_limbs_pos_eq_neg_mod_limb);
    register_demo!(runner, demo_limbs_pos_eq_neg_mod);
    register_demo!(runner, demo_limbs_pos_eq_neg_mod_ref);
    register_demo!(runner, demo_integer_eq_mod);
    register_demo!(runner, demo_integer_eq_mod_val_val_ref);
    register_demo!(runner, demo_integer_eq_mod_val_ref_val);
    register_demo!(runner, demo_integer_eq_mod_val_ref_ref);
    register_demo!(runner, demo_integer_eq_mod_ref_val_val);
    register_demo!(runner, demo_integer_eq_mod_ref_val_ref);
    register_demo!(runner, demo_integer_eq_mod_ref_ref_val);
    register_demo!(runner, demo_integer_eq_mod_ref_ref_ref);

    register_bench!(runner, benchmark_limbs_eq_neg_limb_mod_limb);
    register_bench!(runner, benchmark_limbs_pos_limb_eq_neg_limb_mod);
    register_bench!(
        runner,
        benchmark_limbs_pos_eq_neg_limb_mod_evaluation_strategy
    );
    register_bench!(runner, benchmark_limbs_pos_eq_neg_mod_limb);
    register_bench!(runner, benchmark_limbs_pos_eq_neg_mod_evaluation_strategy);
    register_bench!(runner, benchmark_integer_eq_mod_evaluation_strategy);
    register_bench!(runner, benchmark_integer_eq_mod_library_comparison);
    register_bench!(runner, benchmark_integer_eq_mod_algorithms);
}

fn demo_limbs_eq_neg_limb_mod_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y, m) in unsigned_vec_unsigned_unsigned_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_eq_neg_limb_mod_limb({:?}, {}, {}) = {}",
            xs,
            y,
            m,
            limbs_eq_neg_limb_mod_limb(&xs, y, m)
        );
    }
}

fn demo_limbs_pos_limb_eq_neg_limb_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (m, x, y) in unsigned_vec_unsigned_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_pos_limb_eq_neg_limb_mod({}, {}, {:?}) = {}",
            x,
            y,
            m,
            limbs_pos_limb_eq_neg_limb_mod(x, y, &m)
        );
    }
}

fn demo_limbs_pos_eq_neg_limb_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, mut m, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let old_m = m.clone();
        println!(
            "limbs_pos_eq_neg_limb_mod({:?}, {}, {:?}) = {}",
            xs,
            y,
            old_m,
            limbs_pos_eq_neg_limb_mod(&xs, y, &mut m)
        );
    }
}

fn demo_limbs_pos_eq_neg_limb_mod_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, m, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_pos_eq_neg_limb_mod_ref({:?}, {}, {:?}) = {}",
            xs,
            y,
            m,
            limbs_pos_eq_neg_limb_mod_ref(&xs, y, &m)
        );
    }
}

fn demo_limbs_pos_eq_neg_mod_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, m) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_pos_eq_neg_mod_limb({:?}, {:?}, {}) = {}",
            xs,
            ys,
            m,
            limbs_pos_eq_neg_mod_limb(&xs, &ys, m)
        );
    }
}

fn demo_limbs_pos_eq_neg_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, mut m) in unsigned_vec_triple_gen_var_36().get(gm, config).take(limit) {
        let old_m = m.clone();
        println!(
            "limbs_pos_eq_neg_mod({:?}, {:?}, {:?}) = {}",
            xs,
            ys,
            old_m,
            limbs_pos_eq_neg_mod(&xs, &ys, &mut m)
        );
    }
}

fn demo_limbs_pos_eq_neg_mod_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, m) in unsigned_vec_triple_gen_var_36().get(gm, config).take(limit) {
        println!(
            "limbs_pos_eq_neg_mod_ref({:?}, {:?}, {:?}) = {}",
            xs,
            ys,
            m,
            limbs_pos_eq_neg_mod_ref(&xs, &ys, &m)
        );
    }
}

fn demo_integer_eq_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in integer_integer_natural_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        if x.eq_mod(y, m) {
            println!("{x_old} is equal to {y_old} mod {m_old}");
        } else {
            println!("{x_old} is not equal to {y_old} mod {m_old}");
        }
    }
}

fn demo_integer_eq_mod_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in integer_integer_natural_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        if x.eq_mod(y, &m) {
            println!("{x_old} is equal to {y_old} mod &{m}");
        } else {
            println!("{x_old} is not equal to {y_old} mod &{m}");
        }
    }
}

fn demo_integer_eq_mod_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in integer_integer_natural_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let m_old = m.clone();
        if x.eq_mod(&y, m) {
            println!("{x_old} is equal to &{y} mod {m_old}");
        } else {
            println!("{x_old} is not equal to &{y} mod {m_old}");
        }
    }
}

fn demo_integer_eq_mod_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in integer_integer_natural_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        if x.eq_mod(&y, &m) {
            println!("{x_old} is equal to &{y} mod &{m}");
        } else {
            println!("{x_old} is not equal to &{y} mod &{m}");
        }
    }
}

fn demo_integer_eq_mod_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in integer_integer_natural_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let m_old = m.clone();
        if (&x).eq_mod(y, m) {
            println!("&{x} is equal to {y_old} mod {m_old}");
        } else {
            println!("&{x} is not equal to {y_old} mod {m_old}");
        }
    }
}

fn demo_integer_eq_mod_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in integer_integer_natural_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        if (&x).eq_mod(y, &m) {
            println!("&{x} is equal to {y_old} mod &{m}");
        } else {
            println!("&{x} is not equal to {y_old} mod &{m}");
        }
    }
}

fn demo_integer_eq_mod_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in integer_integer_natural_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let m_old = m.clone();
        if (&x).eq_mod(&y, m) {
            println!("&{x} is equal to &{y} mod {m_old}");
        } else {
            println!("&{x} is not equal to &{y} mod {m_old}");
        }
    }
}

fn demo_integer_eq_mod_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in integer_integer_natural_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        if (&x).eq_mod(&y, &m) {
            println!("&{x} is equal to &{y} mod &{m}");
        } else {
            println!("&{x} is not equal to &{y} mod &{m}");
        }
    }
}

fn benchmark_limbs_eq_neg_limb_mod_limb(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_neg_limb_mod_limb(&mut [Limb], Limb, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("limbs_eq_neg_limb_mod_limb", &mut |(xs, y, m)| {
            no_out!(limbs_eq_neg_limb_mod_limb(&xs, y, m))
        })],
    );
}

fn benchmark_limbs_pos_limb_eq_neg_limb_mod(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_pos_limb_eq_neg_limb_mod(Limb, Limb, &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("m"),
        &mut [("limbs_pos_limb_eq_neg_limb_mod", &mut |(m, x, y)| {
            no_out!(limbs_pos_limb_eq_neg_limb_mod(x, y, &m))
        })],
    );
}

fn benchmark_limbs_pos_eq_neg_limb_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_mod_limb(&[Limb], Limb, &[Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [
            ("limbs_pos_eq_neg_limb_mod", &mut |(xs, mut m, y)| {
                no_out!(limbs_pos_eq_neg_limb_mod(&xs, y, &mut m))
            }),
            ("limbs_pos_eq_neg_limb_mod_ref", &mut |(xs, m, y)| {
                no_out!(limbs_pos_eq_neg_limb_mod_ref(&xs, y, &m))
            }),
        ],
    );
}

fn benchmark_limbs_pos_eq_neg_mod_limb(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_pos_eq_neg_mod_limb(&[Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [("limbs_pos_eq_neg_mod_limb", &mut |(xs, ys, m)| {
            no_out!(limbs_pos_eq_neg_mod_limb(&xs, &ys, m))
        })],
    );
}

fn benchmark_limbs_pos_eq_neg_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_mod_limb(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("limbs_pos_eq_neg_mod", &mut |(ref xs, ref y, ref mut m)| {
                no_out!(limbs_pos_eq_neg_mod(xs, y, m))
            }),
            ("limbs_pos_eq_neg_mod_ref", &mut |(ref xs, ref y, ref m)| {
                no_out!(limbs_pos_eq_neg_mod_ref(xs, y, m))
            }),
        ],
    );
}

fn benchmark_integer_eq_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.eq_mod(Integer, Natural)",
        BenchmarkType::EvaluationStrategy,
        integer_integer_natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Integer.eq_mod(Integer, Natural)", &mut |(x, y, m)| {
                no_out!(x.eq_mod(y, m))
            }),
            ("Integer.eq_mod(Integer, &Integer)", &mut |(x, y, m)| {
                no_out!(x.eq_mod(y, &m))
            }),
            ("Integer.eq_mod(&Integer, Integer)", &mut |(x, y, m)| {
                no_out!(x.eq_mod(&y, m))
            }),
            ("Integer.eq_mod(&Integer, &Integer)", &mut |(x, y, m)| {
                no_out!(x.eq_mod(&y, &m))
            }),
            ("(&Integer).eq_mod(Integer, Natural)", &mut |(x, y, m)| {
                no_out!((&x).eq_mod(y, m))
            }),
            ("(&Integer).eq_mod(Integer, &Integer)", &mut |(x, y, m)| {
                no_out!((&x).eq_mod(y, &m))
            }),
            ("(&Integer).eq_mod(&Integer, Integer)", &mut |(x, y, m)| {
                no_out!((&x).eq_mod(&y, m))
            }),
            ("(&Integer).eq_mod(&Integer, &Integer)", &mut |(x, y, m)| {
                no_out!((&x).eq_mod(&y, &m))
            }),
        ],
    );
}

fn benchmark_integer_eq_mod_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.eq_mod(Integer, Natural)",
        BenchmarkType::LibraryComparison,
        integer_integer_natural_triple_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y, m))| no_out!(x.eq_mod(y, m))),
            ("rug", &mut |((x, y, m), _)| no_out!(x.is_congruent(&y, &m))),
        ],
    );
}

#[allow(clippy::short_circuit_statement, unused_must_use)]
fn benchmark_integer_eq_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.eq_mod(Integer, Natural)",
        BenchmarkType::Algorithms,
        integer_integer_natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Integer.eq_mod(Integer, Natural)", &mut |(x, y, m)| {
                no_out!(x.eq_mod(y, m))
            }),
            (
                "Integer == Integer || Integer != 0 && Integer % Natural == Integer % Natural",
                &mut |(x, y, m)| {
                    no_out!(x == y || m != 0 && x.unsigned_abs() % &m == y.unsigned_abs() % m)
                },
            ),
            (
                "(Integer - Integer).divisible_by(Natural)",
                &mut |(x, y, m)| no_out!((x - y).divisible_by(Integer::from(m))),
            ),
        ],
    );
}
