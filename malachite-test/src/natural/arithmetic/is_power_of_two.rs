use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::naturals;
use malachite_base::num::{IsPowerOfTwo, SignificantBits};

pub fn demo_natural_is_power_of_two(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        if n.is_power_of_two() {
            println!("{} is a power of two", n);
        } else {
            println!("{} is not a power of two", n);
        }
    }
}

pub fn benchmark_natural_is_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.is_power_of_two()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.is_power_of_two())))],
    );
}
