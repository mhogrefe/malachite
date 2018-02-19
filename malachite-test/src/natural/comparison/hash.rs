use common::{natural_to_biguint, GenerationMode};
use hash::hash;
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;
use num::BigUint;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};

pub fn demo_natural_hash(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("hash({}) = {}", n, hash(&n));
    }
}

pub fn benchmark_natural_hash(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural hash", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: naturals(gm),
        function_f: &mut (|n| hash(&n)),
        function_g: &mut (|n: BigUint| hash(&n)),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| natural_to_biguint(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Natural hash",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
