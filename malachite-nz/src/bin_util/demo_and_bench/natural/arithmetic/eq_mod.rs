// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, UnsignedAbs};
use malachite_base::test_util::bench::bucketers::{
    triple_1_2_vec_max_len_bucketer, triple_1_bit_bucketer, triple_1_vec_len_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_triple_gen_var_21, unsigned_vec_triple_gen_var_36,
    unsigned_vec_unsigned_unsigned_triple_gen_var_10,
    unsigned_vec_unsigned_unsigned_triple_gen_var_7,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6,
};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::eq_mod::{
    limbs_eq_limb_mod, limbs_eq_limb_mod_limb, limbs_eq_limb_mod_ref_ref,
    limbs_eq_limb_mod_ref_val, limbs_eq_limb_mod_val_ref, limbs_eq_mod_limb_ref_ref,
    limbs_eq_mod_limb_ref_val, limbs_eq_mod_limb_val_ref, limbs_eq_mod_ref_ref_ref,
    limbs_eq_mod_ref_ref_val, limbs_eq_mod_ref_val_ref, limbs_eq_mod_ref_val_val,
    limbs_limb_mod_exact_odd_limb, limbs_mod_exact_odd_limb,
};
use malachite_nz::natural::arithmetic::mod_op::limbs_mod_limb;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_triple_1_2_natural_max_bit_bucketer, triple_1_2_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_triple_gen, natural_triple_gen_rm};
use malachite_nz::test_util::natural::arithmetic::eq_mod::{
    combined_limbs_eq_limb_mod_limb, limbs_eq_limb_mod_naive_1, limbs_eq_limb_mod_naive_2,
    limbs_eq_mod_limb_naive_1, limbs_eq_mod_limb_naive_2, limbs_eq_mod_naive_1,
    limbs_eq_mod_naive_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_limb_mod_exact_odd_limb);
    register_demo!(runner, demo_limbs_mod_exact_odd_limb);
    register_demo!(runner, demo_limbs_eq_limb_mod_limb);
    register_demo!(runner, demo_limbs_eq_limb_mod);
    register_demo!(runner, demo_limbs_eq_limb_mod_val_ref);
    register_demo!(runner, demo_limbs_eq_limb_mod_ref_val);
    register_demo!(runner, demo_limbs_eq_limb_mod_ref_ref);
    register_demo!(runner, demo_limbs_eq_mod_limb_val_ref);
    register_demo!(runner, demo_limbs_eq_mod_limb_ref_val);
    register_demo!(runner, demo_limbs_eq_mod_limb_ref_ref);
    register_demo!(runner, demo_limbs_eq_mod_ref_val_val);
    register_demo!(runner, demo_limbs_eq_mod_ref_val_ref);
    register_demo!(runner, demo_limbs_eq_mod_ref_ref_val);
    register_demo!(runner, demo_limbs_eq_mod_ref_ref_ref);
    register_demo!(runner, demo_natural_eq_mod);
    register_demo!(runner, demo_natural_eq_mod_val_val_ref);
    register_demo!(runner, demo_natural_eq_mod_val_ref_val);
    register_demo!(runner, demo_natural_eq_mod_val_ref_ref);
    register_demo!(runner, demo_natural_eq_mod_ref_val_val);
    register_demo!(runner, demo_natural_eq_mod_ref_val_ref);
    register_demo!(runner, demo_natural_eq_mod_ref_ref_val);
    register_demo!(runner, demo_natural_eq_mod_ref_ref_ref);

    register_bench!(runner, benchmark_limbs_limb_mod_exact_odd_limb);
    register_bench!(runner, benchmark_limbs_mod_exact_odd_limb);
    register_bench!(runner, benchmark_limbs_eq_limb_mod_limb_algorithms);
    register_bench!(runner, benchmark_limbs_eq_limb_mod_evaluation_strategy);
    register_bench!(runner, benchmark_limbs_eq_limb_mod_algorithms);
    register_bench!(runner, benchmark_limbs_eq_mod_limb_evaluation_strategy);
    register_bench!(runner, benchmark_limbs_eq_mod_limb_algorithms);
    register_bench!(runner, benchmark_limbs_eq_mod_evaluation_strategy);
    register_bench!(runner, benchmark_limbs_eq_mod_algorithms);
    register_bench!(runner, benchmark_natural_eq_mod_evaluation_strategy);
    register_bench!(runner, benchmark_natural_eq_mod_library_comparison);
    register_bench!(runner, benchmark_natural_eq_mod_algorithms);
}

fn demo_limbs_limb_mod_exact_odd_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, d, carry) in unsigned_triple_gen_var_21().get(gm, config).take(limit) {
        println!(
            "limbs_limb_mod_exact_odd_limb({}, {}, {}) = {}",
            n,
            d,
            carry,
            limbs_limb_mod_exact_odd_limb(n, d, carry)
        );
    }
}

fn demo_limbs_mod_exact_odd_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ns, d, carry) in unsigned_vec_unsigned_unsigned_triple_gen_var_10()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_mod_exact_odd_limb({:?}, {}, {}) = {}",
            ns,
            d,
            carry,
            limbs_mod_exact_odd_limb(&ns, d, carry)
        );
    }
}

fn demo_limbs_eq_limb_mod_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, y, m) in unsigned_vec_unsigned_unsigned_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_eq_limb_mod_limb({:?}, {}, {}) = {}",
            xs,
            y,
            m,
            limbs_eq_limb_mod_limb(&xs, y, m)
        );
    }
}

fn demo_limbs_eq_limb_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, mut ms, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let old_xs = xs.clone();
        let old_ms = ms.clone();
        println!(
            "limbs_eq_limb_mod({:?}, {}, {:?}) = {}",
            old_xs,
            y,
            old_ms,
            limbs_eq_limb_mod(&mut xs, y, &mut ms)
        );
    }
}

fn demo_limbs_eq_limb_mod_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ms, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let old_xs = xs.clone();
        println!(
            "limbs_eq_limb_mod_val_ref({:?}, {}, {:?}) = {}",
            old_xs,
            y,
            ms,
            limbs_eq_limb_mod_val_ref(&mut xs, y, &ms)
        );
    }
}

fn demo_limbs_eq_limb_mod_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, mut ms, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let old_ms = ms.clone();
        println!(
            "limbs_eq_limb_mod_ref_val({:?}, {}, {:?}) = {}",
            xs,
            y,
            old_ms,
            limbs_eq_limb_mod_ref_val(&xs, y, &mut ms)
        );
    }
}

fn demo_limbs_eq_limb_mod_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ms, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_eq_limb_mod_ref_ref({:?}, {}, {:?}) = {}",
            xs,
            y,
            ms,
            limbs_eq_limb_mod_ref_ref(&xs, y, &ms)
        );
    }
}

fn demo_limbs_eq_mod_limb_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, ys, m) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let old_xs = xs.clone();
        println!(
            "limbs_eq_mod_limb_val_ref({:?}, {:?}, {}) = {}",
            old_xs,
            ys,
            m,
            limbs_eq_mod_limb_val_ref(&mut xs, &ys, m)
        );
    }
}

fn demo_limbs_eq_mod_limb_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, mut ys, m) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let old_ys = ys.clone();
        println!(
            "limbs_eq_mod_limb_ref_val({:?}, {:?}, {}) = {}",
            xs,
            old_ys,
            m,
            limbs_eq_mod_limb_ref_val(&xs, &mut ys, m)
        );
    }
}

fn demo_limbs_eq_mod_limb_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, m) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "limbs_eq_mod_limb_ref_ref({:?}, {:?}, {}) = {}",
            xs,
            ys,
            m,
            limbs_eq_mod_limb_ref_ref(&xs, &ys, m)
        );
    }
}

fn demo_limbs_eq_mod_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, mut ys, mut ms) in unsigned_vec_triple_gen_var_36().get(gm, config).take(limit) {
        let old_ys = ys.clone();
        let old_ms = ms.clone();
        println!(
            "limbs_eq_mod_ref_val_val({:?}, {:?}, {:?}) = {}",
            xs,
            old_ys,
            old_ms,
            limbs_eq_mod_ref_val_val(&xs, &mut ys, &mut ms)
        );
    }
}

fn demo_limbs_eq_mod_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, mut ys, ms) in unsigned_vec_triple_gen_var_36().get(gm, config).take(limit) {
        let old_ys = ys.clone();
        println!(
            "limbs_eq_mod_ref_val_ref({:?}, {:?}, {:?}) = {}",
            xs,
            old_ys,
            ms,
            limbs_eq_mod_ref_val_ref(&xs, &mut ys, &ms)
        );
    }
}

fn demo_limbs_eq_mod_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, mut ms) in unsigned_vec_triple_gen_var_36().get(gm, config).take(limit) {
        let old_ms = ms.clone();
        println!(
            "limbs_eq_mod_ref_ref_val({:?}, {:?}, {:?}) = {}",
            xs,
            ys,
            old_ms,
            limbs_eq_mod_ref_ref_val(&xs, &ys, &mut ms)
        );
    }
}

fn demo_limbs_eq_mod_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, ms) in unsigned_vec_triple_gen_var_36().get(gm, config).take(limit) {
        println!(
            "limbs_eq_mod_ref_ref_ref({:?}, {:?}, {:?}) = {}",
            xs,
            ys,
            ms,
            limbs_eq_mod_ref_ref_ref(&xs, &ys, &ms)
        );
    }
}

fn demo_natural_eq_mod(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen().get(gm, config).take(limit) {
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

fn demo_natural_eq_mod_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        if x.eq_mod(y, &m) {
            println!("{x_old} is equal to {y_old} mod &{m}");
        } else {
            println!("{x_old} is not equal to {y_old} mod &{m}");
        }
    }
}

fn demo_natural_eq_mod_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        if x.eq_mod(&y, m) {
            println!("{x_old} is equal to &{y} mod {m_old}");
        } else {
            println!("{x_old} is not equal to &{y} mod {m_old}");
        }
    }
}

fn demo_natural_eq_mod_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        if x.eq_mod(&y, &m) {
            println!("{x_old} is equal to &{y} mod &{m}");
        } else {
            println!("{x_old} is not equal to &{y} mod &{m}");
        }
    }
}

fn demo_natural_eq_mod_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        let m_old = m.clone();
        if (&x).eq_mod(y, m) {
            println!("&{x} is equal to {y_old} mod {m_old}");
        } else {
            println!("&{x} is not equal to {y_old} mod {m_old}");
        }
    }
}

fn demo_natural_eq_mod_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        if (&x).eq_mod(y, &m) {
            println!("&{x} is equal to {y_old} mod &{m}");
        } else {
            println!("&{x} is not equal to {y_old} mod &{m}");
        }
    }
}

fn demo_natural_eq_mod_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen().get(gm, config).take(limit) {
        let m_old = m.clone();
        if (&x).eq_mod(&y, m) {
            println!("&{x} is equal to &{y} mod {m_old}");
        } else {
            println!("&{x} is not equal to &{y} mod {m_old}");
        }
    }
}

fn demo_natural_eq_mod_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, m) in natural_triple_gen().get(gm, config).take(limit) {
        if (&x).eq_mod(&y, &m) {
            println!("&{x} is equal to &{y} mod &{m}");
        } else {
            println!("&{x} is not equal to &{y} mod &{m}");
        }
    }
}

fn benchmark_limbs_limb_mod_exact_odd_limb(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_limb_mod_exact_odd_limb(Limb, Limb, Limb)",
        BenchmarkType::Single,
        unsigned_triple_gen_var_21().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, d, carry)| {
            no_out!(limbs_limb_mod_exact_odd_limb(n, d, carry))
        })],
    );
}

fn benchmark_limbs_mod_exact_odd_limb(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mod_exact_odd_limb(&[Limb], Limb, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("ns"),
        &mut [("Malachite", &mut |(ref ns, d, carry)| {
            no_out!(limbs_mod_exact_odd_limb(ns, d, carry))
        })],
    );
}

// use large params
#[allow(clippy::unnecessary_operation, unused_must_use)]
fn benchmark_limbs_eq_limb_mod_limb_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_limb_mod_limb(&mut [Limb], Limb, Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_unsigned_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [
            ("limbs_eq_limb_mod_limb", &mut |(ref xs, y, m)| {
                no_out!(limbs_eq_limb_mod_limb(xs, y, m))
            }),
            ("limbs_mod_limb", &mut |(ref xs, y, m)| {
                no_out!(limbs_mod_limb(xs, m) == y % m)
            }),
            ("combined_limbs_eq_limb_mod_limb", &mut |(ref xs, y, m)| {
                no_out!(combined_limbs_eq_limb_mod_limb(xs, y, m))
            }),
        ],
    );
}

fn benchmark_limbs_eq_limb_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_limb_mod(&[Limb], Limb, &[Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [
            ("limbs_eq_limb_mod", &mut |(ref mut xs, ref mut ms, y)| {
                no_out!(limbs_eq_limb_mod(xs, y, ms))
            }),
            ("limbs_eq_limb_mod_val_ref", &mut |(
                ref mut xs,
                ref mut ms,
                y,
            )| {
                no_out!(limbs_eq_limb_mod_val_ref(xs, y, ms))
            }),
            ("limbs_eq_limb_mod_ref_val", &mut |(
                ref xs,
                ref mut ms,
                y,
            )| {
                no_out!(limbs_eq_limb_mod_ref_val(xs, y, ms))
            }),
            ("limbs_eq_limb_mod_ref_ref", &mut |(
                ref xs,
                ref mut ms,
                y,
            )| {
                no_out!(limbs_eq_limb_mod_ref_ref(xs, y, ms))
            }),
        ],
    );
}

fn benchmark_limbs_eq_limb_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_limb_mod_ref_ref(&[Limb], Limb, &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [
            ("standard", &mut |(ref xs, ref ms, y)| {
                no_out!(limbs_eq_limb_mod_ref_ref(xs, y, ms))
            }),
            ("naive 1", &mut |(ref xs, ref ms, y)| {
                no_out!(limbs_eq_limb_mod_naive_1(xs, y, ms))
            }),
            ("naive 2", &mut |(ref xs, ref ms, y)| {
                no_out!(limbs_eq_limb_mod_naive_2(xs, y, ms))
            }),
        ],
    );
}

fn benchmark_limbs_eq_mod_limb_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_mod_limb_val_ref(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("limbs_eq_mod_limb_val_ref", &mut |(
                ref mut xs,
                ref ys,
                ms,
            )| {
                no_out!(limbs_eq_mod_limb_val_ref(xs, ys, ms))
            }),
            ("limbs_eq_mod_limb_ref_val", &mut |(
                ref xs,
                ref mut ys,
                ms,
            )| {
                no_out!(limbs_eq_mod_limb_ref_val(xs, ys, ms))
            }),
            ("limbs_eq_mod_limb_ref_ref", &mut |(ref xs, ref ys, ms)| {
                no_out!(limbs_eq_mod_limb_ref_ref(xs, ys, ms))
            }),
        ],
    );
}

fn benchmark_limbs_eq_mod_limb_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_mod_limb_val_ref(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("standard", &mut |(ref xs, ref ys, ms)| {
                no_out!(limbs_eq_mod_limb_ref_ref(xs, ys, ms))
            }),
            ("naive 1", &mut |(ref xs, ref ys, ms)| {
                no_out!(limbs_eq_mod_limb_naive_1(xs, ys, ms))
            }),
            ("naive 2", &mut |(ref xs, ref ys, ms)| {
                no_out!(limbs_eq_mod_limb_naive_2(xs, ys, ms))
            }),
        ],
    );
}

fn benchmark_limbs_eq_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_mod_ref_ref_ref(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("limbs_eq_mod_ref_val_val", &mut |(
                ref xs,
                ref mut ys,
                ref mut ms,
            )| {
                no_out!(limbs_eq_mod_ref_val_val(xs, ys, ms))
            }),
            ("limbs_eq_mod_ref_val_ref", &mut |(
                ref xs,
                ref mut ys,
                ref ms,
            )| {
                no_out!(limbs_eq_mod_ref_val_ref(xs, ys, ms))
            }),
            ("limbs_eq_mod_ref_ref_val", &mut |(
                ref xs,
                ref ys,
                ref mut ms,
            )| {
                no_out!(limbs_eq_mod_ref_ref_val(xs, ys, ms))
            }),
            ("limbs_eq_mod_ref_ref_ref", &mut |(
                ref xs,
                ref ys,
                ref ms,
            )| {
                no_out!(limbs_eq_mod_ref_ref_ref(xs, ys, ms))
            }),
        ],
    );
}

fn benchmark_limbs_eq_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_eq_mod_ref_ref_ref(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("standard", &mut |(ref xs, ref ys, ref ms)| {
                no_out!(limbs_eq_mod_ref_ref_ref(xs, ys, ms))
            }),
            ("naive 1", &mut |(ref xs, ref ys, ref ms)| {
                no_out!(limbs_eq_mod_naive_1(xs, ys, ms))
            }),
            ("naive 2", &mut |(ref xs, ref ys, ref ms)| {
                no_out!(limbs_eq_mod_naive_2(xs, ys, ms))
            }),
        ],
    );
}

fn benchmark_natural_eq_mod_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.eq_mod(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.eq_mod(Natural, Natural)", &mut |(x, y, m)| {
                no_out!(x.eq_mod(y, m))
            }),
            ("Natural.eq_mod(Natural, &Natural)", &mut |(x, y, m)| {
                no_out!(x.eq_mod(y, &m))
            }),
            ("Natural.eq_mod(&Natural, Natural)", &mut |(x, y, m)| {
                no_out!(x.eq_mod(&y, m))
            }),
            ("Natural.eq_mod(&Natural, &Natural)", &mut |(x, y, m)| {
                no_out!(x.eq_mod(&y, &m))
            }),
            ("(&Natural).eq_mod(Natural, Natural)", &mut |(x, y, m)| {
                no_out!((&x).eq_mod(y, m))
            }),
            ("(&Natural).eq_mod(Natural, &Natural)", &mut |(x, y, m)| {
                no_out!((&x).eq_mod(y, &m))
            }),
            ("(&Natural).eq_mod(&Natural, Natural)", &mut |(x, y, m)| {
                no_out!((&x).eq_mod(&y, m))
            }),
            ("(&Natural).eq_mod(&Natural, &Natural)", &mut |(x, y, m)| {
                no_out!((&x).eq_mod(&y, &m))
            }),
        ],
    );
}

fn benchmark_natural_eq_mod_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.eq_mod(Natural, Natural)",
        BenchmarkType::LibraryComparison,
        natural_triple_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y, m))| no_out!(x.eq_mod(y, m))),
            ("rug", &mut |((x, y, m), _)| no_out!(x.is_congruent(&y, &m))),
        ],
    );
}

#[allow(clippy::no_effect, clippy::short_circuit_statement, unused_must_use)]
fn benchmark_natural_eq_mod_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.eq_mod(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.eq_mod(Natural, Natural)", &mut |(x, y, m)| {
                no_out!(x.eq_mod(y, m))
            }),
            (
                "Natural == Natural || Natural != 0 && Natural % Natural == Natural % Natural",
                &mut |(x, y, m)| no_out!(x == y || m != 0 && x % &m == y % m),
            ),
            (
                "|Natural - Natural|.divisible_by(Natural)",
                &mut |(x, y, m)| {
                    no_out!((Integer::from(x) - Integer::from(y))
                        .unsigned_abs()
                        .divisible_by(m))
                },
            ),
        ],
    );
}
