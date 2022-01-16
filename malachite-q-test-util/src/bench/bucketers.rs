use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::bucketers::Bucketer;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use std::cmp::max;

pub fn rational_bit_bucketer(var_name: &str) -> Bucketer<Rational> {
    Bucketer {
        bucketing_function: &|q| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn pair_2_pair_1_rational_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, (Rational, U))> {
    Bucketer {
        bucketing_function: &|(_, (q, _))| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn pair_1_rational_bit_bucketer<T>(var_name: &str) -> Bucketer<(Rational, T)> {
    Bucketer {
        bucketing_function: &|(q, _)| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn triple_1_rational_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(Rational, T, U)> {
    Bucketer {
        bucketing_function: &|(q, _, _)| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn triple_3_rational_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, U, Rational)> {
    Bucketer {
        bucketing_function: &|(_, _, q)| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn quadruple_1_rational_bit_bucketer<T, U, V>(var_name: &str) -> Bucketer<(Rational, T, U, V)> {
    Bucketer {
        bucketing_function: &|(q, _, _, _)| usize::exact_from(q.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
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
        bucketing_label: format!(
            "max({}.significant_bits(), {}.significant_bits())",
            x_name, y_name
        ),
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
        bucketing_label: format!(
            "max({}.significant_bits(), {}.significant_bits())",
            x_name, y_name
        ),
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
        bucketing_label: format!(
            "max({}.significant_bits(), {}.significant_bits())",
            x_name, y_name
        ),
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
        bucketing_label: format!(
            "max({}.significant_bits(), {}.significant_bits())",
            x_name, y_name
        ),
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
        bucketing_label: format!(
            "max({}.significant_bits(), {}.significant_bits())",
            x_name, y_name
        ),
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
        bucketing_label: format!(
            "max({}.significant_bits(), {}.significant_bits())",
            x_name, y_name
        ),
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
        bucketing_label: format!(
            "max({}.significant_bits(), {}.significant_bits())",
            x_name, y_name
        ),
    }
}

pub fn rational_deserialize_bucketer<'a>() -> Bucketer<'a, (String, String, String)> {
    Bucketer {
        bucketing_function: &|&(_, _, ref s)| {
            let n: Rational = serde_json::from_str(s).unwrap();
            usize::exact_from(n.significant_bits())
        },
        bucketing_label: "n.significant_bits()".to_string(),
    }
}
