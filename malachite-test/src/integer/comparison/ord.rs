use common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use inputs::integer::pairs_of_integers;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use std::cmp::{max, Ordering};

pub fn demo_integer_cmp(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integers(gm).take(limit) {
        match x.cmp(&y) {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

pub fn benchmark_integer_cmp(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.cmp(&Integer)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_integers(gm),
        function_f: &mut (|(x, y): (Integer, Integer)| x.cmp(&y)),
        function_g: &mut (|(x, y): (BigInt, BigInt)| x.cmp(&y)),
        function_h: &mut (|(x, y): (rug::Integer, rug::Integer)| x.cmp(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (integer_to_bigint(x), integer_to_bigint(y))),
        z_cons: &(|&(ref x, ref y)| (integer_to_rug_integer(x), integer_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer.cmp(&Integer)",
        x_axis_label: "max(x.significant_bits(), y.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
