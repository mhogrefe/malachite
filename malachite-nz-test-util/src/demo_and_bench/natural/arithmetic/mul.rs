use malachite_base_test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, pair_sum_vec_len_bucketer, quadruple_2_vec_len_bucketer,
    triple_1_vec_len_bucketer, triple_2_3_sum_vec_len_bucketer, triple_2_vec_len_bucketer,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    large_type_gen_var_1, unsigned_pair_gen, unsigned_vec_pair_gen_var_1,
    unsigned_vec_pair_gen_var_2, unsigned_vec_triple_gen_var_1, unsigned_vec_triple_gen_var_2,
    unsigned_vec_triple_gen_var_3, unsigned_vec_unsigned_pair_gen,
    unsigned_vec_unsigned_unsigned_triple_gen, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::fft::*;
use malachite_nz::natural::arithmetic::mul::limb::{
    limbs_mul_limb, limbs_mul_limb_to_out, limbs_mul_limb_with_carry_to_out,
    limbs_slice_mul_limb_in_place, limbs_slice_mul_limb_with_carry_in_place,
    limbs_vec_mul_limb_in_place,
};
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22, _limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_22_scratch_len, _limbs_mul_greater_to_out_toom_32,
    _limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_32_scratch_len, _limbs_mul_greater_to_out_toom_33,
    _limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_33_scratch_len, _limbs_mul_greater_to_out_toom_42,
    _limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_42_scratch_len, _limbs_mul_greater_to_out_toom_43,
    _limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_43_scratch_len, _limbs_mul_greater_to_out_toom_44,
    _limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_44_scratch_len, _limbs_mul_greater_to_out_toom_52,
    _limbs_mul_greater_to_out_toom_52_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_52_scratch_len, _limbs_mul_greater_to_out_toom_53,
    _limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_53_scratch_len, _limbs_mul_greater_to_out_toom_54,
    _limbs_mul_greater_to_out_toom_54_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_54_scratch_len, _limbs_mul_greater_to_out_toom_62,
    _limbs_mul_greater_to_out_toom_62_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_62_scratch_len, _limbs_mul_greater_to_out_toom_63,
    _limbs_mul_greater_to_out_toom_63_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_63_scratch_len, _limbs_mul_greater_to_out_toom_6h,
    _limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_6h_scratch_len, _limbs_mul_greater_to_out_toom_8h,
    _limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_8h_scratch_len,
};
use malachite_nz::natural::arithmetic::mul::{
    _limbs_mul_greater_to_out_basecase, limbs_mul, limbs_mul_greater, limbs_mul_greater_to_out,
    limbs_mul_same_length_to_out, limbs_mul_to_out,
};
use malachite_nz_test_util::generators::{
    unsigned_vec_triple_gen_var_10, unsigned_vec_triple_gen_var_11, unsigned_vec_triple_gen_var_12,
    unsigned_vec_triple_gen_var_13, unsigned_vec_triple_gen_var_14, unsigned_vec_triple_gen_var_15,
    unsigned_vec_triple_gen_var_16, unsigned_vec_triple_gen_var_18, unsigned_vec_triple_gen_var_19,
    unsigned_vec_triple_gen_var_20, unsigned_vec_triple_gen_var_21, unsigned_vec_triple_gen_var_22,
    unsigned_vec_triple_gen_var_23, unsigned_vec_triple_gen_var_4, unsigned_vec_triple_gen_var_5,
    unsigned_vec_triple_gen_var_6, unsigned_vec_triple_gen_var_7, unsigned_vec_triple_gen_var_8,
    unsigned_vec_triple_gen_var_9,
};
use malachite_nz_test_util::natural::arithmetic::mul::_limbs_mul_greater_to_out_basecase_mem_opt;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_mul_limb);
    register_demo!(runner, demo_limbs_mul_limb_with_carry_to_out);
    register_demo!(runner, demo_limbs_mul_limb_to_out);
    register_demo!(runner, demo_limbs_slice_mul_limb_with_carry_in_place);
    register_demo!(runner, demo_limbs_slice_mul_limb_in_place);
    register_demo!(runner, demo_limbs_vec_mul_limb_in_place);
    register_demo!(runner, demo_limbs_mul_greater);
    register_demo!(runner, demo_limbs_mul);
    register_demo!(runner, demo_limbs_mul_same_length_to_out);
    register_demo!(runner, demo_limbs_mul_greater_to_out);
    register_demo!(runner, demo_limbs_mul_to_out);
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_22_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_32_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_33_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_42_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_43_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_44_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_52_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_53_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_54_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_62_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_63_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_6h_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_toom_8h_input_sizes_valid
    );
    register_demo!(
        runner,
        demo_limbs_mul_greater_to_out_fft_input_sizes_threshold
    );
    register_bench!(runner, benchmark_limbs_mul_limb);
    register_bench!(runner, benchmark_limbs_mul_limb_with_carry_to_out);
    register_bench!(runner, benchmark_limbs_mul_limb_to_out);
    register_bench!(runner, benchmark_limbs_slice_mul_limb_with_carry_in_place);
    register_bench!(runner, benchmark_limbs_slice_mul_limb_in_place);
    register_bench!(runner, benchmark_limbs_vec_mul_limb_in_place);
    register_bench!(runner, benchmark_limbs_mul_greater);
    register_bench!(runner, benchmark_limbs_mul);
    register_bench!(runner, benchmark_limbs_mul_same_length_to_out);
    register_bench!(runner, benchmark_limbs_mul_greater_to_out_algorithms);
    register_bench!(runner, benchmark_limbs_mul_to_out);
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_basecase_mem_opt_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_22_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_32_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_33_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_33_same_length_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_42_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_43_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_44_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_44_same_length_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_52_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_53_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_54_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_62_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_63_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_6h_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_6h_same_length_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_8h_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_8h_same_length_algorithms
    );
    register_bench!(runner, benchmark_limbs_mul_greater_to_out_fft_algorithms);
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_32_to_43_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_toom_42_to_53_algorithms
    );
    register_bench!(
        runner,
        benchmark_limbs_mul_greater_to_out_fft_same_length_algorithms
    );
}

fn demo_limbs_mul_limb(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "limbs_mul_limb({:?}, {}) = {:?}",
            xs,
            y,
            limbs_mul_limb(&xs, y)
        );
    }
}

fn demo_limbs_mul_limb_with_carry_to_out(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, xs, y, carry) in large_type_gen_var_1().get(gm, &config).take(limit) {
        let out_old = out.clone();
        let carry_out = limbs_mul_limb_with_carry_to_out(&mut out, &xs, y, carry);
        println!(
            "out := {:?}; limbs_mul_limb_with_carry_to_out(&mut out, {:?}, {}, {}) = {}; \
             out = {:?}",
            out_old, xs, y, carry, carry_out, out
        );
    }
}

fn demo_limbs_mul_limb_to_out(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        let out_old = out.clone();
        let carry = limbs_mul_limb_to_out(&mut out, &xs, y);
        println!(
            "out := {:?}; limbs_mul_limb_to_out(&mut out, {:?}, {}) = {}; out = {:?}",
            out_old, xs, y, carry, out
        );
    }
}

fn demo_limbs_slice_mul_limb_with_carry_in_place(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, y, carry) in unsigned_vec_unsigned_unsigned_triple_gen()
        .get(gm, &config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry_out = limbs_slice_mul_limb_with_carry_in_place(&mut xs, y, carry);
        println!(
            "xs := {:?}; limbs_slice_mul_limb_with_carry_in_place(&mut xs, {}, {}) = {}; xs = {:?}",
            xs_old, y, carry, carry_out, xs
        );
    }
}

fn demo_limbs_slice_mul_limb_in_place(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen()
        .get(gm, &config)
        .take(limit)
    {
        let xs_old = xs.clone();
        let carry = limbs_slice_mul_limb_in_place(&mut xs, y);
        println!(
            "xs := {:?}; limbs_slice_mul_limb_in_place(&mut xs, {}) = {}; xs = {:?}",
            xs_old, y, carry, xs
        );
    }
}

fn demo_limbs_vec_mul_limb_in_place(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen()
        .get(gm, &config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_vec_mul_limb_in_place(&mut xs, y);
        println!(
            "xs := {:?}; limbs_vec_mul_limb_in_place(&mut xs, {}); xs = {:?}",
            xs_old, y, xs
        );
    }
}

fn demo_limbs_mul_greater(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_1().get(gm, &config).take(limit) {
        println!(
            "limbs_mul_greater({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_mul_greater(&xs, &ys)
        );
    }
}

fn demo_limbs_mul(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_2().get(gm, &config).take(limit) {
        println!("limbs_mul({:?}, {:?}) = {:?}", xs, ys, limbs_mul(&xs, &ys));
    }
}

fn demo_limbs_mul_same_length_to_out(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_1().get(gm, &config).take(limit) {
        let out_old = out.clone();
        limbs_mul_same_length_to_out(&mut out, &xs, &ys);
        println!(
            "out := {:?}; limbs_mul_same_length_to_out(&mut out, {:?}, {:?}); out = {:?}",
            out_old, xs, ys, out
        );
    }
}

fn demo_limbs_mul_greater_to_out(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_2().get(gm, &config).take(limit) {
        let out_old = out.clone();
        let carry = limbs_mul_greater_to_out(&mut out, &xs, &ys);
        println!(
            "out := {:?}; limbs_mul_greater_to_out(&mut out, {:?}, {:?}) = {}; out = {:?}",
            out_old, xs, ys, carry, out
        );
    }
}

fn demo_limbs_mul_to_out(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_3().get(gm, &config).take(limit) {
        let out_old = out.clone();
        let carry = limbs_mul_to_out(&mut out, &xs, &ys);
        println!(
            "out := {:?}; limbs_mul_to_out(&mut out, {:?}, {:?}) = {}; out = {:?}",
            out_old, xs, ys, carry, out
        );
    }
}

macro_rules! mul_valid_helper {
    ($name: ident, $demo_name: ident) => {
        fn $demo_name(gm: GenMode, config: GenConfig, limit: usize) {
            for (x, y) in unsigned_pair_gen().get(gm, &config).take(limit) {
                println!(
                    concat!(stringify!($name), "({}, {}) = {}"),
                    x,
                    y,
                    $name(x, y)
                );
            }
        }
    };
}
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_22_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_32_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_33_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_42_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_43_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_44_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_52_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_52_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_53_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_54_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_54_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_62_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_62_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_63_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_63_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_6h_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
    demo_limbs_mul_greater_to_out_toom_8h_input_sizes_valid
);
mul_valid_helper!(
    _limbs_mul_greater_to_out_fft_input_sizes_threshold,
    demo_limbs_mul_greater_to_out_fft_input_sizes_threshold
);

fn benchmark_limbs_mul_limb(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| no_out!(limbs_mul_limb(&xs, y)))],
    );
}

fn benchmark_limbs_mul_limb_with_carry_to_out(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_limb_with_carry_to_out(&mut [Limb], &[Limb], Limb, Limb)",
        BenchmarkType::Single,
        large_type_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &quadruple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y, carry)| {
            no_out!(limbs_mul_limb_with_carry_to_out(&mut out, &xs, y, carry))
        })],
    );
}

fn benchmark_limbs_mul_limb_to_out(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y)| {
            no_out!(limbs_mul_limb_to_out(&mut out, &xs, y))
        })],
    );
}

fn benchmark_limbs_slice_mul_limb_with_carry_in_place(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_mul_limb_with_carry_in_place(&mut [Limb], Limb, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y, carry)| {
            no_out!(limbs_slice_mul_limb_with_carry_in_place(&mut xs, y, carry))
        })],
    );
}

fn benchmark_limbs_slice_mul_limb_in_place(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_mul_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            no_out!(limbs_slice_mul_limb_in_place(&mut xs, y))
        })],
    );
}

fn benchmark_limbs_vec_mul_limb_in_place(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_mul_limb_in_place(&mut Vec<Limb>, Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            limbs_vec_mul_limb_in_place(&mut xs, y)
        })],
    );
}

fn benchmark_limbs_mul_greater(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_greater(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_sum_vec_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| {
            no_out!(limbs_mul_greater(&xs, &ys))
        })],
    );
}

fn benchmark_limbs_mul(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_sum_vec_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(xs, ys)| no_out!(limbs_mul(&xs, &ys)))],
    );
}

fn benchmark_limbs_mul_same_length_to_out(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_same_length_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            limbs_mul_same_length_to_out(&mut out, &xs, &ys)
        })],
    );
}

fn benchmark_limbs_mul_greater_to_out_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("full", &mut |(mut out, xs, ys)| {
                no_out!(limbs_mul_greater_to_out(&mut out, &xs, &ys))
            }),
        ],
    );
}

fn benchmark_limbs_mul_to_out(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_mul_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [("Malachite", &mut |(mut out, xs, ys)| {
            no_out!(limbs_mul_to_out(&mut out, &xs, &ys))
        })],
    );
}

fn benchmark_limbs_mul_greater_to_out_basecase_mem_opt_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_mul_greater_to_out_basecase_mem_opt(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("limbs_mul_greater_to_out_basecase", &mut |(
                mut out,
                xs,
                ys,
            )| {
                _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            (
                "limbs_mul_greater_to_out_basecase_mem_opt",
                &mut |(mut out, xs, ys)| {
                    _limbs_mul_greater_to_out_basecase_mem_opt(&mut out, &xs, &ys)
                },
            ),
        ],
    );
}

macro_rules! bench_mul_helper_1 {
    ($bench: ident, $mul: ident, $scratch: ident, $gen: ident, $name: expr) => {
        fn $bench(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
            run_benchmark(
                concat!(stringify!($mul), "(&mut [Limb], &[Limb], &[Limb])"),
                BenchmarkType::Algorithms,
                $gen().get(gm, &config),
                gm.name(),
                limit,
                file_name,
                &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
                &mut [
                    ("basecase", &mut |(mut out, xs, ys)| {
                        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
                    }),
                    ($name, &mut |(mut out, xs, ys)| {
                        let mut scratch = vec![0; $scratch(xs.len())];
                        $mul(&mut out, &xs, &ys, &mut scratch)
                    }),
                ],
            );
        }
    };
}

macro_rules! bench_mul_helper_2 {
    ($bench: ident, $mul: ident, $scratch: ident, $gen: ident, $name: expr) => {
        fn $bench(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
            run_benchmark(
                concat!(stringify!($mul), "(&mut [Limb], &[Limb], &[Limb])"),
                BenchmarkType::Algorithms,
                $gen().get(gm, &config),
                gm.name(),
                limit,
                file_name,
                &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
                &mut [
                    ("basecase", &mut |(mut out, xs, ys)| {
                        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
                    }),
                    ($name, &mut |(mut out, xs, ys)| {
                        let mut scratch = vec![0; $scratch(xs.len(), ys.len())];
                        $mul(&mut out, &xs, &ys, &mut scratch)
                    }),
                ],
            );
        }
    };
}
bench_mul_helper_1!(
    benchmark_limbs_mul_greater_to_out_toom_22_algorithms,
    _limbs_mul_greater_to_out_toom_22,
    _limbs_mul_greater_to_out_toom_22_scratch_len,
    unsigned_vec_triple_gen_var_4,
    "Toom22"
);
bench_mul_helper_2!(
    benchmark_limbs_mul_greater_to_out_toom_32_algorithms,
    _limbs_mul_greater_to_out_toom_32,
    _limbs_mul_greater_to_out_toom_32_scratch_len,
    unsigned_vec_triple_gen_var_5,
    "Toom32"
);
bench_mul_helper_1!(
    benchmark_limbs_mul_greater_to_out_toom_33_algorithms,
    _limbs_mul_greater_to_out_toom_33,
    _limbs_mul_greater_to_out_toom_33_scratch_len,
    unsigned_vec_triple_gen_var_6,
    "Toom33"
);
bench_mul_helper_2!(
    benchmark_limbs_mul_greater_to_out_toom_42_algorithms,
    _limbs_mul_greater_to_out_toom_42,
    _limbs_mul_greater_to_out_toom_42_scratch_len,
    unsigned_vec_triple_gen_var_7,
    "Toom42"
);
bench_mul_helper_2!(
    benchmark_limbs_mul_greater_to_out_toom_43_algorithms,
    _limbs_mul_greater_to_out_toom_43,
    _limbs_mul_greater_to_out_toom_43_scratch_len,
    unsigned_vec_triple_gen_var_8,
    "Toom43"
);
bench_mul_helper_1!(
    benchmark_limbs_mul_greater_to_out_toom_44_algorithms,
    _limbs_mul_greater_to_out_toom_44,
    _limbs_mul_greater_to_out_toom_44_scratch_len,
    unsigned_vec_triple_gen_var_9,
    "Toom44"
);
bench_mul_helper_2!(
    benchmark_limbs_mul_greater_to_out_toom_52_algorithms,
    _limbs_mul_greater_to_out_toom_52,
    _limbs_mul_greater_to_out_toom_52_scratch_len,
    unsigned_vec_triple_gen_var_10,
    "Toom52"
);
bench_mul_helper_2!(
    benchmark_limbs_mul_greater_to_out_toom_54_algorithms,
    _limbs_mul_greater_to_out_toom_54,
    _limbs_mul_greater_to_out_toom_54_scratch_len,
    unsigned_vec_triple_gen_var_12,
    "Toom54"
);
bench_mul_helper_2!(
    benchmark_limbs_mul_greater_to_out_toom_62_algorithms,
    _limbs_mul_greater_to_out_toom_62,
    _limbs_mul_greater_to_out_toom_62_scratch_len,
    unsigned_vec_triple_gen_var_13,
    "Toom62"
);
bench_mul_helper_2!(
    benchmark_limbs_mul_greater_to_out_toom_6h_algorithms,
    _limbs_mul_greater_to_out_toom_6h,
    _limbs_mul_greater_to_out_toom_6h_scratch_len,
    unsigned_vec_triple_gen_var_15,
    "Toom6h"
);
bench_mul_helper_2!(
    benchmark_limbs_mul_greater_to_out_toom_8h_algorithms,
    _limbs_mul_greater_to_out_toom_8h,
    _limbs_mul_greater_to_out_toom_8h_scratch_len,
    unsigned_vec_triple_gen_var_16,
    "Toom8h"
);

fn benchmark_limbs_mul_greater_to_out_toom_53_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_53(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_11().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("Toom32", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_32_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom42", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom53", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_53_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_63_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_63(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_14().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("Toom42", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom63", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_63_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_fft_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_fft(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("FFT", &mut |(mut out, xs, ys)| {
                _limbs_mul_greater_to_out_fft(&mut out, &xs, &ys)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_33_same_length_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_33(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_18().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("Toom22", &mut |(mut out, xs, ys)| {
                let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_22_scratch_len(xs.len())];
                _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom33", &mut |(mut out, xs, ys)| {
                let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_len(xs.len())];
                _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_44_same_length_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_44(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_18().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("Toom33", &mut |(mut out, xs, ys)| {
                let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_len(xs.len())];
                _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom44", &mut |(mut out, xs, ys)| {
                let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_len(xs.len())];
                _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_6h_same_length_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_6h(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_19().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("Toom44", &mut |(mut out, xs, ys)| {
                let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_len(xs.len())];
                _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom6h", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_8h_same_length_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_toom_8h(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_20().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("basecase", &mut |(mut out, xs, ys)| {
                _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys)
            }),
            ("Toom6h", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom8h", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_8h_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_32_to_43_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Toom32 to Toom43",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_22().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("Toom32", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_32_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom43", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_43_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_toom_42_to_53_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Toom42 to Toom53",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_23().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_3_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("Toom42", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch)
            }),
            ("Toom53", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_53_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch)
            }),
        ],
    );
}

fn benchmark_limbs_mul_greater_to_out_fft_same_length_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "_limbs_mul_greater_to_out_fft(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        unsigned_vec_triple_gen_var_21().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [
            ("Toom42", &mut |(mut out, xs, ys)| {
                let mut scratch =
                    vec![0; _limbs_mul_greater_to_out_toom_8h_scratch_len(xs.len(), ys.len())];
                _limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch)
            }),
            ("FFT", &mut |(mut out, xs, ys)| {
                _limbs_mul_greater_to_out_fft(&mut out, &xs, &ys)
            }),
        ],
    );
}
