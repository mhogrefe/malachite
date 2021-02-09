use std::cmp::max;

use malachite_base::chars::crement::char_to_contiguous_range;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

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

pub fn bit_bucketer<T: Copy + SignificantBits>(var_name: &str) -> Bucketer<T> {
    Bucketer {
        bucketing_function: &|&x| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", var_name),
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

pub fn string_len_bucketer<'a>() -> Bucketer<'a, String> {
    Bucketer {
        bucketing_function: &String::len,
        bucketing_label: "s.len()".to_string(),
    }
}

pub fn pair_string_len_bucketer<'a>() -> Bucketer<'a, (String, String)> {
    Bucketer {
        bucketing_function: &|(s, t)| max(s.len(), t.len()),
        bucketing_label: "max(s.len(), t.len())".to_string(),
    }
}

pub fn vec_len_bucketer<'a, T>() -> Bucketer<'a, Vec<T>> {
    Bucketer {
        bucketing_function: &Vec::len,
        bucketing_label: "xs.len()".to_string(),
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
        bucketing_label: format!(
            "max({}.significant_bits(), {}.significant_bits())",
            x_name, y_name
        ),
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
            "max({}.significant_bits(), {}.significant_bits(), {}.significant_bits())",
            x_name, y_name, z_name
        ),
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

pub fn triple_2_bucketer<T, U: Copy, V>(y_name: &str) -> Bucketer<(T, U, V)>
where
    usize: ExactFrom<U>,
{
    Bucketer {
        bucketing_function: &|&(_, y, _)| usize::exact_from(y),
        bucketing_label: y_name.to_string(),
    }
}

pub fn pair_1_bit_bucketer<T: Copy + SignificantBits, U>(x_name: &str) -> Bucketer<(T, U)> {
    Bucketer {
        bucketing_function: &|&(x, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", x_name),
    }
}

pub fn triple_1_bit_bucketer<T: Copy + SignificantBits, U, V>(x_name: &str) -> Bucketer<(T, U, V)> {
    Bucketer {
        bucketing_function: &|&(x, _, _)| usize::exact_from(x.significant_bits()),
        bucketing_label: format!("{}.significant_bits()", x_name),
    }
}

pub fn pair_1_vec_len_bucketer<T, U>(xs_name: &str) -> Bucketer<(Vec<T>, U)> {
    Bucketer {
        bucketing_function: &|&(ref xs, _)| xs.len(),
        bucketing_label: format!("{}.len()", xs_name),
    }
}

pub fn triple_1_vec_len_bucketer<T, U, V>(xs_name: &str) -> Bucketer<(Vec<T>, U, V)> {
    Bucketer {
        bucketing_function: &|&(ref xs, _, _)| xs.len(),
        bucketing_label: format!("{}.len()", xs_name),
    }
}

pub fn triple_3_vec_len_bucketer<T, U, V>(xs_name: &str) -> Bucketer<(T, U, Vec<V>)> {
    Bucketer {
        bucketing_function: &|&(_, _, ref xs)| xs.len(),
        bucketing_label: format!("{}.len()", xs_name),
    }
}

pub fn triple_2_vec_len_bucketer<T, U, V>(xs_name: &str) -> Bucketer<(T, Vec<U>, V)> {
    Bucketer {
        bucketing_function: &|&(_, ref xs, _)| xs.len(),
        bucketing_label: format!("{}.len()", xs_name),
    }
}

pub fn quadruple_2_vec_len_bucketer<T, U, V, W>(xs_name: &str) -> Bucketer<(T, Vec<U>, V, W)> {
    Bucketer {
        bucketing_function: &|&(_, ref xs, _, _)| xs.len(),
        bucketing_label: format!("{}.len()", xs_name),
    }
}

pub fn quadruple_3_vec_len_bucketer<T, U, V, W>(xs_name: &str) -> Bucketer<(T, U, Vec<V>, W)> {
    Bucketer {
        bucketing_function: &|&(_, _, ref xs, _)| xs.len(),
        bucketing_label: format!("{}.len()", xs_name),
    }
}

pub fn pair_sum_vec_len_bucketer<'a, T, U>(
    xs_name: &str,
    ys_name: &str,
) -> Bucketer<'a, (Vec<T>, Vec<U>)> {
    Bucketer {
        bucketing_function: &|(xs, ys)| xs.len() + ys.len(),
        bucketing_label: format!("{}.len() + {}.len()", xs_name, ys_name),
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
