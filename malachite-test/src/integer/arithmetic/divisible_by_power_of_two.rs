use malachite_base::num::arithmetic::traits::DivisibleByPowerOfTwo;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_small_unsigned, rm_pairs_of_integer_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_divisible_by_power_of_two);
    register_bench!(
        registry,
        Large,
        benchmark_integer_divisible_by_power_of_two_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_divisible_by_power_of_two_algorithms
    );
}

fn demo_integer_divisible_by_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        if n.divisible_by_power_of_two(pow) {
            println!("{} is divisible by 2^{}", n, pow);
        } else {
            println!("{} is not divisible by 2^{}", n, pow);
        }
    }
}

fn benchmark_integer_divisible_by_power_of_two_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.divisible_by_power_of_two(u64)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (n, pow))| no_out!(n.divisible_by_power_of_two(pow))),
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

fn benchmark_integer_divisible_by_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.divisible_by_power_of_two(u64)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.divisible_by_power_of_2(u64)",
                &mut (|(n, pow)| no_out!(n.divisible_by_power_of_two(pow))),
            ),
            (
                "Integer.trailing_zeros().map_or(true, |z| z >= u64)",
                &mut (|(n, pow)| no_out!(n.trailing_zeros().map_or(true, |z| z >= pow))),
            ),
        ],
    );
}
