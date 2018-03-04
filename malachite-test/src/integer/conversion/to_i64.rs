use common::GenerationMode;
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_integer_to_i64(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_i64({}) = {:?}", n, n.to_i64());
    }
}

pub fn demo_integer_to_i64_wrapping(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_i64_wrapping({}) = {:?}", n, n.to_i64_wrapping());
    }
}

pub fn benchmark_integer_to_i64(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.to_i64()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &mut (|n: Integer| n.to_i64()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.to_i64()",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_to_i64_wrapping(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.to_i64_wrapping()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &mut (|n: Integer| n.to_i64_wrapping()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.to_i64_wrapping()",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
