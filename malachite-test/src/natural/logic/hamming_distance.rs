use std::cmp::max;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{HammingDistance, SignificantBits};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::logic::hamming_distance::{
    limbs_hamming_distance, limbs_hamming_distance_limb, limbs_hamming_distance_same_length,
};
use malachite_nz_test_util::natural::logic::hamming_distance::{
    natural_hamming_distance_alt_1, natural_hamming_distance_alt_2, rug_hamming_distance,
};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_unsigned_vec_var_1,
    pairs_of_unsigned_vec_var_2,
};
use malachite_test::inputs::natural::{pairs_of_naturals, rm_pairs_of_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_hamming_distance_limb);
    register_demo!(registry, demo_limbs_hamming_distance_same_length);
    register_demo!(registry, demo_limbs_hamming_distance);
    register_demo!(registry, demo_natural_hamming_distance);
    register_bench!(registry, Small, benchmark_limbs_hamming_distance_limb);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_hamming_distance_same_length
    );
    register_bench!(registry, Small, benchmark_limbs_hamming_distance);
    register_bench!(
        registry,
        Large,
        benchmark_natural_hamming_distance_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_hamming_distance_algorithms
    );
}

fn demo_limbs_hamming_distance_limb(gm: GenerationMode, limit: usize) {
    for (ref limbs, limb) in pairs_of_nonempty_unsigned_vec_and_unsigned(gm).take(limit) {
        println!(
            "limbs_hamming_distance_limb({:?}, {}) = {}",
            limbs,
            limb,
            limbs_hamming_distance_limb(limbs, limb)
        );
    }
}

fn demo_limbs_hamming_distance_same_length(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_1(gm).take(limit) {
        println!(
            "limbs_hamming_distance_same_length({:?}, {:?}) = {}",
            xs,
            ys,
            limbs_hamming_distance_same_length(&xs, &ys),
        );
    }
}

fn demo_limbs_hamming_distance(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_2(gm).take(limit) {
        println!(
            "limbs_hamming_distance({:?}, {:?}) = {}",
            xs,
            ys,
            limbs_hamming_distance(&xs, &ys)
        );
    }
}

fn demo_natural_hamming_distance(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!(
            "hamming_distance({}, {}) = {}",
            x,
            y,
            x.hamming_distance(&y)
        );
    }
}

fn benchmark_limbs_hamming_distance_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_hamming_distance_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_nonempty_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|(ref limbs, limb)| no_out!(limbs_hamming_distance_limb(limbs, limb))),
        )],
    );
}

fn benchmark_limbs_hamming_distance_same_length(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_hamming_distance_same_length(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "Malachite",
            &mut (|(xs, ys)| no_out!(limbs_hamming_distance_same_length(&xs, &ys))),
        )],
    );
}

fn benchmark_limbs_hamming_distance(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_hamming_distance(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [(
            "Malachite",
            &mut (|(xs, ys)| no_out!(limbs_hamming_distance(&xs, &ys))),
        )],
    );
}

fn benchmark_natural_hamming_distance_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.hamming_distance(&Natural)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "Malachite",
                &mut (|(_, (x, y))| no_out!(x.hamming_distance(&y))),
            ),
            (
                "rug",
                &mut (|((x, y), _)| no_out!(rug_hamming_distance(&x, &y))),
            ),
        ],
    );
}

fn benchmark_natural_hamming_distance_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.hamming_distance(&Natural)",
        BenchmarkType::Algorithms,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| usize::exact_from(max(x.significant_bits(), y.significant_bits()))),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("default", &mut (|(x, y)| no_out!(x.hamming_distance(&y)))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(natural_hamming_distance_alt_1(&x, &y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(x, y)| no_out!(natural_hamming_distance_alt_2(&x, &y))),
            ),
        ],
    );
}
