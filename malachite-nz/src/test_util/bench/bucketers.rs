// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::logic::significant_bits::limbs_significant_bits;
use crate::natural::Natural;
use crate::platform::Limb;
use crate::test_util::natural::arithmetic::gcd::OwnedHalfGcdMatrix;
use malachite_base::max;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rational_sequences::RationalSequence;
use malachite_base::test_util::bench::bucketers::Bucketer;
use std::cmp::{max, min};

pub fn natural_bucketer(var_name: &str) -> Bucketer<Natural> {
    Bucketer {
        bucketing_function: &|x| usize::exact_from(x),
        bucketing_label: var_name.to_string(),
    }
}

pub fn natural_bit_bucketer(var_name: &str) -> Bucketer<Natural> {
    Bucketer {
        bucketing_function: &|x| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn pair_natural_max_bit_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Natural, Natural)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_natural_min_bit_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Natural, Natural)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(min(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("min({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_natural_max_bit_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Natural, Natural))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_natural_min_bit_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Natural, Natural))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(min(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("min({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_natural_bit_u64_max_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Natural, u64)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.significant_bits(), *y)),
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name})"),
    }
}

pub fn pair_natural_bit_i64_max_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Natural, i64)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.unsigned_abs()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.unsigned_abs())"),
    }
}

pub fn triple_natural_bit_i64_u64_max_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Natural, i64, u64)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(x.significant_bits(), y.unsigned_abs(), *z))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.unsigned_abs(), {z_name})"
        ),
    }
}

pub fn quadruple_1_2_3_natural_bit_i64_u64_max_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Natural, i64, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, y, z, _)| {
            usize::exact_from(max!(x.significant_bits(), y.unsigned_abs(), *z))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.unsigned_abs(), {z_name})"
        ),
    }
}

pub fn pair_integer_bit_i64_max_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Integer, i64)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.unsigned_abs()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.unsigned_abs())"),
    }
}

pub fn pair_integer_bit_u64_max_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Integer, u64)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.significant_bits(), *y)),
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name})"),
    }
}

pub fn pair_2_pair_integer_bit_u64_max_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Integer, u64))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| usize::exact_from(max(x.significant_bits(), *y)),
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name})"),
    }
}

pub fn triple_integer_bit_i64_u64_max_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Integer, i64, u64)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(x.significant_bits(), y.unsigned_abs(), *z))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.unsigned_abs(), {z_name})"
        ),
    }
}

pub fn quadruple_1_2_3_integer_bit_i64_u64_max_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Integer, i64, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, y, z, _)| {
            usize::exact_from(max!(x.significant_bits(), y.unsigned_abs(), *z))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.unsigned_abs(), {z_name})"
        ),
    }
}

pub fn triple_3_pair_natural_max_bit_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, U, (Natural, Natural))> {
    Bucketer {
        bucketing_function: &|(_, _, (x, y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn triple_3_pair_natural_min_bit_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, U, (Natural, Natural))> {
    Bucketer {
        bucketing_function: &|(_, _, (x, y))| {
            usize::exact_from(min(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("min({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_1_natural_bit_bucketer<T>(var_name: &str) -> Bucketer<(Natural, T)> {
    Bucketer {
        bucketing_function: &|(x, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn pair_2_natural_bit_bucketer<T>(var_name: &str) -> Bucketer<(T, Natural)> {
    Bucketer {
        bucketing_function: &|(_, x)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn triple_1_natural_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(Natural, T, U)> {
    Bucketer {
        bucketing_function: &|(x, _, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn triple_3_natural_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, U, Natural)> {
    Bucketer {
        bucketing_function: &|(_, _, x)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn triple_3_pair_1_natural_bit_bucketer<T, U, V>(
    var_name: &str,
) -> Bucketer<(T, U, (Natural, V))> {
    Bucketer {
        bucketing_function: &|(_, _, (x, _))| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn pair_2_pair_1_natural_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, (Natural, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, _))| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn triple_natural_max_bit_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Natural, Natural, Natural)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(
                x.significant_bits(),
                y.significant_bits(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.significant_bits(), \
            {z_name}.significant_bits())"
        ),
    }
}

pub fn integer_bit_bucketer(var_name: &str) -> Bucketer<Integer> {
    Bucketer {
        bucketing_function: &|x| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn pair_1_integer_bit_bucketer<T>(var_name: &str) -> Bucketer<(Integer, T)> {
    Bucketer {
        bucketing_function: &|(x, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn pair_2_integer_bit_bucketer<T>(var_name: &str) -> Bucketer<(T, Integer)> {
    Bucketer {
        bucketing_function: &|(_, x)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn pair_integer_max_bit_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Integer, Integer)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn triple_integer_max_bit_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Integer, Integer, Integer)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(
                x.significant_bits(),
                y.significant_bits(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.significant_bits(), \
            {z_name}.significant_bits())"
        ),
    }
}

pub fn triple_1_2_natural_max_bit_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Natural, Natural, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_triple_1_2_natural_max_bit_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Natural, Natural, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, _))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn triple_1_2_integer_max_bit_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Integer, Integer, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_triple_1_2_integer_max_bit_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Integer, Integer, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, _))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_integer_max_bit_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Integer, Integer))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn integer_natural_max_bit_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Integer, Natural)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_1_integer_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, (Integer, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, _))| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn pair_2_integer_natural_max_bit_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Integer, Natural))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn triple_1_integer_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(Integer, T, U)> {
    Bucketer {
        bucketing_function: &|(x, _, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn triple_3_integer_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, U, Integer)> {
    Bucketer {
        bucketing_function: &|(_, _, x)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn triple_3_pair_1_integer_bit_bucketer<T, U, V>(
    var_name: &str,
) -> Bucketer<(T, U, (Integer, V))> {
    Bucketer {
        bucketing_function: &|(_, _, (x, _))| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn triple_3_pair_integer_max_bit_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, U, (Integer, Integer))> {
    Bucketer {
        bucketing_function: &|(_, _, (x, y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn natural_bit_ratio_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Natural, Natural)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(x.significant_bits() / y.significant_bits())
        },
        bucketing_label: format!("{x_name}.significant_bits() / {y_name}.significant_bits()"),
    }
}

pub fn pair_1_vec_len_times_pair_2_natural_bits_bucketer<'a, T>(
    xs_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Vec<T>, Natural)> {
    Bucketer {
        bucketing_function: &|(xs, y)| {
            xs.len()
                .checked_mul(usize::exact_from(y.significant_bits()))
                .unwrap()
        },
        bucketing_label: format!("{xs_name}.len() * {y_name}.significant_bits()"),
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
        bucketing_function: &|&(ref xs, y)| xs.len().checked_mul(usize::exact_from(y)).unwrap(),
        bucketing_label: format!("{xs_name}.len() * {y_name}"),
    }
}

pub fn natural_deserialize_bucketer<'a>() -> Bucketer<'a, (String, String, String)> {
    Bucketer {
        bucketing_function: &|(_, _, s)| {
            let n: Natural = serde_json::from_str(s).unwrap();
            usize::exact_from(n.significant_bits())
        },
        bucketing_label: "n.significant_bits()".to_string(),
    }
}

pub fn integer_deserialize_bucketer<'a>() -> Bucketer<'a, (String, String, String)> {
    Bucketer {
        bucketing_function: &|(_, _, s)| {
            let n: Integer = serde_json::from_str(s).unwrap();
            usize::exact_from(n.significant_bits())
        },
        bucketing_label: "n.significant_bits()".to_string(),
    }
}

pub fn triple_1_3_prod_natural_bits_bucketer<'a, T>(
    xs_name: &'a str,
    zs_name: &'a str,
) -> Bucketer<'a, (Natural, T, Natural)> {
    Bucketer {
        bucketing_function: &|(x, _, z)| {
            usize::exact_from(x.significant_bits())
                .checked_mul(usize::exact_from(z.significant_bits()))
                .unwrap()
        },
        bucketing_label: format!("{xs_name}.significant_bits() * {zs_name}.significant_bits()"),
    }
}

pub fn triple_3_triple_1_3_prod_natural_bits_bucketer<'a, T, U, V>(
    xs_name: &'a str,
    zs_name: &'a str,
) -> Bucketer<'a, (T, U, (Natural, V, Natural))> {
    Bucketer {
        bucketing_function: &|(_, _, (x, _, z))| {
            usize::exact_from(x.significant_bits())
                .checked_mul(usize::exact_from(z.significant_bits()))
                .unwrap()
        },
        bucketing_label: format!("{xs_name}.significant_bits() * {zs_name}.significant_bits()"),
    }
}

pub fn pair_1_half_gcd_matrix_bucketer<T>(m_name: &str) -> Bucketer<(OwnedHalfGcdMatrix, T)> {
    Bucketer {
        bucketing_function: &|(m, _)| m.s,
        bucketing_label: m_name.to_string(),
    }
}

pub fn triple_1_half_gcd_matrix_bucketer<T, U>(
    m_name: &str,
) -> Bucketer<(OwnedHalfGcdMatrix, T, U)> {
    Bucketer {
        bucketing_function: &|(m, _, _)| m.s,
        bucketing_label: m_name.to_string(),
    }
}

#[allow(clippy::type_complexity)]
pub fn limbs_matrix_2_2_mul_bucketer<'a>() -> Bucketer<
    'a,
    (
        Vec<Limb>,
        Vec<Limb>,
        Vec<Limb>,
        Vec<Limb>,
        usize,
        Vec<Limb>,
        Vec<Limb>,
        Vec<Limb>,
        Vec<Limb>,
    ),
> {
    Bucketer {
        bucketing_function: &|(_, _, _, _, xs_len, ys00, _, _, _)| max(*xs_len, ys00.len()),
        bucketing_label: "max(xs_len, ys_len)".to_string(),
    }
}

pub fn triple_3_pair_1_integer_bits_times_pair_2_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, U, (Integer, u64))> {
    Bucketer {
        bucketing_function: &|&(_, _, (ref x, y))| usize::exact_from(x.significant_bits() * y),
        bucketing_label: format!("{x_name}.significant_bits() * {y_name}"),
    }
}

pub fn triple_1_2_natural_bit_u64_max_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Natural, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| usize::exact_from(max(x.significant_bits(), *y)),
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name})"),
    }
}

pub fn triple_1_2_integer_bit_u64_max_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Integer, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| usize::exact_from(max(x.significant_bits(), *y)),
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name})"),
    }
}

pub fn limbs_div_to_out_balancing_bucketer<'a>() -> Bucketer<'a, (Vec<Limb>, Vec<Limb>, Vec<Limb>)>
{
    Bucketer {
        bucketing_function: &|(_, ns, ds)| max(2, (ds.len() << 1).saturating_sub(ns.len())),
        bucketing_label: "max(2, 2 * ds.len() - ns.len())".to_string(),
    }
}

#[allow(clippy::type_complexity)]
pub fn limbs_div_mod_extra_bucketer<'a>(
) -> Bucketer<'a, (Vec<Limb>, usize, Vec<Limb>, Limb, Limb, u64)> {
    Bucketer {
        bucketing_function: &|(_, fraction_len, ref ns, _, _, _)| ns.len() + fraction_len,
        bucketing_label: "ns.len() + fraction_len".to_string(),
    }
}

#[allow(clippy::type_complexity)]
pub fn limbs_div_mod_barrett_product_bucketer<'a>(
) -> Bucketer<'a, (Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>, usize, usize)> {
    Bucketer {
        bucketing_function: &|(_, _, _, _, _, i_len)| i_len << 1,
        bucketing_label: "2 * i_len".to_string(),
    }
}

#[allow(clippy::type_complexity)]
pub fn limbs_div_mod_barrett_helper_bucketer<'a>(
) -> Bucketer<'a, (Vec<Limb>, Vec<Limb>, Vec<Limb>, Vec<Limb>)> {
    Bucketer {
        bucketing_function: &|(_, _, ns, ds)| (ds.len() << 1).saturating_sub(ns.len()),
        bucketing_label: "max(0, 2 * ds.len() - ns.len())".to_string(),
    }
}

pub fn limb_pair_significant_bits_bucketer(var_name: &str) -> Bucketer<(Limb, Limb)> {
    Bucketer {
        bucketing_function: &|&(hi, lo)| usize::exact_from(limbs_significant_bits(&[lo, hi])),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

#[allow(clippy::type_complexity)]
pub fn limbs_mod_mul_two_limbs_bucketer<'a>(
) -> Bucketer<'a, (Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb)> {
    Bucketer {
        bucketing_function: &|&(x_1, x_0, y_1, y_0, _, _, _, _, _)| {
            usize::exact_from(max(
                limbs_significant_bits(&[x_0, x_1]),
                limbs_significant_bits(&[y_0, y_1]),
            ))
        },
        bucketing_label: "m.significant_bits()".to_string(),
    }
}

pub fn limbs_mod_limb_small_unnormalized_bucketer<'a>() -> Bucketer<'a, (Vec<Limb>, Limb)> {
    Bucketer {
        bucketing_function: &|(ns, d)| {
            if *ns.last().unwrap() < *d {
                ns.len() - 1
            } else {
                ns.len()
            }
        },
        bucketing_label: "adjusted ns.len()".to_string(),
    }
}

pub fn rational_from_power_of_2_digits_bucketer<'a>(
) -> Bucketer<'a, (u64, Vec<Natural>, RationalSequence<Natural>)> {
    Bucketer {
        bucketing_function: &|(log_base, xs, ys)| {
            usize::exact_from(*log_base) * max(xs.len(), ys.component_len())
        },
        bucketing_label: "log_base * max(before_point.len(), after_point.component_len})"
            .to_string(),
    }
}

pub fn rational_from_digits_bucketer<'a>(
) -> Bucketer<'a, (Natural, Vec<Natural>, RationalSequence<Natural>)> {
    Bucketer {
        bucketing_function: &|(base, xs, ys)| {
            usize::exact_from(base.significant_bits()) * max(xs.len(), ys.component_len())
        },
        bucketing_label:
            "base.significant_bits() * max(before_point.len(), after_point.component_len})"
                .to_string(),
    }
}

pub fn vec_integer_sum_bits_bucketer<'a>() -> Bucketer<'a, Vec<Integer>> {
    Bucketer {
        bucketing_function: &|xs| {
            usize::exact_from(
                xs.iter()
                    .map(SignificantBits::significant_bits)
                    .sum::<u64>(),
            )
        },
        bucketing_label: "xs.map(|x| x.significant_bits()).sum()".to_string(),
    }
}

pub fn vec_natural_sum_bits_bucketer<'a>() -> Bucketer<'a, Vec<Natural>> {
    Bucketer {
        bucketing_function: &|xs| {
            usize::exact_from(
                xs.iter()
                    .map(SignificantBits::significant_bits)
                    .sum::<u64>(),
            )
        },
        bucketing_label: "xs.map(|x| x.significant_bits()).sum()".to_string(),
    }
}

pub fn pair_1_vec_natural_sum_bits_bucketer<'a, T>() -> Bucketer<'a, (Vec<Natural>, T)> {
    Bucketer {
        bucketing_function: &|(xs, _)| {
            usize::exact_from(
                xs.iter()
                    .map(SignificantBits::significant_bits)
                    .sum::<u64>(),
            )
        },
        bucketing_label: "xs.map(|x| x.significant_bits()).sum()".to_string(),
    }
}

pub fn triple_3_vec_integer_sum_bits_bucketer<'a, T, U>() -> Bucketer<'a, (T, U, Vec<Integer>)> {
    Bucketer {
        bucketing_function: &|(_, _, xs)| {
            usize::exact_from(
                xs.iter()
                    .map(SignificantBits::significant_bits)
                    .sum::<u64>(),
            )
        },
        bucketing_label: "xs.map(|x| x.significant_bits()).sum()".to_string(),
    }
}

pub fn triple_3_vec_natural_sum_bits_bucketer<'a, T, U>() -> Bucketer<'a, (T, U, Vec<Natural>)> {
    Bucketer {
        bucketing_function: &|(_, _, xs)| {
            usize::exact_from(
                xs.iter()
                    .map(SignificantBits::significant_bits)
                    .sum::<u64>(),
            )
        },
        bucketing_label: "xs.map(|x| x.significant_bits()).sum()".to_string(),
    }
}
