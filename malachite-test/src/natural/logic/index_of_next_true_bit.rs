use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_vec_and_small_unsigned;
use inputs::natural::pairs_of_natural_and_small_unsigned;
use malachite_base::num::traits::{BitScan, SignificantBits};
use malachite_nz::natural::logic::bit_scan::limbs_index_of_next_true_bit;
use malachite_nz::natural::Natural;

pub fn natural_index_of_next_true_bit_alt(n: &Natural, u: u64) -> Option<u64> {
    for (i, bit) in n.bits().enumerate().skip(u as usize) {
        if bit {
            return Some(i as u64);
        }
    }
    None
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_index_of_next_true_bit);
    register_demo!(registry, demo_natural_index_of_next_true_bit);
    register_bench!(registry, Small, benchmark_limbs_index_of_next_true_bit);
    register_bench!(
        registry,
        Large,
        benchmark_natural_index_of_next_true_bit_algorithms
    );
}

fn demo_limbs_index_of_next_true_bit(gm: GenerationMode, limit: usize) {
    for (ref limbs, u) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_index_of_next_true_bit({:?}, {}) = {:?}",
            limbs,
            u,
            limbs_index_of_next_true_bit(limbs, u)
        );
    }
}

fn demo_natural_index_of_next_true_bit(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        println!(
            "index_of_next_true_bit({}, {}) = {:?}",
            n,
            u,
            n.index_of_next_true_bit(u)
        );
    }
}

fn benchmark_limbs_index_of_next_true_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_index_of_next_true_bit(&[u32], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(ref limbs, u)| no_out!(limbs_index_of_next_true_bit(limbs, u))),
        )],
    );
}

fn benchmark_natural_index_of_next_true_bit_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.index_of_next_true_bit(u64)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(ref n, u)| no_out!(n.index_of_next_true_bit(u))),
            ),
            (
                "using bits explicitly",
                &mut (|(ref n, u)| no_out!(natural_index_of_next_true_bit_alt(&n, u))),
            ),
        ],
    );
}
