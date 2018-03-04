use common::GenerationMode;
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};

pub fn demo_natural_not(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("!({}) = {}", n.clone(), !n);
    }
}

pub fn demo_natural_not_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("!(&{}) = {}", n, !&n);
    }
}

pub fn benchmark_natural_not(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} !Natural", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| !n),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "-Natural",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_not_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} !Natural evaluation strategy", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| !n),
        function_g: &mut (|n: Natural| !&n),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "-Natural",
        g_name: "-&Natural",
        title: "-Natural evaluation strategy",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
