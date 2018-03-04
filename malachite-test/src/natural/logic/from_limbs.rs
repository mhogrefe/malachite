use common::GenerationMode;
use inputs::base::vecs_of_unsigned;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_natural_from_limbs_asc(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_limbs_asc({:?}) = {:?}",
            xs,
            Natural::from_limbs_asc(xs.as_slice())
        );
    }
}

pub fn demo_natural_from_limbs_desc(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_limbs_desc({:?}) = {:?}",
            xs,
            Natural::from_limbs_desc(xs.as_slice())
        );
    }
}

pub fn benchmark_natural_from_limbs_asc(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural::from_limbs_asc(&[u32])", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: vecs_of_unsigned(gm),
        function_f: &mut (|xs: Vec<u32>| Natural::from_limbs_asc(xs.as_slice())),
        x_cons: &(|xs| xs.clone()),
        x_param: &(|xs| xs.len()),
        limit,
        f_name: "malachite",
        title: "Natural::from_limbs_le(&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_from_limbs_desc(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural::from_limbs_desc(&[u32])",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: vecs_of_unsigned(gm),
        function_f: &mut (|xs: Vec<u32>| Natural::from_limbs_desc(xs.as_slice())),
        x_cons: &(|xs| xs.clone()),
        x_param: &(|xs| xs.len()),
        limit,
        f_name: "malachite",
        title: "Natural::from_limbs_be(&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
