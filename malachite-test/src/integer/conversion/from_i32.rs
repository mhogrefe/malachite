use common::GenerationMode;
use inputs::base::signeds;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};

pub fn demo_integer_from_i32(gm: GenerationMode, limit: usize) {
    for i in signeds::<i32>(gm).take(limit) {
        println!("from({}) = {}", i, Integer::from(i));
    }
}

pub fn benchmark_integer_from_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer::from(i32)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: signeds::<i32>(gm),
        function_f: &(|i| Integer::from(i)),
        function_g: &(|i| BigInt::from(i)),
        function_h: &(|i| rug::Integer::from(i)),
        x_cons: &(|&i| i),
        y_cons: &(|&i| i),
        z_cons: &(|&i| i),
        x_param: &(|&i| i.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer::from(i32)",
        x_axis_label: "i.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
