use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::logic::trailing_zeros::limbs_trailing_zeros;
use malachite_nz_test_util::natural::logic::trailing_zeros::natural_trailing_zeros_alt;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::vecs_of_unsigned_var_3;
use malachite_test::inputs::natural::naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_trailing_zeros);
    register_demo!(registry, demo_natural_trailing_zeros);
    register_bench!(registry, Small, benchmark_limbs_trailing_zeros);
    register_bench!(registry, Large, benchmark_natural_trailing_zeros_algorithms);
}

fn demo_limbs_trailing_zeros(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_3(gm).take(limit) {
        println!(
            "limbs_trailing_zeros({:?}) = {}",
            limbs,
            limbs_trailing_zeros(&limbs)
        );
    }
}

fn demo_natural_trailing_zeros(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("trailing_zeros({}) = {:?}", n, n.trailing_zeros());
    }
}

fn benchmark_limbs_trailing_zeros(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_trailing_zeros(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|limbs| no_out!(limbs_trailing_zeros(&limbs))),
        )],
    );
}

fn benchmark_natural_trailing_zeros_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.trailing_zeros()",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.trailing_zeros()))),
            (
                "using bits explicitly",
                &mut (|n| no_out!(natural_trailing_zeros_alt(&n))),
            ),
        ],
    );
}
