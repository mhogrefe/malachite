// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::bench::bucketers::{Bucketer, float_size};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use std::cmp::max;

pub fn big_exponent_float_complexity_bucketer(var_name: &str) -> Bucketer<'_, BigExponentFloat> {
    Bucketer {
        bucketing_function: &|x| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_1_big_exponent_float_complexity_bucketer<T>(
    var_name: &str,
) -> Bucketer<'_, (BigExponentFloat, T)> {
    Bucketer {
        bucketing_function: &|(x, _)| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_2_big_exponent_float_complexity_bucketer<T>(
    var_name: &str,
) -> Bucketer<'_, (T, BigExponentFloat)> {
    Bucketer {
        bucketing_function: &|(_, x)| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn max_pair_1_big_exponent_float_complexity_pair_2_bucketer<'a>(
    x_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, u64)> {
    Bucketer {
        bucketing_function: &|(x, p)| usize::exact_from(max(x.complexity(), *p)),
        bucketing_label: format!("max({x_name}.complexity(), {p_name})"),
    }
}

pub fn max_triple_1_big_exponent_float_complexity_triple_2_bucketer<'a, T>(
    x_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, p, _)| usize::exact_from(max(x.complexity(), *p)),
        bucketing_label: format!("max({x_name}.complexity(), {p_name})"),
    }
}

pub fn pair_2_max_triple_1_big_exponent_float_complexity_triple_2_bucketer<'a, T, U>(
    x_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (T, (BigExponentFloat, u64, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, p, _))| usize::exact_from(max(x.complexity(), *p)),
        bucketing_label: format!("max({x_name}.complexity(), {p_name})"),
    }
}

pub fn max_triple_1_big_exponent_float_complexity_triple_3_bucketer<'a, T>(
    x_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, T, u64)> {
    Bucketer {
        bucketing_function: &|(x, _, p)| usize::exact_from(max(x.complexity(), *p)),
        bucketing_label: format!("max({x_name}.complexity(), {p_name})"),
    }
}

pub fn max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer<'a, T, U>(
    x_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, T, u64, U)> {
    Bucketer {
        bucketing_function: &|(x, _, p, _)| usize::exact_from(max(x.complexity(), *p)),
        bucketing_label: format!("max({x_name}.complexity(), {p_name})"),
    }
}

pub fn pair_2_max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer<'a, T, U, V>(
    x_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (T, (BigExponentFloat, U, u64, V))> {
    Bucketer {
        bucketing_function: &|(_, (x, _, p, _))| usize::exact_from(max(x.complexity(), *p)),
        bucketing_label: format!("max({x_name}.complexity(), {p_name})"),
    }
}

pub fn pair_big_exponent_float_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, BigExponentFloat)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn pair_2_pair_big_exponent_float_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (BigExponentFloat, BigExponentFloat))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn pair_big_exponent_float_float_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, Float)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn pair_big_exponent_float_natural_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, Natural)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_big_exponent_float_integer_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, Integer)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_big_exponent_float_rational_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, Rational)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn triple_1_2_3_big_exponent_float_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, BigExponentFloat, u64)> {
    Bucketer {
        bucketing_function: &|(x, y, p)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), *p))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity(), {p_name})"),
    }
}

pub fn quadruple_1_2_3_big_exponent_float_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, BigExponentFloat, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, y, p, _)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), *p))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity(), {p_name})"),
    }
}

pub fn pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (T, (BigExponentFloat, BigExponentFloat, u64, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, p, _))| {
            usize::exact_from(max!(x.complexity(), y.complexity(), *p))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity(), {p_name})"),
    }
}

pub fn triple_1_2_3_big_exponent_float_rational_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, Rational, u64)> {
    Bucketer {
        bucketing_function: &|(x, y, p)| {
            usize::exact_from(max!(x.complexity(), y.significant_bits(), *p))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.significant_bits(), {p_name})"
        ),
    }
}

pub fn quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, Rational, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, y, p, _)| {
            usize::exact_from(max!(x.complexity(), y.significant_bits(), *p))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.significant_bits(), {p_name})"
        ),
    }
}

pub fn pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (T, (BigExponentFloat, Rational, u64, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, p, _))| {
            usize::exact_from(max!(x.complexity(), y.significant_bits(), *p))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.significant_bits(), {p_name})"
        ),
    }
}

pub fn pair_1_float_complexity_bucketer<T>(var_name: &str) -> Bucketer<'_, (Float, T)> {
    Bucketer {
        bucketing_function: &|(x, _)| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_2_pair_1_float_complexity_bucketer<T, U>(
    var_name: &str,
) -> Bucketer<'_, (T, (Float, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, _))| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_2_float_complexity_bucketer<T>(var_name: &str) -> Bucketer<'_, (T, Float)> {
    Bucketer {
        bucketing_function: &|(_, x)| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn triple_1_float_complexity_bucketer<T, U>(var_name: &str) -> Bucketer<'_, (Float, T, U)> {
    Bucketer {
        bucketing_function: &|(x, _, _)| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_2_triple_1_float_complexity_bucketer<T, U, V>(
    var_name: &str,
) -> Bucketer<'_, (V, (Float, T, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, _, _))| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn quadruple_1_float_complexity_bucketer<T, U, V>(
    var_name: &str,
) -> Bucketer<'_, (Float, T, U, V)> {
    Bucketer {
        bucketing_function: &|(x, _, _, _)| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_2_quadruple_1_float_complexity_bucketer<T, U, V, W>(
    var_name: &str,
) -> Bucketer<'_, (W, (Float, T, U, V))> {
    Bucketer {
        bucketing_function: &|(_, (x, _, _, _))| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn float_complexity_bucketer(var_name: &str) -> Bucketer<'_, Float> {
    Bucketer {
        bucketing_function: &|x| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn vec_float_sum_complexity_bucketer<'a>(xs_name: &'a str) -> Bucketer<'a, Vec<Float>> {
    Bucketer {
        bucketing_function: &|xs| usize::exact_from(xs.iter().map(Float::complexity).sum::<u64>()),
        bucketing_label: format!("{xs_name}.map(|x| x.complexity()).sum()"),
    }
}

pub fn vec_pair_float_sum_complexity_bucketer<'a>(
    xs_name: &'a str,
    ys_name: &'a str,
) -> Bucketer<'a, (Vec<Float>, Vec<Float>)> {
    Bucketer {
        bucketing_function: &|(xs, ys)| {
            usize::exact_from(
                xs.iter()
                    .chain(ys.iter())
                    .map(Float::complexity)
                    .sum::<u64>(),
            )
        },
        bucketing_label: format!("{xs_name}.chain({ys_name}).map(|x| x.complexity()).sum()"),
    }
}

pub fn pair_2_vec_pair_float_sum_complexity_bucketer<'a, T>(
    xs_name: &'a str,
    ys_name: &'a str,
) -> Bucketer<'a, (T, (Vec<Float>, Vec<Float>))> {
    Bucketer {
        bucketing_function: &|(_, (xs, ys))| {
            usize::exact_from(
                xs.iter()
                    .chain(ys.iter())
                    .map(Float::complexity)
                    .sum::<u64>(),
            )
        },
        bucketing_label: format!("{xs_name}.chain({ys_name}).map(|x| x.complexity()).sum()"),
    }
}

pub fn quadruple_1_2_vec_float_sum_complexity_bucketer<'a, T, U>(
    xs_name: &'a str,
    ys_name: &'a str,
) -> Bucketer<'a, (Vec<Float>, Vec<Float>, T, U)> {
    Bucketer {
        bucketing_function: &|(xs, ys, _, _)| {
            usize::exact_from(
                xs.iter()
                    .chain(ys.iter())
                    .map(Float::complexity)
                    .sum::<u64>(),
            )
        },
        bucketing_label: format!("{xs_name}.chain({ys_name}).map(|x| x.complexity()).sum()"),
    }
}

pub fn pair_2_vec_float_sum_complexity_bucketer<'a, T>(
    xs_name: &'a str,
) -> Bucketer<'a, (T, Vec<Float>)> {
    Bucketer {
        bucketing_function: &|(_, xs)| {
            usize::exact_from(xs.iter().map(Float::complexity).sum::<u64>())
        },
        bucketing_label: format!("{xs_name}.map(|x| x.complexity()).sum()"),
    }
}

pub fn triple_1_vec_float_sum_complexity_bucketer<'a, T, U>(
    xs_name: &'a str,
) -> Bucketer<'a, (Vec<Float>, T, U)> {
    Bucketer {
        bucketing_function: &|(xs, _, _)| {
            usize::exact_from(xs.iter().map(Float::complexity).sum::<u64>())
        },
        bucketing_label: format!("{xs_name}.map(|x| x.complexity()).sum()"),
    }
}

pub fn pair_2_triple_1_vec_float_sum_complexity_bucketer<'a, T, U, V>(
    xs_name: &'a str,
) -> Bucketer<'a, (V, (Vec<Float>, T, U))> {
    Bucketer {
        bucketing_function: &|(_, (xs, _, _))| {
            usize::exact_from(xs.iter().map(Float::complexity).sum::<u64>())
        },
        bucketing_label: format!("{xs_name}.map(|x| x.complexity()).sum()"),
    }
}

pub fn pair_1_vec_float_sum_complexity_bucketer<'a, T>(
    xs_name: &'a str,
) -> Bucketer<'a, (Vec<Float>, T)> {
    Bucketer {
        bucketing_function: &|(xs, _)| {
            usize::exact_from(xs.iter().map(Float::complexity).sum::<u64>())
        },
        bucketing_label: format!("{xs_name}.map(|x| x.complexity()).sum()"),
    }
}

pub fn pair_float_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Float)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn triple_1_2_float_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn pair_2_triple_1_2_float_max_complexity_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (U, (Float, Float, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, _))| {
            usize::exact_from(max(x.complexity(), y.complexity()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn triple_1_2_float_rational_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Rational, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn triple_1_2_float_float_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn pair_2_triple_1_2_float_rational_max_complexity_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (U, (Float, Rational, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, _))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, T)> {
    Bucketer {
        bucketing_function: &|(x, z)| usize::exact_from(max!(x.complexity(), z.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits)"),
    }
}

pub fn pair_2_pair_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (U, (Float, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, z))| {
            usize::exact_from(max!(x.complexity(), z.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits)"),
    }
}

pub fn triple_float_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.significant_bits()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.significant_bits)"
        ),
    }
}

pub fn pair_2_triple_float_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt, U>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (U, (Float, Float, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z))| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.significant_bits()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.significant_bits)"
        ),
    }
}

pub fn triple_1_2_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, T, U)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| {
            usize::exact_from(max!(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt, U, V>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (V, (Float, T, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, _))| {
            usize::exact_from(max!(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn quadruple_1_3_float_primitive_int_max_complexity_bucketer<'a, T, U: PrimitiveInt>(
    x_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, T, U, RoundingMode)> {
    Bucketer {
        bucketing_function: &|(x, _, z, _)| {
            usize::exact_from(max!(x.complexity(), z.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {z_name}.significant_bits())"),
    }
}

pub fn pair_2_quadruple_1_3_float_primitive_int_max_complexity_bucketer<
    'a,
    T,
    U: PrimitiveInt,
    V,
>(
    x_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (V, (Float, T, U, RoundingMode))> {
    Bucketer {
        bucketing_function: &|(_, (x, _, z, _))| {
            usize::exact_from(max!(x.complexity(), z.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {z_name}.significant_bits())"),
    }
}

pub fn quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt, U>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, Float, T, U)> {
    Bucketer {
        bucketing_function: &|(x, y, z, _)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.significant_bits()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.significant_bits())"
        ),
    }
}

pub fn triple_1_2_3_float_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, Float, Float)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.complexity()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.complexity())"
        ),
    }
}

pub fn pair_2_triple_1_2_3_float_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (T, (Float, Float, Float))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z))| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.complexity()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.complexity())"
        ),
    }
}

pub fn quadruple_1_2_3_float_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, Float, Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y, z, _)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.complexity()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.complexity())"
        ),
    }
}

pub fn pair_2_quadruple_1_2_3_float_max_complexity_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (U, (Float, Float, Float, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z, _))| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.complexity()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.complexity())"
        ),
    }
}

pub fn quadruple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
    w_name: &'a str,
) -> Bucketer<'a, (Float, Float, Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y, z, w)| {
            usize::exact_from(max!(
                x.complexity(),
                y.complexity(),
                z.complexity(),
                w.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.complexity(), \
            {w_name}.significant_bits())"
        ),
    }
}

pub fn pair_2_quadruple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
    w_name: &'a str,
) -> Bucketer<'a, (U, (Float, Float, Float, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z, w))| {
            usize::exact_from(max!(
                x.complexity(),
                y.complexity(),
                z.complexity(),
                w.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.complexity(), \
            {w_name}.significant_bits())"
        ),
    }
}

pub fn sextuple_1_2_3_4_float_float_float_rational_max_complexity_bucketer<'a, T, U>(
    a_name: &'a str,
    b_name: &'a str,
    c_name: &'a str,
    d_name: &'a str,
) -> Bucketer<'a, (Float, Float, Float, Rational, T, U)> {
    Bucketer {
        bucketing_function: &|(a, b, c, d, _, _)| {
            usize::exact_from(max!(
                a.complexity(),
                b.complexity(),
                c.complexity(),
                d.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({a_name}.complexity(), {b_name}.complexity(), {c_name}.complexity(), \
            {d_name}.significant_bits())"
        ),
    }
}

pub fn sextuple_1_2_3_4_float_max_complexity_bucketer<'a, T, U>(
    a_name: &'a str,
    b_name: &'a str,
    c_name: &'a str,
    d_name: &'a str,
) -> Bucketer<'a, (Float, Float, Float, Float, T, U)> {
    Bucketer {
        bucketing_function: &|(a, b, c, d, _, _)| {
            usize::exact_from(max!(
                a.complexity(),
                b.complexity(),
                c.complexity(),
                d.complexity()
            ))
        },
        bucketing_label: format!(
            "max({a_name}.complexity(), {b_name}.complexity(), {c_name}.complexity(), \
            {d_name}.complexity())"
        ),
    }
}

pub fn pair_2_sextuple_1_2_3_4_float_max_complexity_bucketer<'a, T, U, V>(
    a_name: &'a str,
    b_name: &'a str,
    c_name: &'a str,
    d_name: &'a str,
) -> Bucketer<'a, (V, (Float, Float, Float, Float, T, U))> {
    Bucketer {
        bucketing_function: &|(_, (a, b, c, d, _, _))| {
            usize::exact_from(max!(
                a.complexity(),
                b.complexity(),
                c.complexity(),
                d.complexity()
            ))
        },
        bucketing_label: format!(
            "max({a_name}.complexity(), {b_name}.complexity(), {c_name}.complexity(), \
            {d_name}.complexity())"
        ),
    }
}

pub fn quintuple_1_2_3_float_float_rational_max_complexity_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, Float, Rational, T, U)> {
    Bucketer {
        bucketing_function: &|(x, y, z, _, _)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.significant_bits()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.significant_bits())"
        ),
    }
}

pub fn quintuple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
    w_name: &'a str,
) -> Bucketer<'a, (Float, Float, Float, T, U)> {
    Bucketer {
        bucketing_function: &|(x, y, z, w, _)| {
            usize::exact_from(max!(
                x.complexity(),
                y.complexity(),
                z.complexity(),
                w.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.complexity(), \
            {w_name}.significant_bits())"
        ),
    }
}

pub fn pair_2_quintuple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
    V,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
    w_name: &'a str,
) -> Bucketer<'a, (V, (Float, Float, Float, T, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z, w, _))| {
            usize::exact_from(max!(
                x.complexity(),
                y.complexity(),
                z.complexity(),
                w.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.complexity(), \
            {w_name}.significant_bits())"
        ),
    }
}

pub fn pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
    V,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (V, (Float, Float, T, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z, _))| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.significant_bits()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.significant_bits())"
        ),
    }
}

pub fn quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, Rational, T, U)> {
    Bucketer {
        bucketing_function: &|(x, y, z, _)| {
            usize::exact_from(max!(
                x.complexity(),
                y.significant_bits(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.significant_bits(), {z_name}.significant_bits())"
        ),
    }
}

pub fn quadruple_1_2_3_rational_float_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Rational, Float, T, U)> {
    Bucketer {
        bucketing_function: &|(x, y, z, _)| {
            usize::exact_from(max!(
                x.significant_bits(),
                y.complexity(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.complexity(), {z_name}.significant_bits())"
        ),
    }
}

pub fn pair_2_quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
    V,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (V, (Float, Rational, T, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z, _))| {
            usize::exact_from(max!(
                x.complexity(),
                y.significant_bits(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.significant_bits(), {z_name}.significant_bits())"
        ),
    }
}

pub fn triple_float_rational_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, Rational, T)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(
                x.complexity(),
                y.significant_bits(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.significant_bits(), {z_name}.significant_bits)"
        ),
    }
}

pub fn pair_2_triple_float_rational_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (U, (Float, Rational, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z))| {
            usize::exact_from(max!(
                x.complexity(),
                y.significant_bits(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.significant_bits(), {z_name}.significant_bits)"
        ),
    }
}

pub fn pair_2_pair_float_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Float))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn pair_float_gaussian_integer_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, GaussianInteger)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(
                x.complexity()
                    .max(y.real.significant_bits())
                    .max(y.imaginary.significant_bits()),
            )
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_gaussian_rational_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, GaussianRational)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(
                x.complexity()
                    .max(y.real.significant_bits())
                    .max(y.imaginary.significant_bits()),
            )
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_integer_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Integer)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn triple_1_2_float_integer_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Integer, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn quadruple_1_2_float_integer_max_complexity_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Integer, T, U)> {
    Bucketer {
        bucketing_function: &|(x, y, _, _)| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_natural_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Natural)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_rational_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Rational)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_unsigned_max_complexity_bucketer<'a, T: PrimitiveUnsigned>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_signed_max_complexity_bucketer<'a, T: PrimitiveSigned>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_integer_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Integer))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_natural_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Natural))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_rational_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Rational))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_unsigned_max_complexity_bucketer<'a, T, U: PrimitiveUnsigned>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_signed_max_complexity_bucketer<'a, T, U: PrimitiveSigned>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_primitive_float_max_complexity_bucketer<'a, T, U: PrimitiveFloat>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), u64::exact_from(float_size(*y))))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_primitive_float_max_complexity_bucketer<'a, T: PrimitiveFloat>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.complexity(), u64::exact_from(float_size(*y))))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn max_triple_1_float_complexity_triple_2_bucketer<'a, T>(
    x_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (Float, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, p, _)| usize::exact_from(max(x.complexity(), *p)),
        bucketing_label: format!("max({x_name}.complexity(), {p_name})"),
    }
}

pub fn pair_2_max_triple_1_float_complexity_triple_2_bucketer<'a, T, U>(
    x_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (T, (Float, u64, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, p, _))| usize::exact_from(max(x.complexity(), *p)),
        bucketing_label: format!("max({x_name}.complexity(), {p_name})"),
    }
}

pub fn max_pair_1_complexity_pair_2_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, u64)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), *y)),
        bucketing_label: format!("max({x_name}.complexity(), {y_name})"),
    }
}

pub fn pair_2_max_pair_1_complexity_pair_2_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, u64))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| usize::exact_from(max(x.complexity(), *y)),
        bucketing_label: format!("max({x_name}.complexity(), {y_name})"),
    }
}

pub fn quintuple_1_2_3_4_big_exponent_float_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, BigExponentFloat, BigExponentFloat, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, y, z, p, _)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.complexity(), *p))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.complexity(), {p_name})"
        ),
    }
}

pub fn pair_2_quintuple_1_2_3_4_big_exponent_float_max_complexity_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
    p_name: &'a str,
) -> Bucketer<
    'a,
    (
        T,
        (BigExponentFloat, BigExponentFloat, BigExponentFloat, u64, U),
    ),
> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z, p, _))| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.complexity(), *p))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.complexity(), {p_name})"
        ),
    }
}

pub fn quadruple_1_2_3_4_big_exponent_float_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (BigExponentFloat, BigExponentFloat, BigExponentFloat, u64)> {
    Bucketer {
        bucketing_function: &|(x, y, z, p)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.complexity(), *p))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.complexity(), {p_name})"
        ),
    }
}

pub fn sextuple_1_2_3_4_5_big_exponent_float_max_complexity_bucketer<'a, T>(
    p_name: &'a str,
) -> Bucketer<
    'a,
    (
        BigExponentFloat,
        BigExponentFloat,
        BigExponentFloat,
        BigExponentFloat,
        u64,
        T,
    ),
> {
    Bucketer {
        bucketing_function: &|(x, y, z, w, p, _)| {
            usize::exact_from(max!(
                x.complexity(),
                y.complexity(),
                z.complexity(),
                w.complexity(),
                *p
            ))
        },
        bucketing_label: format!(
            "max(x.complexity(), y.complexity(), z.complexity(), w.complexity(), {p_name})"
        ),
    }
}

pub fn pair_2_sextuple_1_2_3_4_5_big_exponent_float_max_complexity_bucketer<'a, T, U>(
    p_name: &'a str,
) -> Bucketer<
    'a,
    (
        T,
        (
            BigExponentFloat,
            BigExponentFloat,
            BigExponentFloat,
            BigExponentFloat,
            u64,
            U,
        ),
    ),
> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z, w, p, _))| {
            usize::exact_from(max!(
                x.complexity(),
                y.complexity(),
                z.complexity(),
                w.complexity(),
                *p
            ))
        },
        bucketing_label: format!(
            "max(x.complexity(), y.complexity(), z.complexity(), w.complexity(), {p_name})"
        ),
    }
}

pub fn quintuple_1_2_3_4_5_big_exponent_float_max_complexity_bucketer<'a>(
    p_name: &'a str,
) -> Bucketer<
    'a,
    (
        BigExponentFloat,
        BigExponentFloat,
        BigExponentFloat,
        BigExponentFloat,
        u64,
    ),
> {
    Bucketer {
        bucketing_function: &|(x, y, z, w, p)| {
            usize::exact_from(max!(
                x.complexity(),
                y.complexity(),
                z.complexity(),
                w.complexity(),
                *p
            ))
        },
        bucketing_label: format!(
            "max(x.complexity(), y.complexity(), z.complexity(), w.complexity(), {p_name})"
        ),
    }
}

fn big_exponent_float_slice_complexity(xs: &[BigExponentFloat]) -> u64 {
    xs.iter().map(BigExponentFloat::complexity).sum()
}

pub fn triple_1_2_big_exponent_float_vec_sum_complexity_bucketer<'a, T>(
    xs_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (Vec<BigExponentFloat>, u64, T)> {
    Bucketer {
        bucketing_function: &|(xs, p, _)| {
            usize::exact_from(max(big_exponent_float_slice_complexity(xs), *p))
        },
        bucketing_label: format!("max(sum of {xs_name} complexities, {p_name})"),
    }
}

pub fn pair_2_triple_1_2_big_exponent_float_vec_sum_complexity_bucketer<'a, T, U>(
    xs_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (T, (Vec<BigExponentFloat>, u64, U))> {
    Bucketer {
        bucketing_function: &|(_, (xs, p, _))| {
            usize::exact_from(max(big_exponent_float_slice_complexity(xs), *p))
        },
        bucketing_label: format!("max(sum of {xs_name} complexities, {p_name})"),
    }
}

pub fn pair_big_exponent_float_vec_sum_complexity_bucketer<'a>(
    xs_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (Vec<BigExponentFloat>, u64)> {
    Bucketer {
        bucketing_function: &|(xs, p)| {
            usize::exact_from(max(big_exponent_float_slice_complexity(xs), *p))
        },
        bucketing_label: format!("max(sum of {xs_name} complexities, {p_name})"),
    }
}

pub fn quadruple_1_2_3_big_exponent_float_vec_pair_sum_complexity_bucketer<'a, T>(
    p_name: &'a str,
) -> Bucketer<'a, (Vec<BigExponentFloat>, Vec<BigExponentFloat>, u64, T)> {
    Bucketer {
        bucketing_function: &|(xs, ys, p, _)| {
            usize::exact_from(max(
                big_exponent_float_slice_complexity(xs) + big_exponent_float_slice_complexity(ys),
                *p,
            ))
        },
        bucketing_label: format!("max(sum of xs and ys complexities, {p_name})"),
    }
}

pub fn pair_2_quadruple_1_2_3_big_exponent_float_vec_pair_sum_complexity_bucketer<'a, T, U>(
    p_name: &'a str,
) -> Bucketer<'a, (T, (Vec<BigExponentFloat>, Vec<BigExponentFloat>, u64, U))> {
    Bucketer {
        bucketing_function: &|(_, (xs, ys, p, _))| {
            usize::exact_from(max(
                big_exponent_float_slice_complexity(xs) + big_exponent_float_slice_complexity(ys),
                *p,
            ))
        },
        bucketing_label: format!("max(sum of xs and ys complexities, {p_name})"),
    }
}

pub fn triple_1_2_3_big_exponent_float_vec_pair_sum_complexity_bucketer<'a>(
    p_name: &'a str,
) -> Bucketer<'a, (Vec<BigExponentFloat>, Vec<BigExponentFloat>, u64)> {
    Bucketer {
        bucketing_function: &|(xs, ys, p)| {
            usize::exact_from(max(
                big_exponent_float_slice_complexity(xs) + big_exponent_float_slice_complexity(ys),
                *p,
            ))
        },
        bucketing_label: format!("max(sum of xs and ys complexities, {p_name})"),
    }
}

pub fn triple_big_exponent_float_max_complexity_bucketer<'a>()
-> Bucketer<'a, (BigExponentFloat, BigExponentFloat, BigExponentFloat)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.complexity()))
        },
        bucketing_label: "max(x.complexity(), y.complexity(), z.complexity())".to_string(),
    }
}

pub fn quadruple_big_exponent_float_max_complexity_bucketer<'a>() -> Bucketer<
    'a,
    (
        BigExponentFloat,
        BigExponentFloat,
        BigExponentFloat,
        BigExponentFloat,
    ),
> {
    Bucketer {
        bucketing_function: &|(x, y, z, w)| {
            usize::exact_from(max!(
                x.complexity(),
                y.complexity(),
                z.complexity(),
                w.complexity()
            ))
        },
        bucketing_label: "max(x.complexity(), y.complexity(), z.complexity(), w.complexity())"
            .to_string(),
    }
}
