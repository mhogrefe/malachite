use common::GenerationMode;
use inputs::base::vecs_of_unsigned;
use malachite_base::limbs::limbs_set_zero;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_limbs_set_zero(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        let mut mut_xs = xs.clone();
        limbs_set_zero(&mut mut_xs);
        println!("xs := {:?}; limbs_set_zero(&mut xs); x = {:?}", xs, mut_xs);
    }
}

pub fn benchmark_limbs_set_zero(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} limbs_set_zero(&mut [u32])", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: vecs_of_unsigned(gm),
        function_f: &(|mut xs: Vec<u32>| limbs_set_zero(&mut xs)),
        x_cons: &(|xs| xs.clone()),
        x_param: &(|xs| xs.len()),
        limit,
        f_name: "malachite",
        title: "limbs\\\\_set\\\\_zero(\\\\&mut [u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
