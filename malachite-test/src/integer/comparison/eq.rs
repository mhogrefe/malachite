use common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use inputs::integer::pairs_of_integers;
use malachite_base::num::SignificantBits;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use std::cmp::max;

pub fn demo_integer_eq(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

pub fn benchmark_integer_eq(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer == Integer", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_integers(gm),
        function_f: &(|(x, y)| x == y),
        function_g: &(|(x, y)| x == y),
        function_h: &(|(x, y)| x == y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (integer_to_bigint(x), integer_to_bigint(y))),
        z_cons: &(|&(ref x, ref y)| (integer_to_rug_integer(x), integer_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer == Integer",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
