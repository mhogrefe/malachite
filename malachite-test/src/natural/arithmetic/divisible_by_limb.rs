use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::divisible_by_limb::{
    _combined_limbs_divisible_by_limb, limbs_divisible_by_limb,
};
use malachite_nz::natural::arithmetic::mod_limb::limbs_mod_limb;
use malachite_nz::platform::Limb;
use num::{BigUint, Integer, Zero};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_vec_and_positive_unsigned_var_1;
#[cfg(feature = "64_bit_limbs")]
use inputs::natural::nm_pairs_of_natural_and_unsigned;
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::nrm_pairs_of_natural_and_unsigned;
use inputs::natural::{pairs_of_natural_and_unsigned, pairs_of_unsigned_and_natural};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_divisible_by_limb);
    register_demo!(registry, demo_natural_divisible_by_limb);
    register_demo!(registry, demo_limb_divisible_by_natural);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_divisible_by_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_divisible_by_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_limb_divisible_by_natural);
}

pub fn num_divisible_by_limb(x: BigUint, u: Limb) -> bool {
    x == BigUint::zero() || u != 0 && x.is_multiple_of(&BigUint::from(u))
}

fn demo_limbs_divisible_by_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_divisible_by_limb({:?}, {}) = {}",
            limbs,
            limb,
            limbs_divisible_by_limb(&limbs, limb)
        );
    }
}

fn demo_natural_divisible_by_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        if n.divisible_by(u) {
            println!("{} is divisible by {}", n, u);
        } else {
            println!("{} is not divisible by {}", n, u);
        }
    }
}

fn demo_limb_divisible_by_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        if u.divisible_by(&n) {
            println!("{} is divisible by {}", u, n);
        } else {
            println!("{} is not divisible by {}", u, n);
        }
    }
}

fn benchmark_limbs_divisible_by_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    let new_gm = match gm {
        GenerationMode::Random(scale) => GenerationMode::Random(scale * 10),
        GenerationMode::SpecialRandom(scale) => GenerationMode::SpecialRandom(scale * 10),
        gm => gm,
    };
    m_run_benchmark(
        "limbs_divisible_by_limb(&mut [Limb], Limb)",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(new_gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "limbs_divisible_by_limb",
                &mut (|(ref limbs, limb)| no_out!(limbs_divisible_by_limb(limbs, limb))),
            ),
            (
                "limbs_mod_limb",
                &mut (|(ref limbs, limb)| no_out!(limbs_mod_limb(limbs, limb) == 0)),
            ),
            (
                "_combined_limbs_divisible_by_limb",
                &mut (|(ref limbs, limb)| no_out!(_combined_limbs_divisible_by_limb(limbs, limb))),
            ),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_divisible_by_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.divisible_by(Limb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned(gm),
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

#[cfg(feature = "64_bit_limbs")]
fn benchmark_natural_divisible_by_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.divisible_by(Limb)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_natural_and_unsigned(gm),
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

fn benchmark_limb_divisible_by_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Limb.divisible_by(&Natural)",
        BenchmarkType::Single,
        pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [(
            "Limb.divisible_by(&Natural)",
            &mut (|(x, ref y)| no_out!(x.divisible_by(y))),
        )],
    );
}
