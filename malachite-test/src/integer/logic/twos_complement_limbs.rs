use common::{gmp_integer_to_native, GenerationMode};
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};

type It = Iterator<Item = gmp::Integer>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_integers())
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_integers(&EXAMPLE_SEED, scale))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_twos_complement_limbs_le(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!(
            "twos_complement_limbs_le({}) = {:?}",
            n,
            n.twos_complement_limbs_le()
        );
    }
}

pub fn demo_integer_twos_complement_limbs_be(gm: GenerationMode, limit: usize) {
    for n in select_inputs(gm).take(limit) {
        println!(
            "twos_complement_limbs_be({}) = {:?}",
            n,
            n.twos_complement_limbs_be()
        );
    }
}

pub fn benchmark_integer_twos_complement_limbs_le(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.twos_complement_limbs_le()",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: gmp::Integer| n.twos_complement_limbs_le()),
        function_g: &(|n: native::Integer| n.twos_complement_limbs_le()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.twos\\\\_complement\\\\_limbs\\\\_le()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_twos_complement_limbs_be(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.twos_complement_limbs_be()",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|n: gmp::Integer| n.twos_complement_limbs_be()),
        function_g: &(|n: native::Integer| n.twos_complement_limbs_be()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_integer_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.twos\\\\_complement\\\\_limbs\\\\_be()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
