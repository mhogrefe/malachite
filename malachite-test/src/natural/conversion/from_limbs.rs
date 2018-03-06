use common::GenerationMode;
use inputs::base::vecs_of_unsigned;
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};

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

pub fn demo_natural_from_owned_limbs_asc(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_owned_limbs_asc({:?}) = {:?}",
            xs,
            Natural::from_owned_limbs_asc(xs.clone())
        );
    }
}

pub fn demo_natural_from_owned_limbs_desc(gm: GenerationMode, limit: usize) {
    for xs in vecs_of_unsigned(gm).take(limit) {
        println!(
            "from_owned_limbs_desc({:?}) = {:?}",
            xs,
            Natural::from_owned_limbs_desc(xs.clone())
        );
    }
}

pub fn benchmark_natural_from_limbs_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural::from_limbs_asc(&[u32]) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: vecs_of_unsigned(gm),
        function_f: &mut (|limbs: Vec<u32>| Natural::from_limbs_asc(&limbs)),
        function_g: &mut (|limbs| Natural::from_owned_limbs_asc(limbs)),
        x_cons: &(|limbs| limbs.clone()),
        y_cons: &(|limbs| limbs.clone()),
        x_param: &(|limbs| limbs.len()),
        limit,
        f_name: "Natural::from_limbs_asc(&[u32])",
        g_name: "Natural::from_owned_limbs_asc(&[u32])",
        title: "Natural::from_limbs_asc(&[u32]) evaluation strategy",
        x_axis_label: "limbs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_from_limbs_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural::from_limbs_desc(&[u32]) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: vecs_of_unsigned(gm),
        function_f: &mut (|limbs: Vec<u32>| Natural::from_limbs_desc(&limbs)),
        function_g: &mut (|limbs| Natural::from_owned_limbs_desc(limbs)),
        x_cons: &(|limbs| limbs.clone()),
        y_cons: &(|limbs| limbs.clone()),
        x_param: &(|limbs| limbs.len()),
        limit,
        f_name: "Natural::from_limbs_desc(&[u32])",
        g_name: "Natural::from_owned_limbs_desc(&[u32])",
        title: "Natural::from_limbs_desc(&[u32]) evaluation strategy",
        x_axis_label: "limbs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
