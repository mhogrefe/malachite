use common::{integer_to_bigint, GenerationMode};
use hash::hash;
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};

pub fn demo_integer_hash(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("hash({}) = {}", n, hash(&n));
    }
}

pub fn benchmark_integer_hash(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer hash", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: integers(gm),
        function_f: &(|n| hash(&n)),
        function_g: &(|n| hash(&n)),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| integer_to_bigint(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Integer hash",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
