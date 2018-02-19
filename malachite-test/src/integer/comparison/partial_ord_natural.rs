use common::{integer_to_rug_integer, natural_to_rug_integer, GenerationMode};
use inputs::integer::{pairs_of_integer_and_natural, pairs_of_natural_and_integer};
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use std::cmp::{max, Ordering};

pub fn demo_integer_partial_cmp_natural(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_natural(gm).take(limit) {
        match x.partial_cmp(&y).unwrap() {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

pub fn demo_natural_partial_cmp_integer(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_integer(gm).take(limit) {
        match x.partial_cmp(&y).unwrap() {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

pub fn benchmark_integer_partial_cmp_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.partial_cmp(&Natural)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_natural(gm),
        function_f: &mut (|(x, y): (Integer, Natural)| x.partial_cmp(&y)),
        function_g: &mut (|(x, y): (rug::Integer, rug::Integer)| x.partial_cmp(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (integer_to_rug_integer(x), natural_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer.partial\\\\_cmp(\\\\&Natural)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_partial_cmp_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.partial_cmp(&Integer)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_natural_and_integer(gm),
        function_f: &mut (|(x, y): (Natural, Integer)| x.partial_cmp(&y)),
        function_g: &mut (|(x, y): (rug::Integer, rug::Integer)| x.partial_cmp(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_rug_integer(x), integer_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Natural.partial\\\\_cmp(\\\\&Integer)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
