use common::GenerationMode;
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, benchmark_1, benchmark_2};

pub fn demo_integer_into_natural(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!("into_natural({}) = {:?}", n_clone, n.into_natural());
    }
}

pub fn demo_integer_to_natural(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_natural(&{}) = {:?}", n, n.to_natural());
    }
}

pub fn benchmark_integer_to_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.to_natural()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &(|n: Integer| n.into_natural()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.to\\\\_natural()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_to_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.to_natural() evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: integers(gm),
        function_f: &(|n: Integer| n.into_natural()),
        function_g: &(|n: Integer| n.to_natural()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "into\\\\_natural (by value)",
        g_name: "to\\\\_natural (by reference)",
        title: "Integer.to\\\\_natural()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
