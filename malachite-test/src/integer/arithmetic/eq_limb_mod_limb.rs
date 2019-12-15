use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::arithmetic::eq_limb_mod_limb::limbs_eq_neg_limb_mod_limb;
use malachite_nz::integer::Integer;
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
        &(|&(_, (ref n, _, _))| usize::checked_from(n.significant_bits()).unwrap()),
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
        &(|&(ref n, _, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Integer.eq_mod(Limb, Limb)",
                &mut (|(n, u, modulus)| no_out!(n.eq_mod(u, modulus))),
            ),
            (
                "(Integer - Limb).divisible_by(Limb)",
                &mut (|(n, u, modulus)| no_out!((n - Integer::from(u)).divisible_by(modulus))),
            ),
        ],
    );
}
