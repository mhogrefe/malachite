use std::cmp::min;

use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::divisible_by_power_of_2::limbs_divisible_by_power_of_2;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigned_vec_and_small_unsigned_var_1;
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_unsigned, rm_pairs_of_natural_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_divisible_by_power_of_2);
    register_demo!(registry, demo_natural_divisible_by_power_of_2);
    register_bench!(registry, Small, benchmark_limbs_divisible_by_power_of_2);
    register_bench!(
        registry,
        Large,
        benchmark_natural_divisible_by_power_of_2_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_divisible_by_power_of_2_algorithms
    );
}

fn demo_limbs_divisible_by_power_of_2(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_divisible_by_power_of_2({:?}, {}) = {:?}",
            limbs,
            pow,
            limbs_divisible_by_power_of_2(&limbs, pow)
        );
    }
}

fn demo_natural_divisible_by_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        if n.divisible_by_power_of_2(pow) {
            println!("{} is divisible by 2^{}", n, pow);
        } else {
            println!("{} is not divisible by 2^{}", n, pow);
        }
    }
}

fn benchmark_limbs_divisible_by_power_of_2(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_shr_exact(&[u32], u32)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, pow)| min(usize::exact_from(pow), limbs.len())),
        "min(pow, limbs.len())",
        &mut [(
            "Malachite",
            &mut (|(limbs, pow)| no_out!(limbs_divisible_by_power_of_2(&limbs, pow))),
        )],
    );
}

fn benchmark_natural_divisible_by_power_of_2_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.divisible_by_power_of_2(u64)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Malachite",
                &mut (|(_, (n, pow))| no_out!(n.divisible_by_power_of_2(pow))),
            ),
            (
                "rug",
                &mut (|((n, pow), _)| {
                    n.is_divisible_2pow(u32::exact_from(pow));
                }),
            ),
        ],
    );
}

fn benchmark_natural_divisible_by_power_of_2_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.divisible_by_power_of_2(u64)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, pow)| usize::exact_from(min(pow, n.significant_bits()))),
        "min(pow, n.significant_bits())",
        &mut [
            (
                "Natural.divisible_by_power_of_2(u64)",
                &mut (|(n, pow)| no_out!(n.divisible_by_power_of_2(pow))),
            ),
            (
                "Natural.trailing_zeros().map_or(true, |z| z >= u64)",
                &mut (|(n, pow)| no_out!(n.trailing_zeros().map_or(true, |z| z >= pow))),
            ),
        ],
    );
}
