use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use rug;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    rm_triples_of_integer_signed_and_signed, triples_of_integer_signed_and_signed,
    triples_of_signed_integer_and_signed, triples_of_signed_signed_and_integer,
};

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
        &(|&(_, (ref n, _, _))| usize::checked_from(n.significant_bits()).unwrap()),
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
        &(|&(ref n, _, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Integer.eq_mod(SignedLimb, SignedLimb)",
                &mut (|(n, i, modulus)| no_out!(n.eq_mod(i, modulus))),
            ),
            (
                "(Integer - SignedLimb).divisible_by(SignedLimb)",
                &mut (|(n, i, modulus)| no_out!((n - Integer::from(i)).divisible_by(modulus))),
            ),
        ],
    );
}
