use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    rm_triples_of_integer_signed_and_small_unsigned, triples_of_integer_signed_and_small_unsigned,
    triples_of_signed_integer_and_small_unsigned,
};
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{EqModPowerOfTwo, ModPowerOfTwo, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_eq_signed_limb_mod_power_of_two);
    register_demo!(registry, demo_signed_limb_eq_integer_mod_power_of_two);
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_signed_limb_mod_power_of_two_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_signed_limb_mod_power_of_two_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_eq_integer_mod_power_of_two
    );
}

pub fn rug_eq_signed_limb_mod_power_of_two(x: &rug::Integer, i: SignedLimb, pow: u64) -> bool {
    x.is_congruent_2pow(&rug::Integer::from(i), u32::checked_from(pow).unwrap())
}

fn demo_integer_eq_signed_limb_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, i, pow) in
        triples_of_integer_signed_and_small_unsigned::<SignedLimb, u64>(gm).take(limit)
    {
        println!(
            "{}.eq_mod_power_of_two({}, {}) = {}",
            n,
            i,
            pow,
            n.eq_mod_power_of_two(i, pow)
        );
    }
}

fn demo_signed_limb_eq_integer_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (i, n, pow) in
        triples_of_signed_integer_and_small_unsigned::<SignedLimb, u64>(gm).take(limit)
    {
        println!(
            "{}.eq_mod_power_of_two({}, {}) = {}",
            i,
            n,
            pow,
            i.eq_mod_power_of_two(&n, pow)
        );
    }
}

fn benchmark_integer_eq_signed_limb_mod_power_of_two_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod_power_of_two(&SignedLimb, u64)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_integer_signed_and_small_unsigned::<SignedLimb, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (ref n, i, pow))| no_out!(n.eq_mod_power_of_two(i, pow))),
            ),
            (
                "rug",
                &mut (|((ref n, i, pow), _)| {
                    no_out!(rug_eq_signed_limb_mod_power_of_two(n, i, pow))
                }),
            ),
        ],
    );
}

fn benchmark_integer_eq_signed_limb_mod_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod_power_of_two(&SignedLimb, u64)",
        BenchmarkType::Algorithms,
        triples_of_integer_signed_and_small_unsigned::<SignedLimb, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(ref n, _, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Integer.eq_mod_power_of_two(&SignedLimb, u64)",
                &mut (|(n, i, pow)| no_out!(n.eq_mod_power_of_two(i, pow))),
            ),
            (
                "Integer.mod_power_of_two(u64) == SignedLimb.mod_power_of_two(u64)",
                &mut (|(n, i, pow)| {
                    no_out!(n.mod_power_of_two(pow) == Integer::from(i).mod_power_of_two(pow))
                }),
            ),
        ],
    );
}

fn benchmark_signed_limb_eq_integer_mod_power_of_two(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.eq_mod_power_of_two(&Integer, u64)",
        BenchmarkType::Single,
        triples_of_signed_integer_and_small_unsigned::<SignedLimb, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [(
            "SignedLimb.eq_mod_power_of_two(&Integer, u64)",
            &mut (|(i, ref n, pow)| no_out!(n.eq_mod_power_of_two(i, pow))),
        )],
    );
}
