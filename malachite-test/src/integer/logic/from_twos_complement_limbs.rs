use common::GenerationMode;
use inputs::base::vecs_of_unsigned;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_integer_from_twos_complement_limbs_le(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_twos_complement_limbs_le({:?}) = {:?}",
            xs,
            Integer::from_twos_complement_limbs_le(xs.as_slice())
        );
    }
}

pub fn demo_integer_from_twos_complement_limbs_be(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_twos_complement_limbs_be({:?}) = {:?}",
            xs,
            Integer::from_twos_complement_limbs_be(xs.as_slice())
        );
    }
}

pub fn benchmark_integer_from_twos_complement_limbs_le(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer::from_twos_complement_limbs_le(&[u32])",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: vecs_of_unsigned(gm),
        function_f: &mut (|xs: Vec<u32>| Integer::from_twos_complement_limbs_le(xs.as_slice())),
        x_cons: &(|xs| xs.clone()),
        x_param: &(|xs| xs.len()),
        limit,
        f_name: "malachite",
        title: "Integer::from\\\\_twos\\\\_complement\\\\_limbs\\\\_le(\\\\&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_from_twos_complement_limbs_be(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer::from_twos_complement_limbs_be(&[u32])",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: vecs_of_unsigned(gm),
        function_f: &mut (|xs: Vec<u32>| Integer::from_twos_complement_limbs_be(xs.as_slice())),
        x_cons: &(|xs| xs.clone()),
        x_param: &(|xs| xs.len()),
        limit,
        f_name: "malachite",
        title: "Integer::from\\\\_twos\\\\_complement\\\\_limbs\\\\_le(\\\\&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
