use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{CountOnes, SignificantBits};
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::logic::count_ones::limbs_count_ones;
use malachite_nz_test_util::natural::logic::count_ones::{
    natural_count_ones_alt_1, natural_count_ones_alt_2,
};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::vecs_of_unsigned;
use malachite_test::inputs::natural::naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_count_ones);
    register_demo!(registry, demo_natural_count_ones);
    register_bench!(registry, Small, benchmark_limbs_count_ones);
    register_bench!(registry, Large, benchmark_natural_count_ones_algorithms);
}

fn demo_limbs_count_ones(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "limbs_count_ones({:?}) = {}",
            limbs,
            limbs_count_ones(&limbs)
        );
    }
}

fn demo_natural_count_ones(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("count_ones({}) = {}", n, n.count_ones());
    }
}

fn benchmark_limbs_count_ones(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_count_ones(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|limbs| no_out!(limbs_count_ones(&limbs))),
        )],
    );
}

fn benchmark_natural_count_ones_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.count_ones()",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.count_ones()))),
            (
                "using bits explicitly",
                &mut (|n| no_out!(natural_count_ones_alt_1(&n))),
            ),
            (
                "using limbs explicitly",
                &mut (|n| no_out!(natural_count_ones_alt_2(&n))),
            ),
        ],
    );
}
