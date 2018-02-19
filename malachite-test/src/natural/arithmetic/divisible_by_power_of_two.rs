use common::GenerationMode;
use inputs::natural::pairs_of_natural_and_small_u32;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};

pub fn demo_natural_divisible_by_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_natural_and_small_u32(gm).take(limit) {
        if n.divisible_by_power_of_two(pow) {
            println!("{} is divisible by 2^{}", n, pow);
        } else {
            println!("{} is not divisible by 2^{}", n, pow);
        }
    }
}

pub fn benchmark_natural_divisible_by_power_of_two(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.divisible_by_power_of_two(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_natural_and_small_u32(gm),
        function_f: &mut (|(n, pow): (Natural, u32)| n.divisible_by_power_of_two(pow)),
        x_cons: &(|&(ref n, pow)| (n.clone(), pow)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_divisible_by_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.divisible_by_power_of_two(u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_natural_and_small_u32(gm),
        function_f: &mut (|(n, pow): (Natural, u32)| n.divisible_by_power_of_two(pow)),
        function_g: &mut (|(n, pow): (Natural, u32)| {
            n.trailing_zeros().map_or(true, |z| z >= u64::from(pow))
        }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "Natural.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u32)",
        g_name: "Natural.trailing\\\\_zeros().map\\\\_or(true, |z| z >= u32)",
        title: "Natural.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
