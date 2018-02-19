use common::GenerationMode;
use inputs::base::unsigneds;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use num::BigUint;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};

pub fn demo_natural_from_u32(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<u32>(gm).take(limit) {
        println!("from({}) = {}", u, Natural::from(u));
    }
}

pub fn benchmark_natural_from_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural::from(u32)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: unsigneds::<u32>(gm),
        function_f: &mut (|u| Natural::from(u)),
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
        title: "Natural::from(u32)",
        x_axis_label: "u.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
