use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::is_power_of_2::limbs_is_power_of_2;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::vecs_of_unsigned_var_1;
use malachite_test::inputs::natural::{naturals, rm_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_is_power_of_2);
    register_demo!(registry, demo_natural_is_power_of_2);
    register_bench!(registry, Small, benchmark_limbs_is_power_of_2);
    register_bench!(
        registry,
        Large,
        benchmark_natural_is_power_of_2_library_comparison
    );
}

fn demo_limbs_is_power_of_2(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_is_power_of_2({:?}) = {:?}",
            limbs,
            limbs_is_power_of_2(&limbs)
        );
    }
}

fn demo_natural_is_power_of_2(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        if n.is_power_of_2() {
            println!("{} is a power of 2", n);
        } else {
            println!("{} is not a power of 2", n);
        }
    }
}

fn benchmark_limbs_is_power_of_2(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_is_power_of_2(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|ref limbs| no_out!(limbs_is_power_of_2(limbs))),
        )],
    );
}

fn benchmark_natural_is_power_of_2_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.is_power_of_2()",
        BenchmarkType::LibraryComparison,
        rm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, n)| no_out!(n.is_power_of_2()))),
            ("rug", &mut (|(n, _)| no_out!(n.is_power_of_two()))),
        ],
    );
}
