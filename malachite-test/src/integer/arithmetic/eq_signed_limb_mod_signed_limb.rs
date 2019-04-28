use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    rm_triples_of_integer_signed_and_signed, triples_of_integer_signed_and_signed,
    triples_of_signed_integer_and_signed, triples_of_signed_signed_and_integer,
};
use malachite_base::num::traits::{DivisibleBy, EqMod, Mod, SignificantBits};
use malachite_nz::platform::{Limb, SignedLimb};
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_eq_signed_limb_mod_signed_limb);
    register_demo!(registry, demo_signed_limb_eq_integer_mod_signed_limb);
    register_demo!(registry, demo_signed_limb_eq_signed_limb_mod_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_signed_limb_mod_signed_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_signed_limb_mod_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_eq_integer_mod_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_eq_signed_limb_mod_integer_algorithms
    );
}

pub fn rug_eq_signed_limb_mod_signed_limb(n: rug::Integer, i: SignedLimb, m: SignedLimb) -> bool {
    n.is_congruent(&rug::Integer::from(i), &rug::Integer::from(m))
}

fn demo_integer_eq_signed_limb_mod_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i, modulus) in triples_of_integer_signed_and_signed::<SignedLimb>(gm).take(limit) {
        if n.eq_mod(i, modulus) {
            println!("{} is equal to {} mod {}", n, i, modulus);
        } else {
            println!("{} is not equal to {} mod {}", n, i, modulus);
        }
    }
}

fn demo_signed_limb_eq_integer_mod_signed_limb(gm: GenerationMode, limit: usize) {
    for (i, n, modulus) in triples_of_signed_integer_and_signed::<SignedLimb>(gm).take(limit) {
        if i.eq_mod(&n, modulus) {
            println!("{} is equal to {} mod {}", i, n, modulus);
        } else {
            println!("{} is not equal to {} mod {}", i, n, modulus);
        }
    }
}

fn demo_signed_limb_eq_signed_limb_mod_integer(gm: GenerationMode, limit: usize) {
    for (i, j, modulus) in triples_of_signed_signed_and_integer::<SignedLimb>(gm).take(limit) {
        if i.eq_mod(j, &modulus) {
            println!("{} is equal to {} mod {}", i, j, modulus);
        } else {
            println!("{} is not equal to {} mod {}", i, j, modulus);
        }
    }
}

fn benchmark_integer_eq_signed_limb_mod_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod(SignedLimb, SignedLimb)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_integer_signed_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (n, i, modulus))| no_out!(n.eq_mod(i, modulus))),
            ),
            (
                "rug",
                &mut (|((n, i, modulus), _)| {
                    no_out!(rug_eq_signed_limb_mod_signed_limb(n, i, modulus))
                }),
            ),
        ],
    );
}

fn benchmark_integer_eq_signed_limb_mod_signed_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod(SignedLimb, SignedLimb)",
        BenchmarkType::Algorithms,
        triples_of_integer_signed_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.eq_mod(SignedLimb, SignedLimb)",
                &mut (|(n, i, modulus)| no_out!(n.eq_mod(i, modulus))),
            ),
            (
                "Integer == SignedLimb || SignedLimb != 0 && Integer.mod(SignedLimb) == \
                 SignedLimb.mod(SignedLimb)",
                &mut (|(n, i, modulus)| {
                    no_out!(n == i || modulus != 0 && n.mod_op(modulus) == i.mod_op(modulus))
                }),
            ),
            (
                "(Integer - SignedLimb).divisible_by(SignedLimb)",
                &mut (|(n, i, modulus)| no_out!((n - i).divisible_by(modulus))),
            ),
        ],
    );
}

fn benchmark_signed_limb_eq_integer_mod_signed_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.eq_mod(&Integer, SignedLimb)",
        BenchmarkType::Algorithms,
        triples_of_signed_integer_and_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb.eq_mod(&Integer, SignedLimb)",
                &mut (|(i, ref n, modulus)| no_out!(i.eq_mod(n, modulus))),
            ),
            (
                "SignedLimb == Integer || SignedLimb != 0 && SignedLimb.mod_op(SignedLimb) == \
                 Integer.mod_op(SignedLimb)",
                &mut (|(n, i, modulus)| {
                    no_out!(i == n || modulus != 0 && i.mod_op(modulus) == n.mod_op(modulus))
                }),
            ),
        ],
    );
}

fn benchmark_signed_limb_eq_signed_limb_mod_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.eq_mod(SignedLimb, &Integer)",
        BenchmarkType::Algorithms,
        triples_of_signed_signed_and_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.eq_mod(SignedLimb, &Integer)",
                &mut (|(i, j, ref modulus)| no_out!(i.eq_mod(j, modulus))),
            ),
            (
                "SignedLimb == SignedLimb || Integer != 0 && SignedLimb.mod_op(&Integer) == \
                 SignedLimb.mod_op(&Integer)",
                &mut (|(i, j, modulus)| {
                    no_out!(
                        i == j || modulus != 0 as Limb && i.mod_op(&modulus) == j.mod_op(&modulus)
                    )
                }),
            ),
        ],
    );
}
