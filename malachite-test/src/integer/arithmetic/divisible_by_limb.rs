use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
#[cfg(not(feature = "32_bit_limbs"))]
use inputs::integer::nm_pairs_of_integer_and_unsigned;
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::nrm_pairs_of_integer_and_unsigned;
use inputs::integer::{pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer};
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::platform::Limb;
use num::{BigInt, Integer, Zero};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_divisible_by_limb);
    register_demo!(registry, demo_limb_divisible_by_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_divisible_by_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_limb_divisible_by_integer);
}

pub fn num_divisible_by_limb(x: BigInt, u: Limb) -> bool {
    x == BigInt::zero() || u != 0 && x.is_multiple_of(&BigInt::from(u))
}

fn demo_integer_divisible_by_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        if n.divisible_by(u) {
            println!("{} is divisible by {}", n, u);
        } else {
            println!("{} is not divisible by {}", n, u);
        }
    }
}

fn demo_limb_divisible_by_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<Limb>(gm).take(limit) {
        if u.divisible_by(&n) {
            println!("{} is divisible by {}", u, n);
        } else {
            println!("{} is not divisible by {}", u, n);
        }
    }
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_divisible_by_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.divisible_by(Limb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.divisible_by(y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_divisible_by_limb(x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.is_divisible_u(y)))),
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
fn benchmark_integer_divisible_by_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.divisible_by(Limb)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.divisible_by(y)))),
            (
                "num",
                &mut (|((x, y), _)| no_out!(num_divisible_by_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_limb_divisible_by_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Limb.divisible_by(&Integer)",
        BenchmarkType::Single,
        pairs_of_unsigned_and_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [(
            "Limb.divisible_by(&Integer)",
            &mut (|(x, ref y)| no_out!(x.divisible_by(y))),
        )],
    );
}
