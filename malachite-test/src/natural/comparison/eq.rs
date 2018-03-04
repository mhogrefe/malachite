use common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use inputs::natural::pairs_of_naturals;
use malachite_base::num::SignificantBits;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use std::cmp::max;

pub fn demo_natural_eq(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

pub fn benchmark_natural_eq(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural == Natural", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_naturals(gm),
        function_f: &mut (|(x, y)| x == y),
        function_g: &mut (|(x, y)| x == y),
        function_h: &mut (|(x, y)| x == y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_biguint(x), natural_to_biguint(y))),
        z_cons: &(|&(ref x, ref y)| (natural_to_rug_integer(x), natural_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Natural == Natural",
        x_axis_label: "max(x.significant_bits(), y.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
