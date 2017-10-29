use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::vecs::{exhaustive_vecs, random_vecs};

pub fn demo_exhaustive_integer_from_twos_complement_limbs_le(limit: usize) {
    for xs in exhaustive_vecs(exhaustive_u::<u32>()).take(limit) {
        println!(
            "from_twos_complement_limbs_le({:?}) = {:?}",
            xs,
            gmp::Integer::from_twos_complement_limbs_le(xs.as_slice())
        );
    }
}

pub fn demo_random_integer_from_twos_complement_limbs_le(limit: usize) {
    for xs in random_vecs(&EXAMPLE_SEED, 4, &(|seed| random_x::<u32>(seed))).take(limit) {
        println!(
            "from_twos_complement_limbs_le({:?}) = {:?}",
            xs,
            gmp::Integer::from_twos_complement_limbs_le(xs.as_slice())
        );
    }
}

pub fn demo_exhaustive_integer_from_twos_complement_limbs_be(limit: usize) {
    for xs in exhaustive_vecs(exhaustive_u::<u32>()).take(limit) {
        println!(
            "from_twos_complement_limbs_be({:?}) = {:?}",
            xs,
            gmp::Integer::from_twos_complement_limbs_be(xs.as_slice())
        );
    }
}

pub fn demo_random_integer_from_twos_complement_limbs_be(limit: usize) {
    for xs in random_vecs(&EXAMPLE_SEED, 4, &(|seed| random_x::<u32>(seed))).take(limit) {
        println!(
            "from_twos_complement_limbs_be({:?}) = {:?}",
            xs,
            gmp::Integer::from_twos_complement_limbs_be(xs.as_slice())
        );
    }
}

pub fn benchmark_exhaustive_integer_from_twos_complement_limbs_le(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer::from_twos_complement_limbs_le(&[u32])");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_vecs(exhaustive_u::<u32>()),
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

pub fn benchmark_random_integer_from_twos_complement_limbs_le(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer::from_twos_complement_limbs_le(&[u32])");
    benchmark_2(BenchmarkOptions2 {
        xs: random_vecs(&EXAMPLE_SEED, scale, &(|seed| random_x::<u32>(seed))),
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

pub fn benchmark_exhaustive_integer_from_twos_complement_limbs_be(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer::from_twos_complement_limbs_be(&[u32])");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_vecs(exhaustive_u::<u32>()),
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

pub fn benchmark_random_integer_from_twos_complement_limbs_be(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer::from_twos_complement_limbs_be(&[u32])");
    benchmark_2(BenchmarkOptions2 {
        xs: random_vecs(&EXAMPLE_SEED, scale, &(|seed| random_x::<u32>(seed))),
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
