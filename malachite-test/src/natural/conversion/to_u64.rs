use common::GenerationMode;
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_natural_to_u64(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_u64({}) = {:?}", n, n.to_u64());
    }
}

pub fn demo_natural_to_u64_wrapping(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_u64({}) = {:?}", n, n.to_u64_wrapping());
    }
}

pub fn benchmark_natural_to_u64(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.to_u64()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| n.to_u64()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.to_u64()",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_to_u64_wrapping(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.to_u64_wrapping()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| n.to_u64_wrapping()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.to_u64_wrapping()",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
