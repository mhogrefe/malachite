use malachite_base::num::traits::{EqModPowerOfTwo, ModPowerOfTwo, SignificantBits};
use malachite_nz::integer::arithmetic::eq_limb_mod_power_of_two::limbs_eq_mod_power_of_two_neg_limb;
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2;
use inputs::integer::{
    rm_triples_of_integer_unsigned_and_small_unsigned,
    triples_of_integer_unsigned_and_small_unsigned, triples_of_unsigned_integer_and_small_unsigned,
};
use natural::arithmetic::eq_limb_mod_power_of_two::rug_eq_limb_mod_power_of_two;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_mod_power_of_two_neg_limb);
    register_demo!(registry, demo_integer_eq_limb_mod_power_of_two);
    register_demo!(registry, demo_limb_eq_integer_mod_power_of_two);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_eq_mod_power_of_two_neg_limb
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_limb_mod_power_of_two_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_limb_mod_power_of_two_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_eq_integer_mod_power_of_two_algorithms
    );
}

fn demo_limbs_eq_mod_power_of_two_neg_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb, pow) in
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2(gm).take(limit)
    {
        println!(
            "limbs_eq_mod_power_of_two_neg_limb({:?}, {}, {}) = {:?}",
            limbs,
            limb,
            pow,
            limbs_eq_mod_power_of_two_neg_limb(&limbs, limb, pow)
        );
    }
}

fn demo_integer_eq_limb_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u, pow) in triples_of_integer_unsigned_and_small_unsigned::<Limb, u64>(gm).take(limit) {
        println!(
            "{}.eq_mod_power_of_two({}, {}) = {}",
            n,
            u,
            pow,
            n.eq_mod_power_of_two(u, pow)
        );
    }
}

fn demo_limb_eq_integer_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (u, n, pow) in triples_of_unsigned_integer_and_small_unsigned::<Limb, u64>(gm).take(limit) {
        println!(
            "{}.eq_mod_power_of_two({}, {}) = {}",
            u,
            n,
            pow,
            u.eq_mod_power_of_two(&n, pow)
        );
    }
}

fn benchmark_limbs_eq_mod_power_of_two_neg_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_mod_power_of_two_neg_limb(&[Limb], u64)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_and_small_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(ref limbs, limb, pow)| {
                no_out!(limbs_eq_mod_power_of_two_neg_limb(limbs, limb, pow))
            }),
        )],
    );
}

fn benchmark_integer_eq_limb_mod_power_of_two_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod_power_of_two(&Limb, u64)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_integer_unsigned_and_small_unsigned::<Limb, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _, _))| n.significant_bits() as usize),
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

fn benchmark_integer_eq_limb_mod_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod_power_of_two(&Limb, u64)",
        BenchmarkType::Algorithms,
        triples_of_integer_unsigned_and_small_unsigned::<Limb, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(ref n, _, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.eq_mod_power_of_two(&Limb, u64)",
                &mut (|(n, u, pow)| no_out!(n.eq_mod_power_of_two(u, pow))),
            ),
            (
                "Integer.mod_power_of_two(u64) == Limb.mod_power_of_two(u64)",
                &mut (|(n, u, pow)| no_out!(n.mod_power_of_two(pow) == u.mod_power_of_two(pow))),
            ),
        ],
    );
}

fn benchmark_limb_eq_integer_mod_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.eq_mod_power_of_two(&Integer, u64)",
        BenchmarkType::Algorithms,
        triples_of_unsigned_integer_and_small_unsigned::<Limb, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.eq_mod_power_of_two(&Integer, u64)",
                &mut (|(u, ref n, pow)| no_out!(n.eq_mod_power_of_two(u, pow))),
            ),
            (
                "Limb.mod_power_of_two(u64) == Integer.mod_power_of_two(u64)",
                &mut (|(u, ref n, pow)| {
                    no_out!(u.mod_power_of_two(pow) == n.mod_power_of_two(pow))
                }),
            ),
        ],
    );
}
