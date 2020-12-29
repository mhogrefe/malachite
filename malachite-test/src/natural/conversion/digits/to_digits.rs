use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::conversion::digits::general_digits::{
    _limbs_to_digits_asc_basecase, _to_digits_asc_naive,
};
use malachite_nz::natural::Natural;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::quadruples_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_to_digits_asc_basecase);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_to_digits_asc_basecase_algorithms
    );
}

fn demo_limbs_to_digits_asc_basecase(gm: GenerationMode, limit: usize) {
    for (out, len, xs, base) in quadruples_var_1(gm).take(limit) {
        let old_out = out;
        let mut out = old_out.to_vec();
        let out_len = _limbs_to_digits_asc_basecase(&mut out, len, &xs, base);
        println!(
            "out := {:?}; _limbs_to_digits_asc_basecase(&mut out, {}, {:?}, {}) = {}; out = {:?}",
            old_out, len, xs, base, out_len, out
        );
    }
}

fn benchmark_limbs_to_digits_asc_basecase_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "_limbs_to_digits_asc_basecase(&mut [u8], usize, &[Limb], u64)",
        BenchmarkType::Algorithms,
        quadruples_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            ("basecase", &mut |(ref mut out, len, ref xs, base)| {
                no_out!(_limbs_to_digits_asc_basecase(out, len, &xs, base))
            }),
            ("naive", &mut |(_, _, xs, base)| {
                no_out!(_to_digits_asc_naive::<u8, _>(
                    &Natural::from_owned_limbs_asc(xs),
                    base
                ))
            }),
        ],
    );
}
