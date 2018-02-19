use common::GenerationMode;
use inputs::integer::integers;
use malachite_base::misc::Walkable;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_integer_decrement(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.decrement();
        println!("n := {:?}; n.decrement(); n = {:?}", n_old, n);
    }
}

pub fn benchmark_integer_decrement(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.decrement()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &mut (|mut n: Integer| n.decrement()),
        x_cons: &(|n| n.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.decrement()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
