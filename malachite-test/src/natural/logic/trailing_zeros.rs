use common::GenerationMode;
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_natural_trailing_zeros(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("trailing_zeros({}) = {:?}", n, n.trailing_zeros());
    }
}

pub fn benchmark_natural_trailing_zeros(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.trailing_zeros()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| n.trailing_zeros()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.trailing\\\\_zeros()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
