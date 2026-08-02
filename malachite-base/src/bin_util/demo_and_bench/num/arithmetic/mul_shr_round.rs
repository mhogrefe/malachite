// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{MulShrRound, MulShrRoundAssign, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};
use malachite_base::test_util::bench::bucketers::quadruple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_signed_unsigned_rounding_mode_quadruple_gen_var_1,
    unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_mul_shr_round_unsigned_unsigned);
    register_generic_demos_3_only_1_3_in_key!(
        runner,
        demo_mul_shr_round_signed_unsigned,
        [i8, u8, u8],
        [i8, u8, u16],
        [i8, u8, u32],
        [i8, u8, u64],
        [i8, u8, u128],
        [i8, u8, usize],
        [i16, u16, u8],
        [i16, u16, u16],
        [i16, u16, u32],
        [i16, u16, u64],
        [i16, u16, u128],
        [i16, u16, usize],
        [i32, u32, u8],
        [i32, u32, u16],
        [i32, u32, u32],
        [i32, u32, u64],
        [i32, u32, u128],
        [i32, u32, usize],
        [i64, u64, u8],
        [i64, u64, u16],
        [i64, u64, u32],
        [i64, u64, u64],
        [i64, u64, u128],
        [i64, u64, usize],
        [i128, u128, u8],
        [i128, u128, u16],
        [i128, u128, u32],
        [i128, u128, u64],
        [i128, u128, u128],
        [i128, u128, usize],
        [isize, usize, u8],
        [isize, usize, u16],
        [isize, usize, u32],
        [isize, usize, u64],
        [isize, usize, u128],
        [isize, usize, usize]
    );
    register_unsigned_unsigned_demos!(runner, demo_mul_shr_round_assign_unsigned_unsigned);
    register_generic_demos_3_only_1_3_in_key!(
        runner,
        demo_mul_shr_round_assign_signed_unsigned,
        [i8, u8, u8],
        [i8, u8, u16],
        [i8, u8, u32],
        [i8, u8, u64],
        [i8, u8, u128],
        [i8, u8, usize],
        [i16, u16, u8],
        [i16, u16, u16],
        [i16, u16, u32],
        [i16, u16, u64],
        [i16, u16, u128],
        [i16, u16, usize],
        [i32, u32, u8],
        [i32, u32, u16],
        [i32, u32, u32],
        [i32, u32, u64],
        [i32, u32, u128],
        [i32, u32, usize],
        [i64, u64, u8],
        [i64, u64, u16],
        [i64, u64, u32],
        [i64, u64, u64],
        [i64, u64, u128],
        [i64, u64, usize],
        [i128, u128, u8],
        [i128, u128, u16],
        [i128, u128, u32],
        [i128, u128, u64],
        [i128, u128, u128],
        [i128, u128, usize],
        [isize, usize, u8],
        [isize, usize, u16],
        [isize, usize, u32],
        [isize, usize, u64],
        [isize, usize, u128],
        [isize, usize, usize]
    );

    register_unsigned_unsigned_benches!(runner, benchmark_mul_shr_round_unsigned_unsigned);
    register_generic_benches_3_only_1_3_in_key!(
        runner,
        benchmark_mul_shr_round_signed_unsigned,
        [i8, u8, u8],
        [i8, u8, u16],
        [i8, u8, u32],
        [i8, u8, u64],
        [i8, u8, u128],
        [i8, u8, usize],
        [i16, u16, u8],
        [i16, u16, u16],
        [i16, u16, u32],
        [i16, u16, u64],
        [i16, u16, u128],
        [i16, u16, usize],
        [i32, u32, u8],
        [i32, u32, u16],
        [i32, u32, u32],
        [i32, u32, u64],
        [i32, u32, u128],
        [i32, u32, usize],
        [i64, u64, u8],
        [i64, u64, u16],
        [i64, u64, u32],
        [i64, u64, u64],
        [i64, u64, u128],
        [i64, u64, usize],
        [i128, u128, u8],
        [i128, u128, u16],
        [i128, u128, u32],
        [i128, u128, u64],
        [i128, u128, u128],
        [i128, u128, usize],
        [isize, usize, u8],
        [isize, usize, u16],
        [isize, usize, u32],
        [isize, usize, u64],
        [isize, usize, u128],
        [isize, usize, usize]
    );
}

fn demo_mul_shr_round_unsigned_unsigned<
    T: MulShrRound<T, U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    u64: SaturatingFrom<U>,
{
    for (x, y, bits, rm) in unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.mul_shr_round({}, {}, {}) = {:?}",
            x,
            y,
            bits,
            rm,
            x.mul_shr_round(y, bits, rm)
        );
    }
}

fn demo_mul_shr_round_signed_unsigned<
    T: MulShrRound<T, B, Output = T> + PrimitiveSigned + UnsignedAbs<Output = U>,
    U: PrimitiveUnsigned,
    B: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    u64: SaturatingFrom<B>,
{
    for (x, y, bits, rm) in signed_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T, U, B>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).mul_shr_round({}, {}, {}) = {:?}",
            x,
            y,
            bits,
            rm,
            x.mul_shr_round(y, bits, rm)
        );
    }
}

fn demo_mul_shr_round_assign_unsigned_unsigned<
    T: MulShrRoundAssign<T, U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    u64: SaturatingFrom<U>,
{
    for (mut x, y, bits, rm) in
        unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T, U>()
            .get(gm, config)
            .take(limit)
    {
        let old_x = x;
        let o = x.mul_shr_round_assign(y, bits, rm);
        println!("x := {old_x}; x.mul_shr_round_assign({y}, {bits}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_mul_shr_round_assign_signed_unsigned<
    T: MulShrRoundAssign<T, B> + PrimitiveSigned + UnsignedAbs<Output = U>,
    U: PrimitiveUnsigned,
    B: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    u64: SaturatingFrom<B>,
{
    for (mut x, y, bits, rm) in
        signed_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T, U, B>()
            .get(gm, config)
            .take(limit)
    {
        let old_x = x;
        let o = x.mul_shr_round_assign(y, bits, rm);
        println!("x := {old_x}; x.mul_shr_round_assign({y}, {bits}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_mul_shr_round_unsigned_unsigned<
    T: MulShrRound<T, U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    u64: SaturatingFrom<U>,
    usize: ExactFrom<U>,
{
    run_benchmark(
        &format!(
            "{}.mul_shr_round({}, {}, RoundingMode)",
            T::NAME,
            T::NAME,
            U::NAME
        ),
        BenchmarkType::Single,
        unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("bits"),
        &mut [("Malachite", &mut |(x, y, bits, rm)| {
            no_out!(x.mul_shr_round(y, bits, rm))
        })],
    );
}

fn benchmark_mul_shr_round_signed_unsigned<
    T: MulShrRound<T, B, Output = T> + PrimitiveSigned + UnsignedAbs<Output = U>,
    U: PrimitiveUnsigned,
    B: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    u64: SaturatingFrom<B>,
    usize: ExactFrom<B>,
{
    run_benchmark(
        &format!(
            "{}.mul_shr_round({}, {}, RoundingMode)",
            T::NAME,
            T::NAME,
            B::NAME
        ),
        BenchmarkType::Single,
        signed_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T, U, B>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("bits"),
        &mut [("Malachite", &mut |(x, y, bits, rm)| {
            no_out!(x.mul_shr_round(y, bits, rm))
        })],
    );
}
