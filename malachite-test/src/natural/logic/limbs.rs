use common::GenerationMode;
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_natural_to_limbs_le(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_limbs_le({}) = {:?}", n, n.to_limbs_le());
    }
}

pub fn demo_natural_to_limbs_be(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_limbs_be({}) = {:?}", n, n.to_limbs_be());
    }
}

pub fn benchmark_natural_to_limbs_le(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.to_limbs_le()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| n.to_limbs_le()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.to\\\\_limbs\\\\_le()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_to_limbs_be(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.to_limbs_be()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| n.to_limbs_be()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.to\\\\_limbs\\\\_be()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
