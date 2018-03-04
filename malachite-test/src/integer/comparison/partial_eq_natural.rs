use common::{integer_to_rug_integer, natural_to_rug_integer, GenerationMode};
use inputs::integer::{pairs_of_integer_and_natural, pairs_of_natural_and_integer};
use malachite_base::num::SignificantBits;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use std::cmp::max;

pub fn demo_integer_partial_eq_natural(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

pub fn demo_natural_partial_eq_integer(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

pub fn benchmark_integer_partial_eq_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer == Natural", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_natural(gm),
        function_f: &mut (|(x, y)| x == y),
        function_g: &mut (|(x, y)| x == y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (integer_to_rug_integer(x), natural_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer == Natural",
        x_axis_label: "max(x.significant_bits(), y.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_partial_eq_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural == Integer", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_natural_and_integer(gm),
        function_f: &mut (|(x, y)| x == y),
        function_g: &mut (|(x, y)| x == y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_rug_integer(x), integer_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Natural == Integer",
        x_axis_label: "max(x.significant_bits(), y.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
