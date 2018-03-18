use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::pairs_of_integer_and_small_u32;
use malachite_base::num::SignificantBits;

pub fn demo_integer_divisible_by_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_integer_and_small_u32(gm).take(limit) {
        if n.divisible_by_power_of_two(pow) {
            println!("{} is divisible by 2^{}", n, pow);
        } else {
            println!("{} is not divisible by 2^{}", n, pow);
        }
    }
}

pub fn benchmark_integer_divisible_by_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.divisible_by_power_of_two(u32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.divisible_by_power_of_2(u32)",
                &mut (|(n, pow)| no_out!(n.divisible_by_power_of_two(pow))),
            ),
            (
                "Integer.trailing_zeros().map_or(true, |z| z >= u32)",
                &mut (|(n, pow)| no_out!(n.trailing_zeros().map_or(true, |z| z >= u64::from(pow)))),
            ),
        ],
    );
}
