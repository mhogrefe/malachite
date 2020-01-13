use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitScan, SignificantBits};
use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_true_bit_neg;
use malachite_nz::integer::Integer;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_vec_and_small_unsigned_var_1;
use inputs::integer::pairs_of_integer_and_small_u64;

pub fn integer_index_of_next_true_bit_alt(n: &Integer, u: u64) -> Option<u64> {
    if u >= n.significant_bits() {
        if *n >= 0 {
            None
        } else {
            Some(u)
        }
    } else {
        for (i, bit) in n
            .twos_complement_bits()
            .enumerate()
            .skip(usize::exact_from(u))
        {
            if bit {
                return Some(u64::wrapping_from(i));
            }
        }
        None
    }
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_index_of_next_true_bit_neg);
    register_demo!(registry, demo_integer_index_of_next_true_bit);
    register_bench!(registry, Small, benchmark_limbs_index_of_next_true_bit_neg);
    register_bench!(
        registry,
        Large,
        benchmark_integer_index_of_next_true_bit_algorithms
    );
}

fn demo_limbs_index_of_next_true_bit_neg(gm: GenerationMode, limit: usize) {
    for (ref limbs, u) in pairs_of_unsigned_vec_and_small_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_index_of_next_true_bit_neg({:?}, {}) = {:?}",
            limbs,
            u,
            limbs_index_of_next_true_bit_neg(limbs, u)
        );
    }
}

fn demo_integer_index_of_next_true_bit(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_u64(gm).take(limit) {
        println!(
            "index_of_next_true_bit({}, {}) = {:?}",
            n,
            u,
            n.index_of_next_true_bit(u)
        );
    }
}

fn benchmark_limbs_index_of_next_true_bit_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_index_of_next_true_bit_neg(&[u32], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(ref limbs, u)| no_out!(limbs_index_of_next_true_bit_neg(limbs, u))),
        )],
    );
}

fn benchmark_integer_index_of_next_true_bit_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.index_of_next_true_bit(u64)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_small_u64(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(ref n, u)| no_out!(n.index_of_next_true_bit(u))),
            ),
            (
                "using bits explicitly",
                &mut (|(ref n, u)| no_out!(integer_index_of_next_true_bit_alt(&n, u))),
            ),
        ],
    );
}
