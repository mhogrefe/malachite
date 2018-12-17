use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1;
use inputs::integer::{
    rm_triples_of_integer_unsigned_and_unsigned, triples_of_integer_unsigned_and_unsigned,
    triples_of_unsigned_integer_and_unsigned,
};
use malachite_base::num::{DivisibleBy, EqMod, Mod, SignificantBits};
use malachite_nz::integer::arithmetic::eq_u32_mod_u32::limbs_eq_limb_mod_neg_limb;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_limb_mod_neg_limb);
    register_demo!(registry, demo_integer_eq_u32_mod_u32);
    register_demo!(registry, demo_u32_eq_integer_mod_u32);
    register_bench!(registry, Small, benchmark_limbs_eq_limb_mod_neg_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_u32_mod_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_eq_u32_mod_u32_algorithms);
    register_bench!(registry, Large, benchmark_u32_eq_integer_mod_u32_algorithms);
}

fn demo_limbs_eq_limb_mod_neg_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb, modulus) in
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1(gm).take(limit)
    {
        println!(
            "limbs_eq_limb_mod_neg_limb({:?}, {}, {}) = {}",
            limbs,
            limb,
            modulus,
            limbs_eq_limb_mod_neg_limb(&limbs, limb, modulus)
        );
    }
}

fn demo_integer_eq_u32_mod_u32(gm: GenerationMode, limit: usize) {
    for (n, u, modulus) in triples_of_integer_unsigned_and_unsigned::<u32>(gm).take(limit) {
        if n.eq_mod(u, modulus) {
            println!("{} is equal to {} mod {}", n, u, modulus);
        } else {
            println!("{} is not equal to {} mod {}", n, u, modulus);
        }
    }
}

fn demo_u32_eq_integer_mod_u32(gm: GenerationMode, limit: usize) {
    for (u, n, modulus) in triples_of_unsigned_integer_and_unsigned::<u32>(gm).take(limit) {
        if u.eq_mod(&n, modulus) {
            println!("{} is equal to {} mod {}", u, n, modulus);
        } else {
            println!("{} is not equal to {} mod {}", u, n, modulus);
        }
    }
}

fn benchmark_limbs_eq_limb_mod_neg_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_limb_mod_neg_limb(&mut [u32], u32, u32)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "limbs_eq_limb_mod_neg_limb",
            &mut (|(ref limbs, limb, modulus)| {
                no_out!(limbs_eq_limb_mod_neg_limb(limbs, limb, modulus))
            }),
        )],
    );
}

fn benchmark_integer_eq_u32_mod_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod(u32, u32)",
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

fn benchmark_integer_eq_u32_mod_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.eq_mod(u32, u32)",
        BenchmarkType::Algorithms,
        triples_of_integer_unsigned_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.eq_mod(u32, u32)",
                &mut (|(n, u, modulus)| no_out!(n.eq_mod(u, modulus))),
            ),
            (
                "Integer == u32 || u32 != 0 && Integer.mod(u32) == u32.mod(u32)",
                &mut (|(n, u, modulus)| {
                    no_out!(n == u || modulus != 0 && n.mod_op(modulus) == u.mod_op(modulus))
                }),
            ),
            (
                "(Integer - u32).divisible_by(u32)",
                &mut (|(n, u, modulus)| no_out!((n - u).divisible_by(modulus))),
            ),
        ],
    );
}

fn benchmark_u32_eq_integer_mod_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32.eq_mod(&Integer, u32)",
        BenchmarkType::Algorithms,
        triples_of_unsigned_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.eq_mod(&Integer, u32)",
                &mut (|(u, ref n, modulus)| no_out!(u.eq_mod(n, modulus))),
            ),
            (
                "u32 == Integer || u32 != 0 && u32.mod_op(u32) == Integer.mod_op(u32)",
                &mut (|(n, u, modulus)| {
                    no_out!(u == n || modulus != 0 && u.mod_op(modulus) == n.mod_op(modulus))
                }),
            ),
        ],
    );
}
