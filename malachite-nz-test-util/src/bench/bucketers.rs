use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::bucketers::Bucketer;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::max;

pub fn natural_bit_bucketer(var_name: &str) -> Bucketer<Natural> {
    Bucketer {
        bucketing_function: &|x| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
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
        bucketing_label: format!(
            "max({}.significant_bits(), {}.significant_bits())",
            x_name, y_name
        ),
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
        bucketing_label: format!(
            "max({}.significant_bits(), {}.significant_bits())",
            x_name, y_name
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
        bucketing_label: format!(
            "max({}.significant_bits(), {}.significant_bits())",
            x_name, y_name
        ),
    }
}

pub fn pair_1_natural_bit_bucketer<T>(var_name: &str) -> Bucketer<(Natural, T)> {
    Bucketer {
        bucketing_function: &|(x, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn pair_2_natural_bit_bucketer<T>(var_name: &str) -> Bucketer<(T, Natural)> {
    Bucketer {
        bucketing_function: &|(_, x)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn triple_1_natural_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(Natural, T, U)> {
    Bucketer {
        bucketing_function: &|(x, _, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn triple_3_natural_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, U, Natural)> {
    Bucketer {
        bucketing_function: &|(_, _, x)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn triple_3_pair_1_natural_bit_bucketer<T, U, V>(
    var_name: &str,
) -> Bucketer<(T, U, (Natural, V))> {
    Bucketer {
        bucketing_function: &|(_, _, (x, _))| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn integer_bit_bucketer(var_name: &str) -> Bucketer<Integer> {
    Bucketer {
        bucketing_function: &|x| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn pair_1_integer_bit_bucketer<T>(var_name: &str) -> Bucketer<(Integer, T)> {
    Bucketer {
        bucketing_function: &|(x, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn pair_2_integer_bit_bucketer<T>(var_name: &str) -> Bucketer<(T, Integer)> {
    Bucketer {
        bucketing_function: &|(_, x)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn triple_1_integer_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(Integer, T, U)> {
    Bucketer {
        bucketing_function: &|(x, _, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn triple_3_integer_bit_bucketer<T, U>(var_name: &str) -> Bucketer<(T, U, Integer)> {
    Bucketer {
        bucketing_function: &|(_, _, x)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
    }
}

pub fn triple_3_pair_1_integer_bit_bucketer<T, U, V>(
    var_name: &str,
) -> Bucketer<(T, U, (Integer, V))> {
    Bucketer {
        bucketing_function: &|(_, _, (x, _))| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
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
        bucketing_label: format!(
            "{}.significant_bits() / {}.significant_bits()",
            x_name, y_name
        ),
    }
}

pub fn pair_1_vec_len_times_pair_2_natural_bits_bucketer<'a, T>(
    xs_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Vec<T>, Natural)> {
    Bucketer {
        bucketing_function: &|&(ref xs, ref y)| {
            xs.len()
                .checked_mul(usize::exact_from(y.significant_bits()))
                .unwrap()
        },
        bucketing_label: format!("{}.len() * {}.significant_bits()", xs_name, y_name),
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
        bucketing_label: format!("{}.len() * {}", xs_name, y_name),
    }
}

pub fn natural_deserialize_bucketer<'a>() -> Bucketer<'a, (String, String, String)> {
    Bucketer {
        bucketing_function: &|&(_, _, ref s)| {
            let n: Natural = serde_json::from_str(s).unwrap();
            usize::exact_from(n.significant_bits())
        },
        bucketing_label: "n.significant_bits()".to_string(),
    }
}

pub fn integer_deserialize_bucketer<'a>() -> Bucketer<'a, (String, String, String)> {
    Bucketer {
        bucketing_function: &|&(_, _, ref s)| {
            let n: Integer = serde_json::from_str(s).unwrap();
            usize::exact_from(n.significant_bits())
        },
        bucketing_label: "n.significant_bits()".to_string(),
    }
}
