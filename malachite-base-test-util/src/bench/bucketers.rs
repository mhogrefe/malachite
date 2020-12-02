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
        bucketing_function: &(|&c| usize::exact_from(char_to_contiguous_range(c))),
        bucketing_label: "char_to_contiguous_range(c)".to_string(),
    }
}

pub fn usize_convertible_direct_bucketer<T: Copy>(var_name: &str) -> Bucketer<T>
where
    usize: ExactFrom<T>,
{
    Bucketer {
        bucketing_function: &(|&x| usize::exact_from(x)),
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
        bucketing_function: &(|&x| usize::exact_from(x.significant_bits())),
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

pub fn pair_max_bit_bucketer<'a, T: Copy + SignificantBits, U: Copy + SignificantBits>(
    x_name: &str,
    y_name: &str,
) -> Bucketer<'a, (T, U)> {
    Bucketer {
        bucketing_function: &(|&(x, y)| {
            usize::exact_from(max(x.significant_bits(), y.significant_bits()))
        }),
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
        bucketing_function: &(|&(x, y, z)| {
            usize::exact_from(max!(
                x.significant_bits(),
                y.significant_bits(),
                z.significant_bits()
            ))
        }),
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
        bucketing_function: &(|&(_, y)| usize::exact_from(y)),
        bucketing_label: y_name.to_string(),
    }
}
