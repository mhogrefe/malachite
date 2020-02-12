use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{BitIterable, SignificantBits};
use malachite_nz::integer::logic::checked_count_zeros::limbs_count_zeros_neg;
use malachite_nz::integer::Integer;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::nonempty_vecs_of_unsigned;
use inputs::integer::integers;

pub fn integer_checked_count_zeros_alt_1(n: &Integer) -> Option<u64> {
    if *n < 0 {
        Some(u64::wrapping_from(n.bits().filter(|&b| !b).count()))
    } else {
        None
    }
}

pub fn integer_checked_count_zeros_alt_2(n: &Integer) -> Option<u64> {
    if *n < 0 {
        Some(
            n.twos_complement_limbs()
                .map(|limb| u64::from(limb.count_zeros()))
                .sum(),
        )
    } else {
        None
    }
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_count_zeros_neg);
    register_demo!(registry, demo_integer_checked_count_zeros);
    register_bench!(registry, Small, benchmark_limbs_count_zeros_neg);
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_count_zeros_algorithms
    );
}

fn demo_limbs_count_zeros_neg(gm: GenerationMode, limit: usize) {
    for limbs in nonempty_vecs_of_unsigned(gm).take(limit) {
        println!(
            "limbs_count_zeros_neg({:?}) = {}",
            limbs,
            limbs_count_zeros_neg(&limbs)
        );
    }
}

fn demo_integer_checked_count_zeros(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("checked_count_zeros({}) = {:?}", n, n.checked_count_zeros());
    }
}

fn benchmark_limbs_count_zeros_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_count_zeros_neg(&[u32])",
        BenchmarkType::Single,
        nonempty_vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|limbs| no_out!(limbs_count_zeros_neg(&limbs))),
        )],
    );
}

fn benchmark_integer_checked_count_zeros_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.checked_count_zeros()",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.checked_count_zeros()))),
            (
                "using bits explicitly",
                &mut (|n| no_out!(integer_checked_count_zeros_alt_1(&n))),
            ),
            (
                "using limbs explicitly",
                &mut (|n| no_out!(integer_checked_count_zeros_alt_2(&n))),
            ),
        ],
    );
}
