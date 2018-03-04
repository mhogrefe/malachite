use common::GenerationMode;
use inputs::base::pairs_of_ordering_and_vec_of_unsigned_var_1;
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};
use std::cmp::Ordering;

pub fn demo_integer_from_sign_and_limbs_asc(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in pairs_of_ordering_and_vec_of_unsigned_var_1(gm).take(limit) {
        println!(
            "from_sign_and_limbs_asc({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_limbs_asc(sign, limbs.as_slice())
        );
    }
}

pub fn demo_integer_from_sign_and_limbs_desc(gm: GenerationMode, limit: usize) {
    for (sign, limbs) in pairs_of_ordering_and_vec_of_unsigned_var_1(gm).take(limit) {
        println!(
            "from_sign_and_limbs_desc({:?}, {:?}) = {:?}",
            sign,
            limbs,
            Integer::from_sign_and_limbs_desc(sign, limbs.as_slice())
        );
    }
}

pub fn benchmark_integer_from_sign_and_limbs_asc(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer::from_sign_and_limbs_asc(&[u32])",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_ordering_and_vec_of_unsigned_var_1(gm),
        function_f: &mut (|(sign, limbs): (Ordering, Vec<u32>)| {
            Integer::from_sign_and_limbs_asc(sign, limbs.as_slice())
        }),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref limbs)| limbs.len()),
        limit,
        f_name: "malachite",
        title: "Integer::from_sign_and_limbs_le(Ordering, &[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_from_sign_and_limbs_desc(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer::from_sign_and_limbs_desc(&[u32])",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_ordering_and_vec_of_unsigned_var_1(gm),
        function_f: &mut (|(sign, limbs): (Ordering, Vec<u32>)| {
            Integer::from_sign_and_limbs_desc(sign, limbs.as_slice())
        }),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref limbs)| limbs.len()),
        limit,
        f_name: "malachite",
        title: "Integer::from_sign_and_limbs_be(Ordering, &[u32])",
        x_axis_label: "xs.len()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
