use common::{gmp_natural_to_native, GenerationMode};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};

type It = Iterator<Item = (gmp::Natural, u32)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| natural_u32s_geometric(seed, scale)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_natural_divisible_by_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, pow) in select_inputs(gm).take(limit) {
        if n.divisible_by_power_of_2(pow) {
            println!("{} is divisible by 2^{}", n, pow);
        } else {
            println!("{} is not divisible by 2^{}", n, pow);
        }
    }
}

pub fn benchmark_natural_divisible_by_power_of_2(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.divisible_by_power_of_2(u32)",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(n, pow): (gmp::Natural, u32)| n.divisible_by_power_of_2(pow)),
        function_g: &(|(n, pow): (native::Natural, u32)| n.divisible_by_power_of_2(pow)),
        x_cons: &(|&(ref n, pow)| (n.clone(), pow)),
        y_cons: &(|&(ref n, pow)| (gmp_natural_to_native(n), pow)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_divisible_by_power_of_2_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.divisible_by_power_of_2(u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(n, pow): (native::Natural, u32)| n.divisible_by_power_of_2(pow)),
        function_g: &(|(n, pow): (native::Natural, u32)| {
            n.trailing_zeros().map_or(true, |z| z >= pow.into())
        }),
        x_cons: &(|&(ref n, pow)| (gmp_natural_to_native(n), pow)),
        y_cons: &(|&(ref n, pow)| (gmp_natural_to_native(n), pow)),
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
