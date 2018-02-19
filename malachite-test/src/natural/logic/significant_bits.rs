use common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use num::BigUint;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};

pub fn demo_natural_significant_bits(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

pub fn benchmark_natural_significant_bits(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.significant_bits()", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| n.significant_bits()),
        function_g: &mut (|n: BigUint| n.bits()),
        function_h: &mut (|n: rug::Integer| n.significant_bits()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| natural_to_biguint(x)),
        z_cons: &(|x| natural_to_rug_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Natural.significant\\\\_bits()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
