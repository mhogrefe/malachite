use common::GenerationMode;
use inputs::base::unsigneds;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigUint;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};

pub fn demo_integer_from_u32(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<u32>(gm).take(limit) {
        println!("from({}) = {}", u, Integer::from(u));
    }
}

pub fn benchmark_integer_from_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer::from(u32)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: unsigneds::<u32>(gm),
        function_f: &mut (|u| Integer::from(u)),
        function_g: &mut (|u| BigUint::from(u)),
        function_h: &mut (|u| rug::Integer::from(u)),
        x_cons: &(|&u| u),
        y_cons: &(|&u| u),
        z_cons: &(|&u| u),
        x_param: &(|&u| u.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer::from(u32)",
        x_axis_label: "u.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
