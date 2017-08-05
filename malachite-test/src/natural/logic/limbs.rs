use common::gmp_natural_to_native;
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};

pub fn demo_exhaustive_natural_limbs_le(limit: usize) {
    for n in exhaustive_naturals().take(limit) {
        println!("limbs_le({}) = {:?}", n, n.limbs_le());
    }
}

pub fn demo_random_natural_limbs_le(limit: usize) {
    for n in random_naturals(&EXAMPLE_SEED, 32).take(limit) {
        println!("limbs_le({}) = {:?}", n, n.limbs_le());
    }
}

pub fn demo_exhaustive_natural_limbs_be(limit: usize) {
    for n in exhaustive_naturals().take(limit) {
        println!("limbs_be({}) = {:?}", n, n.limbs_be());
    }
}

pub fn demo_random_natural_limbs_be(limit: usize) {
    for n in random_naturals(&EXAMPLE_SEED, 32).take(limit) {
        println!("limbs_be({}) = {:?}", n, n.limbs_be());
    }
}

pub fn benchmark_exhaustive_natural_limbs_le(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.limbs_le()");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_naturals(),
        function_f: &(|n: gmp::Natural| n.limbs_le()),
        function_g: &(|n: native::Natural| n.limbs_le()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_natural_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.limbs\\\\_le()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_limbs_le(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.limbs_le()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_naturals(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Natural| n.limbs_le()),
        function_g: &(|n: native::Natural| n.limbs_le()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_natural_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.limbs\\\\_le()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_limbs_be(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.limbs_be()");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_naturals(),
        function_f: &(|n: gmp::Natural| n.limbs_be()),
        function_g: &(|n: native::Natural| n.limbs_be()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_natural_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.limbs\\\\_be()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_limbs_be(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.limbs_be()");
    benchmark_2(BenchmarkOptions2 {
        xs: random_naturals(&EXAMPLE_SEED, scale),
        function_f: &(|n: gmp::Natural| n.limbs_be()),
        function_g: &(|n: native::Natural| n.limbs_be()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| gmp_natural_to_native(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.limbs\\\\_be()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
