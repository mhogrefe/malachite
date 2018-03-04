use common::{integer_to_rug_integer, GenerationMode};
use inputs::integer::pairs_of_integers;
use malachite_base::num::SignificantBits;
use malachite_base::num::OrdAbs;
use malachite_nz::integer::Integer;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use std::cmp::{max, Ordering};

pub fn demo_integer_cmp_abs(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        match x.cmp_abs(&y) {
            Ordering::Less => println!("|{}| < |{}|", x, y),
            Ordering::Equal => println!("|{}| = |{}|", x, y),
            Ordering::Greater => println!("|{}| > |{}|", x, y),
        }
    }
}

pub fn benchmark_integer_cmp_abs(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.cmp_abs(&Integer)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integers(gm),
        function_f: &mut (|(x, y): (Integer, Integer)| x.cmp(&y)),
        function_g: &mut (|(x, y): (rug::Integer, rug::Integer)| x.cmp(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (integer_to_rug_integer(x), integer_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer.cmp_abs(&Integer)",
        x_axis_label: "max(x.significant_bits(), y.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
