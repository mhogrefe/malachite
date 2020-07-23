use std::cmp::max;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{CheckedHammingDistance, SignificantBits};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::integer::logic::checked_hamming_distance::{
    limbs_hamming_distance_limb_neg, limbs_hamming_distance_neg,
};
use malachite_nz_test_util::integer::logic::checked_hamming_distance::{
    integer_checked_hamming_distance_alt_1, integer_checked_hamming_distance_alt_2,
    rug_checked_hamming_distance,
};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_positive_unsigned_var_2, pairs_of_unsigned_vec_var_6,
};
use malachite_test::inputs::integer::{pairs_of_integers, rm_pairs_of_integers};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_hamming_distance_limb_neg);
    register_demo!(registry, demo_limbs_hamming_distance_neg);
    register_demo!(registry, demo_integer_checked_hamming_distance);
    register_bench!(registry, Small, benchmark_limbs_hamming_distance_limb_neg);
    register_bench!(registry, Small, benchmark_limbs_hamming_distance_neg);
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_hamming_distance_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_checked_hamming_distance_algorithms
    );
}

fn demo_limbs_hamming_distance_limb_neg(gm: GenerationMode, limit: usize) {
    for (ref limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_2(gm).take(limit) {
        println!(
            "limbs_hamming_distance_limb_neg({:?}, {}) = {}",
            limbs,
            limb,
            limbs_hamming_distance_limb_neg(limbs, limb)
        );
    }
}

fn demo_limbs_hamming_distance_neg(gm: GenerationMode, limit: usize) {
    for (ref xs, ref ys) in pairs_of_unsigned_vec_var_6(gm).take(limit) {
        println!(
            "limbs_hamming_distance_neg({:?}, {:?}) = {}",
            xs,
            ys,
            limbs_hamming_distance_neg(xs, ys)
        );
    }
}

fn demo_integer_checked_hamming_distance(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        println!(
            "checked_hamming_distance({}, {}) = {:?}",
            x,
            y,
            x.checked_hamming_distance(&y)
        );
    }
}

fn benchmark_limbs_hamming_distance_limb_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_hamming_distance_limb_neg(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_positive_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(ref limbs, limb)| no_out!(limbs_hamming_distance_limb_neg(limbs, limb))),
        )],
    );
}

fn benchmark_limbs_hamming_distance_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_hamming_distance_neg(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(ref xs, ref ys)| no_out!(limbs_hamming_distance_neg(xs, ys))),
        )],
    );
}

fn benchmark_integer_checked_hamming_distance_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.checked_hamming_distance(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "malachite",
                &mut (|(_, (x, y))| no_out!(x.checked_hamming_distance(&y))),
            ),
            (
                "rug",
                &mut (|((x, y), _)| no_out!(rug_checked_hamming_distance(&x, &y))),
            ),
        ],
    );
}

fn benchmark_integer_checked_hamming_distance_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.checked_hamming_distance(&Integer)",
        BenchmarkType::Algorithms,
        pairs_of_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "default",
                &mut (|(n, other)| no_out!(n.checked_hamming_distance(&other))),
            ),
            (
                "using bits explicitly",
                &mut (|(n, other)| no_out!(integer_checked_hamming_distance_alt_1(&n, &other))),
            ),
            (
                "using limbs explicitly",
                &mut (|(n, other)| no_out!(integer_checked_hamming_distance_alt_2(&n, &other))),
            ),
        ],
    );
}
