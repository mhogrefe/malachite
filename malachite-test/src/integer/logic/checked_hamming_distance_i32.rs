use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_u32_vec_and_positive_u32_var_1;
use inputs::integer::{pairs_of_integer_and_signed, pairs_of_signed_and_integer};
use malachite_base::num::{CheckedHammingDistance, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_nz::integer::logic::checked_hamming_distance_i32::limbs_hamming_distance_limb_neg;
use std::iter::repeat;

pub fn integer_checked_hamming_distance_i32_alt(n: &Integer, i: i32) -> Option<u64> {
    let negative = i < 0;
    if (*n < 0) != negative {
        return None;
    }
    let i = Integer::from(i);
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if n.significant_bits() >= i.significant_bits() {
            Box::new(
                n.twos_complement_bits()
                    .zip(i.twos_complement_bits().chain(repeat(negative))),
            )
        } else {
            Box::new(n.twos_complement_bits().chain(repeat(negative)).zip(i.twos_complement_bits()))
        };
    let mut distance = 0u64;
    for (b, c) in bit_zip {
        if b != c {
            distance += 1;
        }
    }
    Some(distance)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_hamming_distance_limb_neg);
    register_demo!(registry, demo_integer_checked_hamming_distance_i32);
    register_demo!(registry, demo_i32_checked_hamming_distance_integer);
    register_bench!(registry, Small, benchmark_limbs_hamming_distance_limb_neg);
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_hamming_distance_i32_algorithms
    );
    register_bench!(registry, Large, benchmark_i32_checked_hamming_distance_integer);
}

fn demo_limbs_hamming_distance_limb_neg(gm: GenerationMode, limit: usize) {
    for (ref limbs, limb) in pairs_of_u32_vec_and_positive_u32_var_1(gm).take(limit) {
        println!(
            "limbs_hamming_distance_limb_neg({:?}, {}) = {}",
            limbs,
            limb,
            limbs_hamming_distance_limb_neg(limbs, limb)
        );
    }
}

fn demo_integer_checked_hamming_distance_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            n,
            i,
            n.checked_hamming_distance(i)
        );
    }
}

fn demo_i32_checked_hamming_distance_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_integer::<i32>(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            i,
            n,
            i.checked_hamming_distance(&n)
        );
    }
}

fn benchmark_limbs_hamming_distance_limb_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_hamming_distance_limb_neg(&[u32], u32)",
        BenchmarkType::Single,
        pairs_of_u32_vec_and_positive_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "malachite",
                &mut (|(ref limbs, limb)| no_out!(limbs_hamming_distance_limb_neg(limbs, limb))),
            ),
        ],
    );
}

fn benchmark_integer_checked_hamming_distance_i32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.checked_hamming_distance(i32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(ref n, other)| no_out!(n.checked_hamming_distance(other))),
            ),
            (
                "using bits explicitly",
                &mut (|(ref n, other)| {
                    no_out!(integer_checked_hamming_distance_i32_alt(&n, other))
                }),
            ),
        ],
    );
}

fn benchmark_i32_checked_hamming_distance_integer(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32.checked_hamming_distance(&Integer)",
        BenchmarkType::Single,
        pairs_of_signed_and_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "i.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(i, ref other)| no_out!(i.checked_hamming_distance(other))),
            ),
        ],
    );
}
