use std::cmp::max;

use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::arithmetic::eq_mod::{
    limbs_eq_neg_limb_mod_limb, limbs_pos_eq_neg_limb_mod, limbs_pos_eq_neg_limb_mod_ref,
    limbs_pos_eq_neg_mod, limbs_pos_eq_neg_mod_limb, limbs_pos_eq_neg_mod_ref,
    limbs_pos_limb_eq_neg_limb_mod,
};
use malachite_nz::integer::Integer;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    triples_of_unsigned_unsigned_and_unsigned_vec_var_1,
    triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8, triples_of_unsigned_vec_var_55,
};
use inputs::integer::{
    rm_triples_of_integer_integer_and_natural, triples_of_integer_integer_and_natural,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_neg_limb_mod_limb);
    register_demo!(registry, demo_limbs_pos_limb_eq_neg_limb_mod);
    register_demo!(registry, demo_limbs_pos_eq_neg_limb_mod);
    register_demo!(registry, demo_limbs_pos_eq_neg_limb_mod_ref);
    register_demo!(registry, demo_limbs_pos_eq_neg_mod_limb);
    register_demo!(registry, demo_limbs_pos_eq_neg_mod);
    register_demo!(registry, demo_limbs_pos_eq_neg_mod_ref);
    register_demo!(registry, demo_integer_eq_mod);
    register_demo!(registry, demo_integer_eq_mod_val_val_ref);
    register_demo!(registry, demo_integer_eq_mod_val_ref_val);
    register_demo!(registry, demo_integer_eq_mod_val_ref_ref);
    register_demo!(registry, demo_integer_eq_mod_ref_val_val);
    register_demo!(registry, demo_integer_eq_mod_ref_val_ref);
    register_demo!(registry, demo_integer_eq_mod_ref_ref_val);
    register_demo!(registry, demo_integer_eq_mod_ref_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_eq_neg_limb_mod_limb);
    register_bench!(registry, Small, benchmark_limbs_pos_limb_eq_neg_limb_mod);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_pos_eq_neg_limb_mod_evaluation_strategy
    );
    register_bench!(registry, Small, benchmark_limbs_pos_eq_neg_mod_limb);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_pos_eq_neg_mod_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_mod_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_eq_mod_library_comparison);
    register_bench!(registry, Large, benchmark_integer_eq_mod_algorithms);
}

fn demo_limbs_eq_neg_limb_mod_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb, modulus) in
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1(gm).take(limit)
    {
        println!(
            "limbs_eq_neg_limb_mod_limb({:?}, {}, {}) = {}",
            limbs,
            limb,
            modulus,
            limbs_eq_neg_limb_mod_limb(&limbs, limb, modulus)
        );
    }
}

fn demo_limbs_pos_limb_eq_neg_limb_mod(gm: GenerationMode, limit: usize) {
    for (x, y, modulus) in triples_of_unsigned_unsigned_and_unsigned_vec_var_1(gm).take(limit) {
        println!(
            "limbs_pos_limb_eq_neg_limb_mod({}, {}, {:?}) = {}",
            x,
            y,
            modulus,
            limbs_pos_limb_eq_neg_limb_mod(x, y, &modulus)
        );
    }
}

fn demo_limbs_pos_eq_neg_limb_mod(gm: GenerationMode, limit: usize) {
    for (xs, y, mut modulus) in
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm).take(limit)
    {
        let old_modulus = modulus.clone();
        println!(
            "limbs_pos_eq_neg_limb_mod({:?}, {}, {:?}) = {}",
            xs,
            y,
            old_modulus,
            limbs_pos_eq_neg_limb_mod(&xs, y, &mut modulus)
        );
    }
}

fn demo_limbs_pos_eq_neg_limb_mod_ref(gm: GenerationMode, limit: usize) {
    for (xs, y, modulus) in triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm).take(limit)
    {
        println!(
            "limbs_pos_eq_neg_limb_mod_ref({:?}, {}, {:?}) = {}",
            xs,
            y,
            modulus,
            limbs_pos_eq_neg_limb_mod_ref(&xs, y, &modulus)
        );
    }
}

fn demo_limbs_pos_eq_neg_mod_limb(gm: GenerationMode, limit: usize) {
    for (xs, ys, modulus) in triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm).take(limit)
    {
        println!(
            "limbs_pos_eq_neg_mod_limb({:?}, {:?}, {}) = {}",
            xs,
            ys,
            modulus,
            limbs_pos_eq_neg_mod_limb(&xs, &ys, modulus)
        );
    }
}

fn demo_limbs_pos_eq_neg_mod(gm: GenerationMode, limit: usize) {
    for (xs, ys, mut modulus) in triples_of_unsigned_vec_var_55(gm).take(limit) {
        let old_modulus = modulus.clone();
        println!(
            "limbs_pos_eq_neg_mod({:?}, {:?}, {:?}) = {}",
            xs,
            ys,
            old_modulus,
            limbs_pos_eq_neg_mod(&xs, &ys, &mut modulus)
        );
    }
}

fn demo_limbs_pos_eq_neg_mod_ref(gm: GenerationMode, limit: usize) {
    for (xs, ys, modulus) in triples_of_unsigned_vec_var_55(gm).take(limit) {
        println!(
            "limbs_pos_eq_neg_mod_ref({:?}, {:?}, {:?}) = {}",
            xs,
            ys,
            modulus,
            limbs_pos_eq_neg_mod_ref(&xs, &ys, &modulus)
        );
    }
}

fn demo_integer_eq_mod(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_integer_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        if x.eq_mod(y, m) {
            println!("{} is equal to {} mod {}", x_old, y_old, m_old);
        } else {
            println!("{} is not equal to {} mod {}", x_old, y_old, m_old);
        }
    }
}

fn demo_integer_eq_mod_val_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_integer_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        if x.eq_mod(y, &m) {
            println!("{} is equal to {} mod &{}", x_old, y_old, m);
        } else {
            println!("{} is not equal to {} mod &{}", x_old, y_old, m);
        }
    }
}

fn demo_integer_eq_mod_val_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_integer_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        if x.eq_mod(&y, m) {
            println!("{} is equal to &{} mod {}", x_old, y, m_old);
        } else {
            println!("{} is not equal to &{} mod {}", x_old, y, m_old);
        }
    }
}

fn demo_integer_eq_mod_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_integer_integer_and_natural(gm).take(limit) {
        let x_old = x.clone();
        if x.eq_mod(&y, &m) {
            println!("{} is equal to &{} mod &{}", x_old, y, m);
        } else {
            println!("{} is not equal to &{} mod &{}", x_old, y, m);
        }
    }
}

fn demo_integer_eq_mod_ref_val_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_integer_integer_and_natural(gm).take(limit) {
        let y_old = y.clone();
        let m_old = m.clone();
        if (&x).eq_mod(y, m) {
            println!("&{} is equal to {} mod {}", x, y_old, m_old);
        } else {
            println!("&{} is not equal to {} mod {}", x, y_old, m_old);
        }
    }
}

fn demo_integer_eq_mod_ref_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_integer_integer_and_natural(gm).take(limit) {
        let y_old = y.clone();
        if (&x).eq_mod(y, &m) {
            println!("&{} is equal to {} mod &{}", x, y_old, m);
        } else {
            println!("&{} is not equal to {} mod &{}", x, y_old, m);
        }
    }
}

fn demo_integer_eq_mod_ref_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_integer_integer_and_natural(gm).take(limit) {
        let m_old = m.clone();
        if (&x).eq_mod(&y, m) {
            println!("&{} is equal to &{} mod {}", x, y, m_old);
        } else {
            println!("&{} is not equal to &{} mod {}", x, y, m_old);
        }
    }
}

fn demo_integer_eq_mod_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_integer_integer_and_natural(gm).take(limit) {
        if (&x).eq_mod(&y, &m) {
            println!("&{} is equal to &{} mod &{}", x, y, m);
        } else {
            println!("&{} is not equal to &{} mod &{}", x, y, m);
        }
    }
}

fn benchmark_limbs_eq_neg_limb_mod_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_neg_limb_mod_limb(&mut [Limb], Limb, Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "limbs_eq_neg_limb_mod_limb",
            &mut (|(ref limbs, limb, modulus)| {
                no_out!(limbs_eq_neg_limb_mod_limb(limbs, limb, modulus))
            }),
        )],
    );
}

fn benchmark_limbs_pos_limb_eq_neg_limb_mod(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_pos_limb_eq_neg_limb_mod(Limb, Limb, &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_and_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref modulus)| modulus.len()),
        "modulus.len()",
        &mut [(
            "limbs_pos_limb_eq_neg_limb_mod",
            &mut (|(x, y, ref modulus)| no_out!(limbs_pos_limb_eq_neg_limb_mod(x, y, modulus))),
        )],
    );
}

fn benchmark_limbs_pos_eq_neg_limb_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_eq_mod_limb(&[Limb], Limb, &[Limb])",
        BenchmarkType::EvaluationStrategy,
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "limbs_pos_eq_neg_limb_mod",
                &mut (|(ref xs, y, ref mut modulus)| {
                    no_out!(limbs_pos_eq_neg_limb_mod(xs, y, modulus))
                }),
            ),
            (
                "limbs_pos_eq_neg_limb_mod_ref",
                &mut (|(ref xs, y, ref modulus)| {
                    no_out!(limbs_pos_eq_neg_limb_mod_ref(xs, y, modulus))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_pos_eq_neg_mod_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_pos_eq_neg_mod_limb(&[Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "limbs_pos_eq_neg_mod_limb",
            &mut (|(ref x, ref y, modulus)| no_out!(limbs_pos_eq_neg_mod_limb(x, y, modulus))),
        )],
    );
}

fn benchmark_limbs_pos_eq_neg_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_eq_mod_limb(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::EvaluationStrategy,
        triples_of_unsigned_vec_var_55(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [
            (
                "limbs_pos_eq_neg_mod",
                &mut (|(ref xs, ref y, ref mut modulus)| {
                    no_out!(limbs_pos_eq_neg_mod(xs, y, modulus))
                }),
            ),
            (
                "limbs_pos_eq_neg_mod_ref",
                &mut (|(ref xs, ref y, ref modulus)| {
                    no_out!(limbs_pos_eq_neg_mod_ref(xs, y, modulus))
                }),
            ),
        ],
    );
}

fn benchmark_integer_eq_mod_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.eq_mod(Integer, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "Integer.eq_mod(Integer, Natural)",
                &mut (|(x, y, m)| no_out!(x.eq_mod(y, m))),
            ),
            (
                "Integer.eq_mod(Integer, &Integer)",
                &mut (|(x, y, m)| no_out!(x.eq_mod(y, &m))),
            ),
            (
                "Integer.eq_mod(&Integer, Integer)",
                &mut (|(x, y, m)| no_out!(x.eq_mod(&y, m))),
            ),
            (
                "Integer.eq_mod(&Integer, &Integer)",
                &mut (|(x, y, m)| no_out!(x.eq_mod(&y, &m))),
            ),
            (
                "(&Integer).eq_mod(Integer, Natural)",
                &mut (|(x, y, m)| no_out!((&x).eq_mod(y, m))),
            ),
            (
                "(&Integer).eq_mod(Integer, &Integer)",
                &mut (|(x, y, m)| no_out!((&x).eq_mod(y, &m))),
            ),
            (
                "(&Integer).eq_mod(&Integer, Integer)",
                &mut (|(x, y, m)| no_out!((&x).eq_mod(&y, m))),
            ),
            (
                "(&Integer).eq_mod(&Integer, &Integer)",
                &mut (|(x, y, m)| no_out!((&x).eq_mod(&y, &m))),
            ),
        ],
    );
}

fn benchmark_integer_eq_mod_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.eq_mod(Integer, Natural)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_integer_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y, _))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (x, y, m))| no_out!(x.eq_mod(y, m)))),
            (
                "rug",
                &mut (|((x, y, m), _)| no_out!(x.is_congruent(&y, &m))),
            ),
        ],
    );
}

fn benchmark_integer_eq_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.eq_mod(Integer, Natural)",
        BenchmarkType::Algorithms,
        triples_of_integer_integer_and_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y, _)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "Integer.eq_mod(Integer, Natural)",
                &mut (|(x, y, m)| no_out!(x.eq_mod(y, m))),
            ),
            (
                "Integer == Integer || Integer != 0 && Integer % Natural == Integer % Natural",
                &mut (|(x, y, m)| {
                    no_out!(
                        x == y
                            || m != Integer::ZERO && x.unsigned_abs() % &m == y.unsigned_abs() % m
                    )
                }),
            ),
            (
                "|Integer - Integer|.divisible_by(Natural)",
                &mut (|(x, y, m)| {
                    no_out!((Integer::from(x) - Integer::from(y))
                        .unsigned_abs()
                        .divisible_by(m))
                }),
            ),
        ],
    );
}
