use common::GenerationMode;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::vecs::{exhaustive_vecs, random_vecs};

type It = Iterator<Item = Vec<u32>>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_vecs(exhaustive_u()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_vecs(
        &EXAMPLE_SEED,
        scale,
        &(|seed| random_x::<u32>(seed)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_from_twos_complement_limbs_le(gm: GenerationMode, limit: usize) {
    for xs in select_inputs(gm).take(limit) {
        println!(
            "from_twos_complement_limbs_le({:?}) = {:?}",
            xs,
            gmp::Integer::from_twos_complement_limbs_le(xs.as_slice())
        );
    }
}

pub fn demo_integer_from_twos_complement_limbs_be(gm: GenerationMode, limit: usize) {
    for xs in select_inputs(gm).take(limit) {
        println!(
            "from_twos_complement_limbs_be({:?}) = {:?}",
            xs,
            gmp::Integer::from_twos_complement_limbs_be(xs.as_slice())
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
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|xs: Vec<u32>| gmp::Integer::from_twos_complement_limbs_le(xs.as_slice())),
        function_g: &(|xs: Vec<u32>| native::Integer::from_twos_complement_limbs_le(xs.as_slice())),
        x_cons: &(|xs| xs.clone()),
        y_cons: &(|xs| xs.clone()),
        x_param: &(|xs| xs.len()),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
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
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|xs: Vec<u32>| gmp::Integer::from_twos_complement_limbs_be(xs.as_slice())),
        function_g: &(|xs: Vec<u32>| native::Integer::from_twos_complement_limbs_be(xs.as_slice())),
        x_cons: &(|xs| xs.clone()),
        y_cons: &(|xs| xs.clone()),
        x_param: &(|xs| xs.len()),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer::from\\\\_twos\\\\_complement\\\\_limbs\\\\_le(\\\\&[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
