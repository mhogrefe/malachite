// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::chars::crement::char_to_contiguous_range;
use crate::max;
use crate::num::arithmetic::traits::UnsignedAbs;
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::logic::traits::SignificantBits;
use crate::rational_sequences::RationalSequence;
use std::cmp::{max, min};

pub struct Bucketer<'a, T> {
    pub bucketing_function: &'a dyn Fn(&T) -> usize,
    pub bucketing_label: String,
}

pub fn char_bucketer<'a>() -> Bucketer<'a, char> {
    Bucketer {
        bucketing_function: &|&c| usize::exact_from(char_to_contiguous_range(c)),
        bucketing_label: "char_to_contiguous_range(c)".to_string(),
    }
}

pub fn float_size<T: PrimitiveFloat>(f: T) -> usize {
    if f == T::ZERO || !f.is_finite() || f.is_nan() {
        0
    } else {
        let (m, e) = f.integer_mantissa_and_exponent();
        usize::exact_from(m.significant_bits()) + usize::wrapping_from(e.abs())
    }
}

pub fn primitive_float_bucketer<'a, T: PrimitiveFloat>(var_name: &str) -> Bucketer<'a, T> {
    Bucketer {
        bucketing_function: &|&f| float_size(f),
        bucketing_label: format!("precision({var_name}) + |exponent({var_name})|"),
    }
}

pub fn pair_1_primitive_float_bucketer<'a, T: PrimitiveFloat, U>(
    var_name: &str,
) -> Bucketer<'a, (T, U)> {
    Bucketer {
        bucketing_function: &|&(f, _)| float_size(f),
        bucketing_label: format!("precision({var_name}) + |exponent({var_name})|"),
    }
}

pub fn pair_max_primitive_float_bucketer<'a, T: PrimitiveFloat>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (T, T)> {
    Bucketer {
        bucketing_function: &|&(f, g)| max(float_size(f), float_size(g)),
        bucketing_label: format!(
            "max(precision({x_name}) + |exponent({x_name})|, \
            precision({y_name}) + |exponent({y_name})|)"
        ),
    }
}

pub fn triple_1_primitive_float_bucketer<'a, T: PrimitiveFloat, U, V>(
    var_name: &str,
) -> Bucketer<'a, (T, U, V)> {
    Bucketer {
        bucketing_function: &|&(f, _, _)| float_size(f),
        bucketing_label: format!("precision({var_name}) + |exponent({var_name})|"),
    }
}

pub fn triple_max_primitive_float_bucketer<'a, T: PrimitiveFloat>(
    x_name: &str,
    y_name: &str,
    z_name: &str,
) -> Bucketer<'a, (T, T, T)> {
    Bucketer {
        bucketing_function: &|&(f, g, h)| max!(float_size(f), float_size(g), float_size(h)),
        bucketing_label: format!(
            "max(precision({x_name}) + |exponent({x_name})|, \
            precision({y_name}) + |exponent({y_name})|, \
            precision({z_name}) + |exponent({z_name})|)",
        ),
    }
}

pub fn pair_primitive_float_bit_u64_max_bucketer<'a, T: PrimitiveFloat>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, u64)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(u64::wrapping_from(float_size(*x)), *y))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name})"),
    }
}

pub fn triple_1_2_primitive_float_bit_u64_max_bucketer<'a, T: PrimitiveFloat, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, u64, U)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| {
            usize::exact_from(max(u64::wrapping_from(float_size(*x)), *y))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name})"),
    }
}

pub fn usize_convertible_direct_bucketer<T: Copy>(var_name: &str) -> Bucketer<T>
where
    usize: ExactFrom<T>,
{
    Bucketer {
        bucketing_function: &|&x| usize::exact_from(x),
        bucketing_label: var_name.to_string(),
    }
}

pub fn primitive_int_direct_bucketer<'a, T: PrimitiveInt>() -> Bucketer<'a, T>
where
    usize: ExactFrom<T>,
{
    usize_convertible_direct_bucketer("n")
}

pub fn unsigned_direct_bucketer<'a, T: PrimitiveUnsigned>() -> Bucketer<'a, T>
where
    usize: ExactFrom<T>,
{
    usize_convertible_direct_bucketer("u")
}

pub fn signed_direct_bucketer<'a, T: PrimitiveSigned>() -> Bucketer<'a, T>
where
    usize: ExactFrom<T>,
{
    usize_convertible_direct_bucketer("i")
}

pub fn usize_convertible_pair_max_bucketer<'a, T: Copy + Ord>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, T)>
where
    usize: ExactFrom<T>,
{
    Bucketer {
        bucketing_function: &|&(x, y)| usize::exact_from(max(x, y)),
        bucketing_label: format!("max({x_name}, {y_name})"),
    }
}

pub fn abs_usize_convertible_pair_max_bucketer<'a, T: Copy + UnsignedAbs>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, T)>
where
    usize: ExactFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: Ord,
{
    Bucketer {
        bucketing_function: &|&(x, y)| usize::exact_from(max(x.unsigned_abs(), y.unsigned_abs())),
        bucketing_label: format!("max({x_name}, {y_name})"),
    }
}

pub fn abs_pair_usize_convertible_max_bucketer<'a, T: Copy + UnsignedAbs, U: Copy>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, U)>
where
    usize: ExactFrom<<T as UnsignedAbs>::Output> + ExactFrom<U>,
    <T as UnsignedAbs>::Output: Ord,
{
    Bucketer {
        bucketing_function: &|&(x, y)| {
            max(usize::exact_from(x.unsigned_abs()), usize::exact_from(y))
        },
        bucketing_label: format!("max({x_name}, {y_name})"),
    }
}

pub fn usize_convertible_pair_ratio_bucketer<'a, T: Copy>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, T)>
where
    usize: ExactFrom<T>,
{
    Bucketer {
        bucketing_function: &|&(x, y)| usize::exact_from(x) / usize::exact_from(y),
        bucketing_label: format!("{x_name} / {y_name}"),
    }
}

pub fn signed_abs_bucketer<T: PrimitiveSigned>(var_name: &str) -> Bucketer<T>
where
    usize: ExactFrom<<T as UnsignedAbs>::Output>,
{
    Bucketer {
        bucketing_function: &|&x| usize::exact_from(x.unsigned_abs()),
        bucketing_label: var_name.to_string(),
    }
}

pub fn bit_bucketer<T: Copy + SignificantBits>(var_name: &str) -> Bucketer<T> {
    Bucketer {
        bucketing_function: &|&x| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn primitive_int_bit_bucketer<'a, T: PrimitiveInt>() -> Bucketer<'a, T> {
    bit_bucketer("n")
}

pub fn unsigned_bit_bucketer<'a, T: PrimitiveUnsigned>() -> Bucketer<'a, T> {
    bit_bucketer("u")
}

pub fn signed_bit_bucketer<'a, T: PrimitiveSigned>() -> Bucketer<'a, T> {
    bit_bucketer("i")
}

pub fn ignore_highest_bit_unsigned_bit_bucketer<'a, T: PrimitiveUnsigned>(
    var_name: &str,
) -> Bucketer<'a, T> {
    Bucketer {
        bucketing_function: &|&x| {
            let mut x = x;
            x.clear_bit(T::WIDTH - 1);
            usize::exact_from(x.significant_bits())
        },
        bucketing_label: format!(
            "({} - (1 << {})).significant_bits()",
            var_name,
            T::WIDTH - 1
        ),
    }
}

pub fn string_len_bucketer<'a>() -> Bucketer<'a, String> {
    Bucketer {
        bucketing_function: &String::len,
        bucketing_label: "s.len()".to_string(),
    }
}

pub fn pair_string_max_len_bucketer<'a>(
    s_name: &str,
    t_name: &str,
) -> Bucketer<'a, (String, String)> {
    Bucketer {
        bucketing_function: &|(s, t)| max(s.len(), t.len()),
        bucketing_label: format!("max(s.len({s_name}), t.len({t_name}))"),
    }
}

pub fn pair_1_string_len_bucketer<T>(s_name: &str) -> Bucketer<(String, T)> {
    Bucketer {
        bucketing_function: &|(ref s, _)| s.len(),
        bucketing_label: format!("{s_name}.len()"),
    }
}

pub fn pair_2_string_len_bucketer<T>(s_name: &str) -> Bucketer<(T, String)> {
    Bucketer {
        bucketing_function: &|(_, ref s)| s.len(),
        bucketing_label: format!("{s_name}.len()"),
    }
}

pub fn vec_len_bucketer<'a, T>() -> Bucketer<'a, Vec<T>> {
    Bucketer {
        bucketing_function: &Vec::len,
        bucketing_label: "xs.len()".to_string(),
    }
}

pub fn pair_vec_max_len_bucketer<'a, T, U>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (Vec<T>, Vec<U>)> {
    Bucketer {
        bucketing_function: &|(xs, ys)| max(xs.len(), ys.len()),
        bucketing_label: format!("max({xs_name}.len(), {ys_name}.len())"),
    }
}

pub fn pair_vec_min_len_bucketer<'a, T, U>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (Vec<T>, Vec<U>)> {
    Bucketer {
        bucketing_function: &|(xs, ys)| min(xs.len(), ys.len()),
        bucketing_label: format!("min({xs_name}.len(), {ys_name}.len())"),
    }
}

pub fn pair_max_bit_bucketer<'a, T: Copy + SignificantBits, U: Copy + SignificantBits>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (T, U)> {
    Bucketer {
        bucketing_function: &|&(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn triple_max_bit_bucketer<
    'a,
    T: Copy + SignificantBits,
    U: Copy + SignificantBits,
    V: Copy + SignificantBits,
>(
    x_name: &str,
    y_name: &str,
    z_name: &str,
) -> Bucketer<'a, (T, U, V)> {
    Bucketer {
        bucketing_function: &|&(x, y, z)| {
            usize::exact_from(max!(
                x.significant_bits(),
                y.significant_bits(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.significant_bits(), \
            {z_name}.significant_bits())",
        ),
    }
}

pub fn quadruple_max_bit_bucketer<
    'a,
    T: Copy + SignificantBits,
    U: Copy + SignificantBits,
    V: Copy + SignificantBits,
    W: Copy + SignificantBits,
>(
    x_name: &str,
    y_name: &str,
    z_name: &str,
    w_name: &str,
) -> Bucketer<'a, (T, U, V, W)> {
    Bucketer {
        bucketing_function: &|&(x, y, z, w)| {
            usize::exact_from(max!(
                x.significant_bits(),
                y.significant_bits(),
                z.significant_bits(),
                w.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.significant_bits(), \
            {z_name}.significant_bits(), {w_name}.significant_bits())",
        ),
    }
}

pub fn sextuple_max_bit_bucketer<
    'a,
    T: Copy + SignificantBits,
    U: Copy + SignificantBits,
    V: Copy + SignificantBits,
    W: Copy + SignificantBits,
    X: Copy + SignificantBits,
    Y: Copy + SignificantBits,
>(
    x_name: &str,
    y_name: &str,
    z_name: &str,
    w_name: &str,
    v_name: &str,
    u_name: &str,
) -> Bucketer<'a, (T, U, V, W, X, Y)> {
    Bucketer {
        bucketing_function: &|&(x, y, z, w, v, u)| {
            usize::exact_from(max!(
                x.significant_bits(),
                y.significant_bits(),
                z.significant_bits(),
                w.significant_bits(),
                v.significant_bits(),
                u.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.significant_bits(), \
            {z_name}.significant_bits(), {w_name}.significant_bits(), \
            {v_name}.significant_bits(), {u_name}.significant_bits())",
        ),
    }
}

#[allow(clippy::type_complexity)]
pub fn octuple_max_bit_bucketer<
    'a,
    T: Copy + SignificantBits,
    U: Copy + SignificantBits,
    V: Copy + SignificantBits,
    W: Copy + SignificantBits,
    X: Copy + SignificantBits,
    Y: Copy + SignificantBits,
    Z: Copy + SignificantBits,
    A: Copy + SignificantBits,
>(
    x_name: &str,
    y_name: &str,
    z_name: &str,
    w_name: &str,
    v_name: &str,
    u_name: &str,
    t_name: &str,
    s_name: &str,
) -> Bucketer<'a, (T, U, V, W, X, Y, Z, A)> {
    Bucketer {
        bucketing_function: &|&(x, y, z, w, v, u, t, s)| {
            usize::exact_from(max!(
                x.significant_bits(),
                y.significant_bits(),
                z.significant_bits(),
                w.significant_bits(),
                v.significant_bits(),
                u.significant_bits(),
                t.significant_bits(),
                s.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.significant_bits(), \
            {z_name}.significant_bits(), {w_name}.significant_bits(), \
            {v_name}.significant_bits(), {u_name}.significant_bits(), \
            {t_name}.significant_bits(), {s_name}.significant_bits())",
        ),
    }
}

pub fn triple_1_2_max_bit_bucketer<'a, T: Copy + SignificantBits, U: Copy + SignificantBits, V>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (T, U, V)> {
    Bucketer {
        bucketing_function: &|&(x, y, ref _z)| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn triple_2_3_product_bit_bucketer<
    'a,
    T,
    U: Copy + SignificantBits,
    V: Copy + SignificantBits,
>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (T, U, V)> {
    Bucketer {
        bucketing_function: &|&(_, y, z)| {
            usize::exact_from(
                y.significant_bits()
                    .checked_mul(z.significant_bits())
                    .unwrap(),
            )
        },
        bucketing_label: format!("{x_name}.significant_bits() * {y_name}.significant_bits()"),
    }
}

pub fn pair_1_bucketer<T: Copy, U>(x_name: &str) -> Bucketer<(T, U)>
where
    usize: ExactFrom<T>,
{
    Bucketer {
        bucketing_function: &|&(x, _)| usize::exact_from(x),
        bucketing_label: x_name.to_string(),
    }
}

pub fn pair_2_bucketer<T, U: Copy>(y_name: &str) -> Bucketer<(T, U)>
where
    usize: ExactFrom<U>,
{
    Bucketer {
        bucketing_function: &|&(_, y)| usize::exact_from(y),
        bucketing_label: y_name.to_string(),
    }
}

pub fn pair_2_pair_2_bucketer<T, U, V: Copy>(y_name: &str) -> Bucketer<(T, (U, V))>
where
    usize: ExactFrom<V>,
{
    Bucketer {
        bucketing_function: &|&(_, (_, y))| usize::exact_from(y),
        bucketing_label: y_name.to_string(),
    }
}

pub fn pair_2_unsigned_abs_bucketer<T, U: Copy + UnsignedAbs>(y_name: &str) -> Bucketer<(T, U)>
where
    usize: ExactFrom<<U as UnsignedAbs>::Output>,
{
    Bucketer {
        bucketing_function: &|&(_, y)| usize::exact_from(y.unsigned_abs()),
        bucketing_label: y_name.to_string(),
    }
}

pub fn triple_2_bucketer<T, U: Copy, V>(y_name: &str) -> Bucketer<(T, U, V)>
where
    usize: ExactFrom<U>,
{
    Bucketer {
        bucketing_function: &|&(_, y, _)| usize::exact_from(y),
        bucketing_label: y_name.to_string(),
    }
}

pub fn triple_3_bucketer<T, U, V: Copy>(z_name: &str) -> Bucketer<(T, U, V)>
where
    usize: ExactFrom<V>,
{
    Bucketer {
        bucketing_function: &|&(_, _, z)| usize::exact_from(z),
        bucketing_label: z_name.to_string(),
    }
}

pub fn triple_2_unsigned_abs_bucketer<T, U: Copy + UnsignedAbs, V>(
    y_name: &str,
) -> Bucketer<(T, U, V)>
where
    usize: ExactFrom<<U as UnsignedAbs>::Output>,
{
    Bucketer {
        bucketing_function: &|&(_, y, _)| usize::exact_from(y.unsigned_abs()),
        bucketing_label: y_name.to_string(),
    }
}

pub fn pair_2_triple_2_bucketer<T, U, V: Copy, W>(y_name: &str) -> Bucketer<(T, (U, V, W))>
where
    usize: ExactFrom<V>,
{
    Bucketer {
        bucketing_function: &|&(_, (_, y, _))| usize::exact_from(y),
        bucketing_label: y_name.to_string(),
    }
}

pub fn triple_3_pair_2_bucketer<T, U, V, W: Copy>(y_name: &str) -> Bucketer<(T, U, (V, W))>
where
    usize: ExactFrom<W>,
{
    Bucketer {
        bucketing_function: &|&(_, _, (_, y))| usize::exact_from(y),
        bucketing_label: y_name.to_string(),
    }
}

pub fn pair_1_bit_bucketer<T: Copy + SignificantBits, U>(x_name: &str) -> Bucketer<(T, U)> {
    Bucketer {
        bucketing_function: &|&(x, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{x_name}.significant_bits()"),
    }
}

pub fn pair_2_bit_bucketer<T, U: Copy + SignificantBits>(x_name: &str) -> Bucketer<(T, U)> {
    Bucketer {
        bucketing_function: &|&(_, x)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{x_name}.significant_bits()"),
    }
}

pub fn triple_1_bit_bucketer<T: Copy + SignificantBits, U, V>(x_name: &str) -> Bucketer<(T, U, V)> {
    Bucketer {
        bucketing_function: &|&(x, _, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{x_name}.significant_bits()"),
    }
}

pub fn triple_3_bit_bucketer<T, U, V: Copy + SignificantBits>(z_name: &str) -> Bucketer<(T, U, V)> {
    Bucketer {
        bucketing_function: &|&(_, _, z)| usize::exact_from(z.significant_bits()),
        bucketing_label: format!("{z_name}.significant_bits()"),
    }
}

pub fn quadruple_1_2_bit_bucketer<T: PrimitiveUnsigned, U, V>(
    combined_name: &str,
) -> Bucketer<(T, T, U, V)> {
    Bucketer {
        bucketing_function: &|&(x_1, x_0, _, _)| {
            usize::exact_from(if x_1 == T::ZERO {
                x_0.significant_bits()
            } else {
                x_1.significant_bits() + T::WIDTH
            })
        },
        bucketing_label: format!("{combined_name}.significant_bits()"),
    }
}

pub fn quadruple_3_bucketer<T, U, V: Copy, W>(z_name: &str) -> Bucketer<(T, U, V, W)>
where
    usize: ExactFrom<V>,
{
    Bucketer {
        bucketing_function: &|&(_, _, z, _)| usize::exact_from(z),
        bucketing_label: z_name.to_string(),
    }
}

pub fn pair_primitive_int_bit_u64_max_bucketer<'a, T: PrimitiveInt>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, u64)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.significant_bits(), *y)),
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name})"),
    }
}

pub fn triple_1_2_primitive_int_bit_u64_max_bucketer<'a, T: PrimitiveInt, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, u64, U)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| usize::exact_from(max(x.significant_bits(), *y)),
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name})"),
    }
}

pub fn pair_primitive_int_bit_i64_max_bucketer<'a, T: PrimitiveInt>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, i64)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.unsigned_abs()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.unsigned_abs())"),
    }
}

pub fn triple_primitive_int_bit_i64_u64_max_bucketer<'a, T: PrimitiveInt>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (T, i64, u64)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(x.significant_bits(), y.unsigned_abs(), *z))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.unsigned_abs(), {z_name})"
        ),
    }
}

pub fn quadruple_1_2_3_primitive_int_bit_i64_u64_max_bucketer<'a, T: PrimitiveInt, U>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (T, i64, u64, U)> {
    Bucketer {
        bucketing_function: &|(x, y, z, _)| {
            usize::exact_from(max!(x.significant_bits(), y.unsigned_abs(), *z))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.unsigned_abs(), {z_name})"
        ),
    }
}

pub fn pair_1_vec_len_bucketer<T, U>(xs_name: &str) -> Bucketer<(Vec<T>, U)> {
    Bucketer {
        bucketing_function: &|(ref xs, _)| xs.len(),
        bucketing_label: format!("{xs_name}.len()"),
    }
}

pub fn pair_1_vec_len_sub_1_bucketer<T, U>(xs_name: &str) -> Bucketer<(Vec<T>, U)> {
    Bucketer {
        bucketing_function: &|(ref xs, _)| xs.len() - 1,
        bucketing_label: format!("{xs_name}.len() - 1"),
    }
}

pub fn pair_2_vec_len_bucketer<T, U>(ys_name: &str) -> Bucketer<(T, Vec<U>)> {
    Bucketer {
        bucketing_function: &|(_, ref ys)| ys.len(),
        bucketing_label: format!("{ys_name}.len()"),
    }
}

pub fn triple_1_vec_len_bucketer<T, U, V>(xs_name: &str) -> Bucketer<(Vec<T>, U, V)> {
    Bucketer {
        bucketing_function: &|(ref xs, _, _)| xs.len(),
        bucketing_label: format!("{xs_name}.len()"),
    }
}

pub fn triple_3_vec_len_bucketer<T, U, V>(xs_name: &str) -> Bucketer<(T, U, Vec<V>)> {
    Bucketer {
        bucketing_function: &|(_, _, ref xs)| xs.len(),
        bucketing_label: format!("{xs_name}.len()"),
    }
}

pub fn triple_2_vec_len_bucketer<T, U, V>(xs_name: &str) -> Bucketer<(T, Vec<U>, V)> {
    Bucketer {
        bucketing_function: &|(_, ref xs, _)| xs.len(),
        bucketing_label: format!("{xs_name}.len()"),
    }
}

pub fn triple_vec_max_len_bucketer<'a, T, U, V>(
    xs_name: &str,
    ys_name: &str,
    zs_name: &str,
) -> Bucketer<'a, (Vec<T>, Vec<U>, Vec<V>)> {
    Bucketer {
        bucketing_function: &|(xs, ys, zs)| max!(xs.len(), ys.len(), zs.len()),
        bucketing_label: format!("max({xs_name}.len(), {ys_name}.len(), {zs_name}.len())"),
    }
}

pub fn triple_1_2_vec_max_len_bucketer<'a, T, U, V>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (Vec<T>, Vec<U>, V)> {
    Bucketer {
        bucketing_function: &|(xs, ys, _)| max(xs.len(), ys.len()),
        bucketing_label: format!("max({xs_name}.len(), {ys_name}.len())"),
    }
}

pub fn triple_2_3_vec_max_len_bucketer<'a, T, U, V>(
    ys_name: &str,
    zs_name: &str,
) -> Bucketer<'a, (T, Vec<U>, Vec<V>)> {
    Bucketer {
        bucketing_function: &|(_, xs, ys)| max(xs.len(), ys.len()),
        bucketing_label: format!("max({ys_name}.len(), {zs_name}.len())"),
    }
}

pub fn triple_1_2_vec_min_len_bucketer<'a, T, U, V>(
    ys_name: &str,
    zs_name: &str,
) -> Bucketer<'a, (Vec<T>, Vec<U>, V)> {
    Bucketer {
        bucketing_function: &|(xs, ys, _)| min(xs.len(), ys.len()),
        bucketing_label: format!("min({ys_name}.len(), {zs_name}.len())"),
    }
}

pub fn triple_2_3_vec_min_len_bucketer<'a, T, U, V>(
    ys_name: &str,
    zs_name: &str,
) -> Bucketer<'a, (T, Vec<U>, Vec<V>)> {
    Bucketer {
        bucketing_function: &|(_, xs, ys)| min(xs.len(), ys.len()),
        bucketing_label: format!("min({ys_name}.len(), {zs_name}.len())"),
    }
}

pub fn quadruple_2_vec_len_bucketer<T, U, V, W>(xs_name: &str) -> Bucketer<(T, Vec<U>, V, W)> {
    Bucketer {
        bucketing_function: &|(_, ref xs, _, _)| xs.len(),
        bucketing_label: format!("{xs_name}.len()"),
    }
}

pub fn quadruple_3_vec_len_bucketer<T, U, V, W>(xs_name: &str) -> Bucketer<(T, U, Vec<V>, W)> {
    Bucketer {
        bucketing_function: &|(_, _, ref xs, _)| xs.len(),
        bucketing_label: format!("{xs_name}.len()"),
    }
}

pub fn quadruple_4_vec_len_bucketer<T, U, V, W>(xs_name: &str) -> Bucketer<(T, U, V, Vec<W>)> {
    Bucketer {
        bucketing_function: &|(_, _, _, ref xs)| xs.len(),
        bucketing_label: format!("{xs_name}.len()"),
    }
}

pub fn quadruple_1_3_vec_max_len_bucketer<'a, T, U, V, W>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (T, Vec<U>, V, Vec<W>)> {
    Bucketer {
        bucketing_function: &|(_, xs, _, ys)| max(xs.len(), ys.len()),
        bucketing_label: format!("max({xs_name}.len(), {ys_name}.len())"),
    }
}

pub fn quintuple_1_vec_len_bucketer<T, U, V, W, X>(
    xs_name: &str,
) -> Bucketer<(Vec<T>, U, V, W, X)> {
    Bucketer {
        bucketing_function: &|(ref xs, _, _, _, _)| xs.len(),
        bucketing_label: format!("{xs_name}.len()"),
    }
}

pub fn pair_sum_vec_len_bucketer<'a, T, U>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (Vec<T>, Vec<U>)> {
    Bucketer {
        bucketing_function: &|(xs, ys)| xs.len() + ys.len(),
        bucketing_label: format!("{xs_name}.len() + {ys_name}.len()"),
    }
}

pub fn triple_2_3_sum_vec_len_bucketer<'a, T, U, V>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (T, Vec<U>, Vec<V>)> {
    Bucketer {
        bucketing_function: &|(_, xs, ys)| xs.len() + ys.len(),
        bucketing_label: format!("{xs_name}.len() + {ys_name}.len()"),
    }
}

pub fn triple_2_3_diff_vec_len_bucketer<'a, T, U, V>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (T, Vec<U>, Vec<V>)> {
    Bucketer {
        bucketing_function: &|(_, xs, ys)| xs.len() - ys.len(),
        bucketing_label: format!("{xs_name}.len() - {ys_name}.len()"),
    }
}

pub fn quadruple_2_3_diff_vec_len_bucketer<'a, T, U, V, W>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (T, Vec<U>, Vec<V>, W)> {
    Bucketer {
        bucketing_function: &|(_, xs, ys, _)| xs.len() - ys.len(),
        bucketing_label: format!("{xs_name}.len() - {ys_name}.len()"),
    }
}

pub fn get_bits_bucketer<T>() -> Bucketer<'static, (T, u64, u64)> {
    Bucketer {
        bucketing_function: &|&(_, start, end)| usize::exact_from(end - start),
        bucketing_label: "end - start".to_string(),
    }
}

pub fn assign_bits_bucketer<T, U>() -> Bucketer<'static, (T, u64, u64, U)> {
    Bucketer {
        bucketing_function: &|&(_, start, end, _)| usize::exact_from(end - start),
        bucketing_label: "end - start".to_string(),
    }
}

pub fn pair_1_vec_len_times_pair_2_bits_bucketer<'a, T, U: PrimitiveUnsigned>(
    xs_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Vec<T>, U)> {
    Bucketer {
        bucketing_function: &|(ref xs, ref y)| {
            xs.len()
                .checked_mul(usize::exact_from(y.significant_bits()))
                .unwrap()
        },
        bucketing_label: format!("{xs_name}.len() * {y_name}.significant_bits()"),
    }
}

pub fn pair_product_vec_len_bucketer<'a, T, U>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (Vec<T>, Vec<U>)> {
    Bucketer {
        bucketing_function: &|(xs, ys)| xs.len().checked_mul(ys.len()).unwrap(),
        bucketing_label: format!("{xs_name}.len() * {ys_name}.len()"),
    }
}

pub fn triple_1_2_product_vec_len_bucketer<'a, T, U, V>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (Vec<T>, Vec<U>, V)> {
    Bucketer {
        bucketing_function: &|(xs, ys, _)| xs.len().checked_mul(ys.len()).unwrap(),
        bucketing_label: format!("{xs_name}.len() * {ys_name}.len()"),
    }
}

pub fn triple_2_bits_times_triple_3_bucketer<'a, T, U, V: Copy>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, U, V)>
where
    usize: ExactFrom<V>,
    for<'b> &'b U: SignificantBits,
{
    Bucketer {
        bucketing_function: &|(_, ref x, y)| {
            let x_bits: usize = ExactFrom::<u64>::exact_from(x.significant_bits());
            x_bits.checked_mul(ExactFrom::<V>::exact_from(*y)).unwrap()
        },
        bucketing_label: format!("{x_name}.significant_bits() * {y_name}"),
    }
}

pub fn pair_1_vec_len_times_pair_2_bucketer<'a, T, U: Copy>(
    xs_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Vec<T>, U)>
where
    usize: ExactFrom<U>,
{
    Bucketer {
        bucketing_function: &|(ref xs, y)| xs.len().checked_mul(usize::exact_from(*y)).unwrap(),
        bucketing_label: format!("{xs_name}.len() * {y_name}"),
    }
}

pub fn pair_1_bits_times_pair_2_bucketer<'a, T, U: Copy>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, U)>
where
    usize: ExactFrom<U>,
    for<'b> &'b T: SignificantBits,
{
    Bucketer {
        bucketing_function: &|(ref x, y)| {
            let x_bits: usize = ExactFrom::<u64>::exact_from(x.significant_bits());
            x_bits.checked_mul(ExactFrom::<U>::exact_from(*y)).unwrap()
        },
        bucketing_label: format!("{x_name}.significant_bits() * {y_name}"),
    }
}

pub fn triple_3_pair_1_bits_times_pair_2_bucketer<'a, T, U, V, W: Copy>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, U, (V, W))>
where
    usize: ExactFrom<W>,
    for<'b> &'b V: SignificantBits,
{
    Bucketer {
        bucketing_function: &|(_, _, (ref x, y))| {
            let x_bits: usize = ExactFrom::<u64>::exact_from(x.significant_bits());
            x_bits.checked_mul(ExactFrom::<W>::exact_from(*y)).unwrap()
        },
        bucketing_label: format!("{x_name}.significant_bits() * {y_name}"),
    }
}

pub fn rational_sequence_len_bucketer<'a, T: Eq>(
    xs_name: &str,
) -> Bucketer<'a, RationalSequence<T>> {
    Bucketer {
        bucketing_function: &RationalSequence::component_len,
        bucketing_label: format!("{xs_name}.component_len()"),
    }
}

pub fn pair_rational_sequence_max_len_bucketer<'a, T: Eq, U: Eq>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (RationalSequence<T>, RationalSequence<U>)> {
    Bucketer {
        bucketing_function: &|(xs, ys)| max(xs.component_len(), ys.component_len()),
        bucketing_label: format!("max({xs_name}.component_len(), {ys_name}.component_len())"),
    }
}

pub fn pair_1_rational_sequence_len_bucketer<'a, T: Eq, U>(
    xs_name: &str,
) -> Bucketer<'a, (RationalSequence<T>, U)> {
    Bucketer {
        bucketing_function: &|(xs, _)| xs.component_len(),
        bucketing_label: format!("{xs_name}.component_len()"),
    }
}

pub fn quadruple_1_rational_sequence_len_bucketer<'a, T: Eq, U, V, W>(
    xs_name: &str,
) -> Bucketer<'a, (RationalSequence<T>, U, V, W)> {
    Bucketer {
        bucketing_function: &|(xs, _, _, _)| xs.component_len(),
        bucketing_label: format!("{xs_name}.component_len()"),
    }
}
