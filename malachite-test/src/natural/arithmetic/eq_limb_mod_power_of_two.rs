use std::cmp::min;

use malachite_base::num::arithmetic::traits::{EqModPowerOfTwo, ModPowerOfTwo};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::eq_limb_mod_power_of_two::limbs_eq_limb_mod_power_of_two;
use malachite_nz::platform::Limb;
use rug;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_unsigned_and_small_unsigned_var_1;
use inputs::natural::{
    rm_triples_of_natural_unsigned_and_small_unsigned,
    triples_of_natural_unsigned_and_small_unsigned, triples_of_unsigned_natural_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_limb_mod_power_of_two);
    register_demo!(registry, demo_natural_eq_limb_mod_power_of_two);
    register_demo!(registry, demo_limb_eq_natural_mod_power_of_two);
    register_bench!(registry, Small, benchmark_limbs_eq_limb_mod_power_of_two);
    register_bench!(
        registry,
        Large,
        benchmark_natural_eq_limb_mod_power_of_two_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_eq_limb_mod_power_of_two_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_eq_natural_mod_power_of_two_algorithms
    );
}

pub fn rug_eq_limb_mod_power_of_two(x: &rug::Integer, u: Limb, pow: u64) -> bool {
    x.is_congruent_2pow(&rug::Integer::from(u), u32::checked_from(pow).unwrap())
}

fn demo_limbs_eq_limb_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (limbs, limb, pow) in
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_1(gm).take(limit)
    {
        println!(
            "limbs_eq_limb_mod_power_of_two({:?}, {}, {}) = {:?}",
            limbs,
            limb,
            pow,
            limbs_eq_limb_mod_power_of_two(&limbs, limb, pow)
        );
    }
}

fn demo_natural_eq_limb_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u, pow) in triples_of_natural_unsigned_and_small_unsigned::<Limb, u64>(gm).take(limit) {
        println!(
            "{}.eq_mod_power_of_two({}, {}) = {}",
            n,
            u,
            pow,
            n.eq_mod_power_of_two(u, pow)
        );
    }
}

fn demo_limb_eq_natural_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (u, n, pow) in triples_of_unsigned_natural_and_small_unsigned::<Limb, u64>(gm).take(limit) {
        println!(
            "{}.eq_mod_power_of_two(&{}, {}) = {}",
            u,
            n,
            pow,
            u.eq_mod_power_of_two(&n, pow)
        );
    }
}

fn benchmark_limbs_eq_limb_mod_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_limb_mod_power_of_two(&[Limb], u64)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(ref limbs, limb, pow)| {
                no_out!(limbs_eq_limb_mod_power_of_two(limbs, limb, pow))
            }),
        )],
    );
}

fn benchmark_natural_eq_limb_mod_power_of_two_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.eq_mod_power_of_two(&Limb, u64)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_natural_unsigned_and_small_unsigned::<Limb, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (ref n, u, pow))| no_out!(n.eq_mod_power_of_two(u, pow))),
            ),
            (
                "rug",
                &mut (|((ref n, u, pow), _)| no_out!(rug_eq_limb_mod_power_of_two(n, u, pow))),
            ),
        ],
    );
}

fn benchmark_natural_eq_limb_mod_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.eq_mod_power_of_two(&Limb, u64)",
        BenchmarkType::Algorithms,
        triples_of_natural_unsigned_and_small_unsigned::<Limb, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, pow)| usize::checked_from(min(pow, n.significant_bits())).unwrap()),
        "min(pow, n.significant_bits())",
        &mut [
            (
                "Natural.eq_mod_power_of_two(Limb, u64)",
                &mut (|(n, u, pow)| no_out!(n.eq_mod_power_of_two(u, pow))),
            ),
            (
                "Natural.mod_power_of_two(u64) == Limb.mod_power_of_two(u64)",
                &mut (|(n, u, pow)| no_out!(n.mod_power_of_two(pow) == u.mod_power_of_two(pow))),
            ),
        ],
    );
}

fn benchmark_limb_eq_natural_mod_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.eq_mod_power_of_two(&Natural, u64)",
        BenchmarkType::Algorithms,
        triples_of_unsigned_natural_and_small_unsigned::<Limb, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, pow)| usize::checked_from(min(pow, n.significant_bits())).unwrap()),
        "min(pow, n.significant_bits())",
        &mut [
            (
                "Limb.eq_mod_power_of_two(&Natural, u64)",
                &mut (|(u, ref n, pow)| no_out!(u.eq_mod_power_of_two(n, pow))),
            ),
            (
                "Limb.mod_power_of_two(u64) == Natural.mod_power_of_two(u64)",
                &mut (|(u, n, pow)| no_out!(u.mod_power_of_two(pow) == n.mod_power_of_two(pow))),
            ),
        ],
    );
}
