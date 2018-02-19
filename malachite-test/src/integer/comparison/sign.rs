use common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;
use num::bigint::Sign;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};
use std::cmp::Ordering;

pub fn num_sign(x: &BigInt) -> Ordering {
    match x.sign() {
        Sign::NoSign => Ordering::Equal,
        Sign::Plus => Ordering::Greater,
        Sign::Minus => Ordering::Less,
    }
}

pub fn demo_integer_sign(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        match n.sign() {
            Ordering::Less => println!("{} is negative", n),
            Ordering::Equal => println!("{} is zero", n),
            Ordering::Greater => println!("{} is positive", n),
        }
    }
}

pub fn benchmark_integer_sign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.sign()", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: integers(gm),
        function_f: &mut (|n: Integer| n.sign()),
        function_g: &mut (|n: BigInt| num_sign(&n)),
        function_h: &mut (|n: rug::Integer| n.cmp0()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| integer_to_bigint(x)),
        z_cons: &(|x| integer_to_rug_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer.sign()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
