use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    rm_triples_of_integer_signed_and_small_unsigned, triples_of_integer_signed_and_small_unsigned,
};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{EqModPowerOfTwo, ModPowerOfTwo, SignificantBits};
use malachite_nz::integer::Integer;
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_eq_mod_power_of_two_i32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_mod_power_of_two_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_mod_power_of_two_i32_algorithms
    );
}

pub fn rug_eq_mod_power_of_two_i32(x: &rug::Integer, i: &i32, pow: u64) -> bool {
    x.is_congruent_2pow(&rug::Integer::from(*i), u32::checked_from(pow).unwrap())
}

fn demo_integer_eq_mod_power_of_two_i32(gm: GenerationMode, limit: usize) {
    for (n, i, pow) in triples_of_integer_signed_and_small_unsigned::<i32, u64>(gm).take(limit) {
        println!(
            "{}.eq_mod_power_of_two({}, {}) = {}",
            n,
            i,
            pow,
            n.eq_mod_power_of_two(&i, pow)
        );
    }
}

fn benchmark_integer_eq_mod_power_of_two_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod_power_of_two(&i32, u64)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_integer_signed_and_small_unsigned::<i32, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (ref n, ref i, pow))| no_out!(n.eq_mod_power_of_two(i, pow))),
            ),
            (
                "rug",
                &mut (|((ref n, ref i, pow), _)| no_out!(rug_eq_mod_power_of_two_i32(n, i, pow))),
            ),
        ],
    );
}

fn benchmark_integer_eq_mod_power_of_two_i32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod_power_of_two(&i32, u64)",
        BenchmarkType::Algorithms,
        triples_of_integer_signed_and_small_unsigned::<i32, u64>(gm),
        gm.name(),
        limit,
        file_name,
        &(|(ref n, _, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.eq_mod_power_of_two(&i32, u64)",
                &mut (|(n, i, pow)| no_out!(n.eq_mod_power_of_two(&i, pow))),
            ),
            (
                "Integer.mod_power_of_two(u64) == i32.mod_power_of_two(u64)",
                &mut (|(n, i, pow)| {
                    no_out!(n.mod_power_of_two(pow) == Integer::from(i).mod_power_of_two(pow))
                }),
            ),
        ],
    );
}
