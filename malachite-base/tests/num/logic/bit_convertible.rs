use malachite_base::comparison::{Max, Min};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::bit_convertible::{
    _from_bits_asc_alt, _from_bits_asc_signed_naive, _from_bits_asc_unsigned_naive,
    _from_bits_desc_alt, _from_bits_desc_signed_naive, _from_bits_desc_unsigned_naive,
    _to_bits_asc_alt, _to_bits_asc_signed_naive, _to_bits_asc_unsigned_naive, _to_bits_desc_alt,
    _to_bits_desc_signed_naive, _to_bits_desc_unsigned_naive,
};
use malachite_base::num::logic::traits::BitConvertible;

#[test]
pub fn test_to_bits_asc() {
    fn test_unsigned<T: PrimitiveUnsigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_asc(), out);
        assert_eq!(_to_bits_asc_unsigned_naive(x), out);
        assert_eq!(_to_bits_asc_alt(&x), out);
        assert_eq!(x.bits().collect::<Vec<bool>>(), out);
    };
    test_unsigned(0u8, &[]);
    test_unsigned(1u16, &[true]);
    test_unsigned(2u32, &[false, true]);
    test_unsigned(3u64, &[true, true]);
    test_unsigned(123u16, &[true, true, false, true, true, true, true]);
    test_unsigned(u8::MAX, &[true; 8]);

    fn test_signed<T: PrimitiveSigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_asc(), out);
        assert_eq!(_to_bits_asc_signed_naive(x), out);
        assert_eq!(_to_bits_asc_alt(&x), out);
        assert_eq!(x.bits().collect::<Vec<bool>>(), out);
    };
    test_signed(0i8, &[]);
    test_signed(1i16, &[true, false]);
    test_signed(2i32, &[false, true, false]);
    test_signed(3i64, &[true, true, false]);
    test_signed(-1i16, &[true]);
    test_signed(-2i32, &[false, true]);
    test_signed(-3i64, &[true, false, true]);
    test_signed(123i16, &[true, true, false, true, true, true, true, false]);
    test_signed(
        -123i16,
        &[true, false, true, false, false, false, false, true],
    );
    test_signed(i8::MAX, &[true, true, true, true, true, true, true, false]);
    test_signed(
        i8::MIN,
        &[false, false, false, false, false, false, false, true],
    );
}

#[test]
pub fn test_to_bits_desc() {
    fn test_unsigned<T: PrimitiveUnsigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_desc(), out);
        assert_eq!(_to_bits_desc_unsigned_naive(x), out);
        assert_eq!(_to_bits_desc_alt(&x), out);
        assert_eq!(x.bits().rev().collect::<Vec<bool>>(), out);
    };
    test_unsigned(0u8, &[]);
    test_unsigned(1u16, &[true]);
    test_unsigned(2u32, &[true, false]);
    test_unsigned(3u64, &[true, true]);
    test_unsigned(123u16, &[true, true, true, true, false, true, true]);
    test_unsigned(u8::MAX, &[true; 8]);

    fn test_signed<T: PrimitiveSigned>(x: T, out: &[bool]) {
        assert_eq!(x.to_bits_desc(), out);
        assert_eq!(_to_bits_desc_signed_naive(x), out);
        assert_eq!(_to_bits_desc_alt(&x), out);
        assert_eq!(x.bits().rev().collect::<Vec<bool>>(), out);
    };
    test_signed(0i8, &[]);
    test_signed(1i16, &[false, true]);
    test_signed(2i32, &[false, true, false]);
    test_signed(3i64, &[false, true, true]);
    test_signed(-1i16, &[true]);
    test_signed(-2i32, &[true, false]);
    test_signed(-3i64, &[true, false, true]);
    test_signed(123i16, &[false, true, true, true, true, false, true, true]);
    test_signed(
        -123i16,
        &[true, false, false, false, false, true, false, true],
    );
    test_signed(i8::MAX, &[false, true, true, true, true, true, true, true]);
    test_signed(
        i8::MIN,
        &[true, false, false, false, false, false, false, false],
    );
}

#[test]
pub fn test_from_bits_asc() {
    fn test_unsigned<T: PrimitiveUnsigned>(bits: &[bool], out: T) {
        assert_eq!(T::from_bits_asc(bits), out);
        assert_eq!(_from_bits_asc_unsigned_naive::<T>(bits), out);
        assert_eq!(_from_bits_asc_alt::<T>(bits), out);
    };
    test_unsigned(&[], 0u8);
    test_unsigned(&[false], 0u8);
    test_unsigned(&[false, false, false], 0u8);
    test_unsigned(&[false; 100], 0u8);
    test_unsigned(&[true], 1u16);
    test_unsigned(&[false, true], 2u32);
    test_unsigned(&[true, true], 3u64);
    test_unsigned(&[true, true, false, true, true, true, true], 123u16);
    test_unsigned(
        &[
            true, true, false, true, true, true, true, false, false, false,
        ],
        123u16,
    );
    test_unsigned(&[true; 8], u8::MAX);

    fn test_signed<T: PrimitiveSigned>(bits: &[bool], out: T) {
        assert_eq!(T::from_bits_asc(bits), out);
        assert_eq!(_from_bits_asc_signed_naive::<T>(bits), out);
        assert_eq!(_from_bits_asc_alt::<T>(bits), out);
    };
    test_signed(&[], 0i8);
    test_signed(&[false], 0i8);
    test_signed(&[false, false, false], 0i8);
    test_signed(&[false; 100], 0i8);
    test_signed(&[true, false], 1i16);
    test_signed(&[false, true, false], 2i32);
    test_signed(&[true, true, false], 3i64);
    test_signed(&[true], -1i16);
    test_signed(&[true, true, true], -1i16);
    test_signed(&[true; 100], -1i16);
    test_signed(&[false, true], -2i32);
    test_signed(&[true, false, true], -3i64);
    test_signed(&[true, true, false, true, true, true, true, false], 123i16);
    test_signed(
        &[
            true, true, false, true, true, true, true, false, false, false, false,
        ],
        123i16,
    );
    test_signed(
        &[true, false, true, false, false, false, false, true],
        -123i16,
    );
    test_signed(
        &[
            true, false, true, false, false, false, false, true, true, true, true,
        ],
        -123i16,
    );
    test_signed(&[true, true, true, true, true, true, true, false], i8::MAX);
    test_signed(
        &[false, false, false, false, false, false, false, true],
        i8::MIN,
    );
}

macro_rules! from_bits_asc_fail_helper_unsigned {
    ($t:ident, $fail:ident) => {
        #[test]
        #[should_panic]
        fn $fail() {
            $t::from_bits_asc(&[true; 200]);
        }
    };
}

from_bits_asc_fail_helper_unsigned!(u8, u8_from_bits_asc_fail);
from_bits_asc_fail_helper_unsigned!(u16, u16_from_bits_asc_fail);
from_bits_asc_fail_helper_unsigned!(u32, u32_from_bits_asc_fail);
from_bits_asc_fail_helper_unsigned!(u64, u64_from_bits_asc_fail);
from_bits_asc_fail_helper_unsigned!(u128, u128_from_bits_asc_fail);
from_bits_asc_fail_helper_unsigned!(usize, usize_from_bits_asc_fail);

macro_rules! from_bits_asc_fail_helper_signed {
    ($t:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            let mut bits = vec![false; 200];
            bits.push(true);
            bits.push(false);
            $t::from_bits_asc(&bits);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            let mut bits = vec![false; 200];
            bits.push(true);
            $t::from_bits_asc(&bits);
        }
    };
}

from_bits_asc_fail_helper_signed!(i8, i8_from_bits_asc_fail_1, i8_from_bits_asc_fail_2);
from_bits_asc_fail_helper_signed!(i16, i16_from_bits_asc_fail_1, i16_from_bits_asc_fail_2);
from_bits_asc_fail_helper_signed!(i32, i32_from_bits_asc_fail_1, i32_from_bits_asc_fail_2);
from_bits_asc_fail_helper_signed!(i64, i64_from_bits_asc_fail_1, i64_from_bits_asc_fail_2);
from_bits_asc_fail_helper_signed!(i128, i128_from_bits_asc_fail_1, i128_from_bits_asc_fail_2);
from_bits_asc_fail_helper_signed!(
    isize,
    isize_from_bits_asc_fail_1,
    isize_from_bits_asc_fail_2
);

#[test]
pub fn test_from_bits_desc() {
    fn test_unsigned<T: PrimitiveUnsigned>(bits: &[bool], out: T) {
        assert_eq!(T::from_bits_desc(bits), out);
        assert_eq!(_from_bits_desc_unsigned_naive::<T>(bits), out);
        assert_eq!(_from_bits_desc_alt::<T>(bits), out);
    };
    test_unsigned(&[], 0u8);
    test_unsigned(&[false], 0u8);
    test_unsigned(&[false, false, false], 0u8);
    test_unsigned(&[false; 100], 0u8);
    test_unsigned(&[true], 1u16);
    test_unsigned(&[true, false], 2u32);
    test_unsigned(&[true, true], 3u64);
    test_unsigned(&[true, true, true, true, false, true, true], 123u16);
    test_unsigned(
        &[
            false, false, false, true, true, true, true, false, true, true,
        ],
        123u16,
    );
    test_unsigned(&[true; 8], u8::MAX);

    fn test_signed<T: PrimitiveSigned>(bits: &[bool], out: T) {
        assert_eq!(T::from_bits_desc(bits), out);
        assert_eq!(_from_bits_desc_signed_naive::<T>(bits), out);
        assert_eq!(_from_bits_desc_alt::<T>(bits), out);
    };
    test_signed(&[], 0i8);
    test_signed(&[false], 0i8);
    test_signed(&[false, false, false], 0i8);
    test_signed(&[false; 100], 0i8);
    test_signed(&[false, true], 1i16);
    test_signed(&[false, true, false], 2i32);
    test_signed(&[false, true, true], 3i64);
    test_signed(&[true], -1i16);
    test_signed(&[true, true, true], -1i16);
    test_signed(&[true; 100], -1i16);
    test_signed(&[true, false], -2i32);
    test_signed(&[true, false, true], -3i64);
    test_signed(&[false, true, true, true, true, false, true, true], 123i16);
    test_signed(
        &[
            false, false, false, false, true, true, true, true, false, true, true,
        ],
        123i16,
    );
    test_signed(
        &[true, false, false, false, false, true, false, true],
        -123i16,
    );
    test_signed(
        &[
            true, true, true, true, false, false, false, false, true, false, true,
        ],
        -123i16,
    );
    test_signed(&[false, true, true, true, true, true, true, true], i8::MAX);
    test_signed(
        &[true, false, false, false, false, false, false, false],
        i8::MIN,
    );
}

macro_rules! from_bits_desc_fail_helper_unsigned {
    ($t:ident, $fail:ident) => {
        #[test]
        #[should_panic]
        fn $fail() {
            $t::from_bits_desc(&[true; 200]);
        }
    };
}

from_bits_desc_fail_helper_unsigned!(u8, u8_from_bits_desc_fail);
from_bits_desc_fail_helper_unsigned!(u16, u16_from_bits_desc_fail);
from_bits_desc_fail_helper_unsigned!(u32, u32_from_bits_desc_fail);
from_bits_desc_fail_helper_unsigned!(u64, u64_from_bits_desc_fail);
from_bits_desc_fail_helper_unsigned!(u128, u128_from_bits_desc_fail);
from_bits_desc_fail_helper_unsigned!(usize, usize_from_bits_desc_fail);

macro_rules! from_bits_desc_fail_helper_signed {
    ($t:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            let mut bits = vec![false; 202];
            bits[1] = true;
            $t::from_bits_desc(&bits);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            let mut bits = vec![false; 201];
            bits[1] = true;
            $t::from_bits_desc(&bits);
        }
    };
}

from_bits_desc_fail_helper_signed!(i8, i8_from_bits_desc_fail_1, i8_from_bits_desc_fail_2);
from_bits_desc_fail_helper_signed!(i16, i16_from_bits_desc_fail_1, i16_from_bits_desc_fail_2);
from_bits_desc_fail_helper_signed!(i32, i32_from_bits_desc_fail_1, i32_from_bits_desc_fail_2);
from_bits_desc_fail_helper_signed!(i64, i64_from_bits_desc_fail_1, i64_from_bits_desc_fail_2);
from_bits_desc_fail_helper_signed!(i128, i128_from_bits_desc_fail_1, i128_from_bits_desc_fail_2);
from_bits_desc_fail_helper_signed!(
    isize,
    isize_from_bits_desc_fail_1,
    isize_from_bits_desc_fail_2
);
