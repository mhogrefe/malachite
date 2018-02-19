use common::GenerationMode;
use inputs::natural::naturals;
use malachite_base::num::{IsPowerOfTwo, SignificantBits};
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_natural_is_power_of_two(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        if n.is_power_of_two() {
            println!("{} is a power of two", n);
        } else {
            println!("{} is not a power of two", n);
        }
    }
}

pub fn benchmark_natural_is_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.is_power_of_two()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| n.is_power_of_two()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Natural.is\\\\_power\\\\_of\\\\_two()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
