use common::GenerationMode;
use inputs::base::vecs_of_unsigned;
use malachite_base::limbs::limbs_test_zero;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_limbs_test_zero(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "limbs_test_zero({:?}) = {:?}",
            xs,
            limbs_test_zero(xs.as_slice())
        );
    }
}

pub fn benchmark_limbs_test_zero(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} limbs_test_zero(&[u32])", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: vecs_of_unsigned(gm),
        function_f: &mut (|xs: Vec<u32>| limbs_test_zero(xs.as_slice())),
        x_cons: &(|xs| xs.clone()),
        x_param: &(|xs| xs.len()),
        limit,
        f_name: "malachite",
        title: "limbs\\\\_test\\\\_zero(\\\\&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
