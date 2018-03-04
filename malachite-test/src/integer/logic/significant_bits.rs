use common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};

pub fn demo_integer_significant_bits(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

pub fn benchmark_integer_significant_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.significant_bits()", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: integers(gm),
        function_f: &mut (|n: Integer| n.significant_bits()),
        function_g: &mut (|n: BigInt| n.bits()),
        function_h: &mut (|n: rug::Integer| n.significant_bits()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| integer_to_bigint(x)),
        z_cons: &(|x| integer_to_rug_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer.significant_bits()",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
