// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::bench::bucketers::Bucketer;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::max;

pub fn rational_bit_bucketer(var_name: &str) -> Bucketer<Rational> {
    Bucketer {
        bucketing_function: &|q| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn pair_2_pair_1_rational_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, (Rational, U))> {
    Bucketer {
        bucketing_function: &|(_, (q, _))| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn pair_1_rational_bit_bucketer<T>(var_name: &str) -> Bucketer<(Rational, T)> {
    Bucketer {
        bucketing_function: &|(q, _)| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn pair_2_rational_bit_bucketer<T>(var_name: &str) -> Bucketer<(T, Rational)> {
    Bucketer {
        bucketing_function: &|(_, q)| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn triple_1_rational_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(Rational, T, U)> {
    Bucketer {
        bucketing_function: &|(q, _, _)| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn triple_3_rational_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, U, Rational)> {
    Bucketer {
        bucketing_function: &|(_, _, q)| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn quadruple_1_rational_bit_bucketer<T, U, V>(var_name: &str) -> Bucketer<(Rational, T, U, V)> {
    Bucketer {
        bucketing_function: &|(q, _, _, _)| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{var_name}.significant_bits()"),
    }
}

pub fn pair_rational_max_bit_bucketer<'a>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (Rational, Rational)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_rational_max_bit_bucketer<'a, T>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (T, (Rational, Rational))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn triple_3_pair_rational_max_bit_bucketer<'a, T, U>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (T, U, (Rational, Rational))> {
    Bucketer {
        bucketing_function: &|(_, _, (x, y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_rational_integer_max_bit_bucketer<'a, T>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (T, (Rational, Integer))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn rational_natural_max_bit_bucketer<'a>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (Rational, Natural)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn rational_integer_max_bit_bucketer<'a>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (Rational, Integer)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_rational_natural_max_bit_bucketer<'a, T>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (T, (Rational, Natural))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.significant_bits())"),
    }
}

pub fn rational_deserialize_bucketer<'a>() -> Bucketer<'a, (String, String, String)> {
    Bucketer {
        bucketing_function: &|(_, _, s)| {
            let n: Rational = serde_json::from_str(s).unwrap();
            usize::exact_from(n.significant_bits())
        },
        bucketing_label: "n.significant_bits()".to_string(),
    }
}

pub fn triple_3_pair_1_rational_bits_times_pair_2_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, U, (Rational, u64))> {
    Bucketer {
        bucketing_function: &|&(_, _, (ref x, y))| usize::exact_from(x.significant_bits() * y),
        bucketing_label: format!("{x_name}.significant_bits() * {y_name}"),
    }
}

pub fn triple_3_pair_1_rational_bits_times_abs_pair_2_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, U, (Rational, i64))> {
    Bucketer {
        bucketing_function: &|&(_, _, (ref x, y))| {
            usize::exact_from(x.significant_bits() * y.unsigned_abs())
        },
        bucketing_label: format!("{x_name}.significant_bits() * {y_name}"),
    }
}

pub fn triple_1_2_rational_bit_i64_max_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Rational, i64, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| {
            usize::exact_from(max(x.significant_bits(), y.unsigned_abs()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name})"),
    }
}

pub fn vec_rational_sum_bits_bucketer<'a>() -> Bucketer<'a, Vec<Rational>> {
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

pub fn triple_3_vec_rational_sum_bits_bucketer<'a, T, U>() -> Bucketer<'a, (T, U, Vec<Rational>)> {
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

pub fn pair_rational_bit_i64_max_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Rational, i64)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.unsigned_abs()))
        },
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.unsigned_abs())"),
    }
}

pub fn pair_rational_bit_u64_max_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Rational, u64)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.significant_bits(), *y)),
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name}.unsigned_abs())"),
    }
}

pub fn triple_1_2_rational_bit_u64_max_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Rational, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| usize::exact_from(max(x.significant_bits(), *y)),
        bucketing_label: format!("max({x_name}.significant_bits(), {y_name})"),
    }
}

pub fn triple_rational_bit_i64_u64_max_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Rational, i64, u64)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(x.significant_bits(), y.unsigned_abs(), *z))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.unsigned_abs(), {z_name})"
        ),
    }
}

pub fn quadruple_1_2_3_rational_bit_i64_u64_max_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Rational, i64, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, y, z, _)| {
            usize::exact_from(max!(x.significant_bits(), y.unsigned_abs(), *z))
        },
        bucketing_label: format!(
            "max({x_name}.significant_bits(), {y_name}.unsigned_abs(), {z_name})"
        ),
    }
}
