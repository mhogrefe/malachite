use malachite_base::chars::crement::char_to_contiguous_range;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

pub struct Bucketer<'a, T> {
    pub bucketing_function: &'a dyn Fn(&T) -> usize,
    pub bucketing_label: &'a str,
}

pub fn char_bucketer<'a>() -> Bucketer<'a, char> {
    Bucketer {
        bucketing_function: &(|&c| usize::exact_from(char_to_contiguous_range(c))),
        bucketing_label: "char_to_contiguous_range(c)",
    }
}

pub fn usize_convertible_direct_bucketer<T: Copy>(var_name: &str) -> Bucketer<T>
where
    usize: ExactFrom<T>,
{
    Bucketer {
        bucketing_function: &(|&x| usize::exact_from(x)),
        bucketing_label: var_name,
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
        bucketing_label: var_name,
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
