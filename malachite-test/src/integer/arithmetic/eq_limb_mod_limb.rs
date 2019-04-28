use malachite_base::num::traits::{DivisibleBy, EqMod, Mod, SignificantBits};
use malachite_nz::integer::arithmetic::eq_limb_mod_limb::limbs_eq_neg_limb_mod_limb;
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1;
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::rm_triples_of_integer_unsigned_and_unsigned;
use inputs::integer::{
    triples_of_integer_unsigned_and_unsigned, triples_of_unsigned_integer_and_unsigned,
    triples_of_unsigned_unsigned_and_integer,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_neg_limb_mod_limb);
    register_demo!(registry, demo_integer_eq_limb_mod_limb);
    register_demo!(registry, demo_limb_eq_integer_mod_limb);
    register_demo!(registry, demo_limb_eq_limb_mod_integer);
    register_bench!(registry, Small, benchmark_limbs_eq_neg_limb_mod_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_limb_mod_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_limb_mod_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_eq_integer_mod_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_eq_limb_mod_integer_algorithms
    );
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

fn demo_integer_eq_limb_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u, modulus) in triples_of_integer_unsigned_and_unsigned::<Limb>(gm).take(limit) {
        if n.eq_mod(u, modulus) {
            println!("{} is equal to {} mod {}", n, u, modulus);
        } else {
            println!("{} is not equal to {} mod {}", n, u, modulus);
        }
    }
}

fn demo_limb_eq_integer_mod_limb(gm: GenerationMode, limit: usize) {
    for (u, n, modulus) in triples_of_unsigned_integer_and_unsigned::<Limb>(gm).take(limit) {
        if u.eq_mod(&n, modulus) {
            println!("{} is equal to {} mod {}", u, n, modulus);
        } else {
            println!("{} is not equal to {} mod {}", u, n, modulus);
        }
    }
}

fn demo_limb_eq_limb_mod_integer(gm: GenerationMode, limit: usize) {
    for (u, v, modulus) in triples_of_unsigned_unsigned_and_integer::<Limb>(gm).take(limit) {
        if u.eq_mod(v, &modulus) {
            println!("{} is equal to {} mod {}", u, v, modulus);
        } else {
            println!("{} is not equal to {} mod {}", u, v, modulus);
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

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_eq_limb_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod(Limb, Limb)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_integer_unsigned_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (n, u, modulus))| no_out!(n.eq_mod(u, modulus))),
            ),
            (
                "rug",
                &mut (|((n, u, modulus), _)| no_out!(n.is_congruent_u(u, modulus))),
            ),
        ],
    );
}

fn benchmark_integer_eq_limb_mod_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod(Limb, Limb)",
        BenchmarkType::Algorithms,
        triples_of_integer_unsigned_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.eq_mod(Limb, Limb)",
                &mut (|(n, u, modulus)| no_out!(n.eq_mod(u, modulus))),
            ),
            (
                "Integer == Limb || Limb != 0 && Integer.mod(Limb) == Limb.mod(Limb)",
                &mut (|(n, u, modulus)| {
                    no_out!(n == u || modulus != 0 && n.mod_op(modulus) == u.mod_op(modulus))
                }),
            ),
            (
                "(Integer - Limb).divisible_by(Limb)",
                &mut (|(n, u, modulus)| no_out!((n - u).divisible_by(modulus))),
            ),
        ],
    );
}

fn benchmark_limb_eq_integer_mod_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.eq_mod(&Integer, Limb)",
        BenchmarkType::Algorithms,
        triples_of_unsigned_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.eq_mod(&Integer, Limb)",
                &mut (|(u, ref n, modulus)| no_out!(u.eq_mod(n, modulus))),
            ),
            (
                "Limb == Integer || Limb != 0 && Limb.mod_op(Limb) == Integer.mod_op(Limb)",
                &mut (|(n, u, modulus)| {
                    no_out!(u == n || modulus != 0 && u.mod_op(modulus) == n.mod_op(modulus))
                }),
            ),
        ],
    );
}

fn benchmark_limb_eq_limb_mod_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.eq_mod(Limb, &Integer)",
        BenchmarkType::Algorithms,
        triples_of_unsigned_unsigned_and_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.eq_mod(Limb, &Integer)",
                &mut (|(u, v, ref modulus)| no_out!(u.eq_mod(v, modulus))),
            ),
            (
                "Limb == Limb || Integer != 0 && Limb.mod_op(&Integer) == Limb.mod_op(&Integer)",
                &mut (|(u, v, modulus)| {
                    no_out!(
                        u == v || modulus != 0 as Limb && u.mod_op(&modulus) == v.mod_op(&modulus)
                    )
                }),
            ),
        ],
    );
}
