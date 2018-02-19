use common::GenerationMode;
use inputs::natural::naturals;
use malachite_base::misc::Walkable;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_natural_increment(gm: GenerationMode, limit: usize) {
    for mut n in naturals(gm).take(limit) {
        let n_old = n.clone();
        n.increment();
        println!("n := {:?}; n.increment(); n = {:?}", n_old, n);
    }
}

pub fn benchmark_natural_increment(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.increment()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: naturals(gm),
        function_f: &mut (|mut n: Natural| n.increment()),
        x_cons: &(|n| n.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.increment()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
