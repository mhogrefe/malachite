use common::GenerationMode;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};

type It = Iterator<Item = (Integer, u32)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(log_pairs(exhaustive_integers(), exhaustive_u::<u32>()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, scale)),
        &(|seed| natural_u32s_geometric(seed, scale)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_divisible_by_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, pow) in select_inputs(gm).take(limit) {
        if n.divisible_by_power_of_2(pow) {
            println!("{} is divisible by 2^{}", n, pow);
        } else {
            println!("{} is not divisible by 2^{}", n, pow);
        }
    }
}

pub fn benchmark_integer_divisible_by_power_of_2(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.divisible_by_power_of_2(u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(n, pow): (Integer, u32)| n.divisible_by_power_of_2(pow)),
        x_cons: &(|&(ref n, pow)| (n.clone(), pow)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_divisible_by_power_of_2_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.divisible_by_power_of_2(u32)",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(n, pow): (Integer, u32)| n.divisible_by_power_of_2(pow)),
        function_g: &(|(n, pow): (Integer, u32)| {
            n.trailing_zeros().map_or(true, |z| z >= pow.into())
        }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "Integer.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u32)",
        g_name: "Integer.trailing\\\\_zeros().map\\\\_or(true, |z| z >= u32)",
        title: "Integer.divisible\\\\_by\\\\_power\\\\_of\\\\_2(u32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
