use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_signed, pairs_of_integer_and_signed, pairs_of_signed_and_integer,
};
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::platform::SignedLimb;
use num::{BigInt, Integer, Zero};
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_divisible_by_signed_limb);
    register_demo!(registry, demo_signed_limb_divisible_by_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_divisible_by_signed_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_signed_limb_divisible_by_integer);
}

pub fn num_divisible_by_signed_limb(x: BigInt, i: SignedLimb) -> bool {
    x == BigInt::zero() || i != 0 && x.is_multiple_of(&BigInt::from(i))
}

pub fn rug_divisible_by_signed_limb(x: rug::Integer, i: SignedLimb) -> bool {
    x.is_divisible(&rug::Integer::from(i))
}

fn demo_integer_divisible_by_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<SignedLimb>(gm).take(limit) {
        if n.divisible_by(i) {
            println!("{} is divisible by {}", n, i);
        } else {
            println!("{} is not divisible by {}", n, i);
        }
    }
}

fn demo_signed_limb_divisible_by_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_integer::<SignedLimb>(gm).take(limit) {
        if i.divisible_by(&n) {
            println!("{} is divisible by {}", i, n);
        } else {
            println!("{} is not divisible by {}", i, n);
        }
    }
}

fn benchmark_integer_divisible_by_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.divisible_by(SignedLimb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_signed(gm),
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
                &mut (|((x, y), _, _)| no_out!(num_divisible_by_signed_limb(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_divisible_by_signed_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_signed_limb_divisible_by_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "SignedLimb.divisible_by(&Integer)",
        BenchmarkType::Single,
        pairs_of_signed_and_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [(
            "SignedLimb.divisible_by(&Integer)",
            &mut (|(x, ref y)| no_out!(x.divisible_by(y))),
        )],
    );
}
