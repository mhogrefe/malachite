use num::arithmetic::traits::{Parity, PowerOfTwo};
use num::basic::integers::PrimitiveInteger;
use num::conversion::traits::{ExactFrom, WrappingFrom};
use num::logic::traits::{BitAccess, BitConvertible, LeadingZeros, LowMask};

macro_rules! impl_bit_convertible_unsigned {
    ($t:ident) => {
        impl BitConvertible for $t {
            /// Returns a `Vec` containing the bits of `self` in ascending order: least- to most-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, it ends with `true`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(0u8.to_bits_asc(), &[]);
            /// assert_eq!(2u16.to_bits_asc(), &[false, true]);
            /// assert_eq!(123u32.to_bits_asc(), &[true, true, false, true, true, true, true]);
            /// ```
            fn to_bits_asc(&self) -> Vec<bool> {
                let mut bits = Vec::new();
                let mut x = *self;
                while x != 0 {
                    bits.push(x.odd());
                    x >>= 1;
                }
                bits
            }

            /// Returns a `Vec` containing the bits of `self` in descending order: most- to least-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, it begins with `true`.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(0u8.to_bits_desc(), &[]);
            /// assert_eq!(2u16.to_bits_desc(), &[true, false]);
            /// assert_eq!(123u32.to_bits_desc(), &[true, true, true, true, false, true, true]);
            /// ```
            fn to_bits_desc(&self) -> Vec<bool> {
                let mut bits = Vec::new();
                if *self == 0 {
                    return bits;
                }
                bits.push(true);
                if *self == 1 {
                    return bits;
                }
                let mut mask = $t::power_of_two($t::WIDTH - LeadingZeros::leading_zeros(*self) - 2);
                while mask != 0 {
                    bits.push(*self & mask != 0);
                    mask >>= 1;
                }
                bits
            }

            /// Converts a slice of bits into a value. The input bits are in ascending order: least-
            /// to most-significant. The function panics if the input represents a number that can't
            /// fit in $t.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $t.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(u8::from_bits_asc(&[]), 0);
            /// assert_eq!(u16::from_bits_asc(&[false, true, false]), 2);
            /// assert_eq!(u32::from_bits_asc(&[true, true, false, true, true, true, true]), 123);
            /// ```
            fn from_bits_asc(bits: &[bool]) -> $t {
                let width = usize::exact_from($t::WIDTH);
                if bits.len() > width {
                    assert!(bits[width..].iter().all(|&bit| !bit));
                }
                let mut n = 0;
                let mut mask = 1;
                for &bit in bits {
                    if bit {
                        n |= mask;
                    }
                    mask <<= 1;
                }
                n
            }

            /// Converts a slice of bits into a value. The input bits are in descending order: most-
            /// to least-significant. The function panics if the input represents a number that
            /// can't fit in $t.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $t.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(u8::from_bits_desc(&[]), 0);
            /// assert_eq!(u16::from_bits_desc(&[false, true, false]), 2);
            /// assert_eq!(u32::from_bits_desc(&[true, true, true, true, false, true, true]), 123);
            /// ```
            fn from_bits_desc(mut bits: &[bool]) -> $t {
                let width = usize::exact_from($t::WIDTH);
                if bits.len() > width {
                    let (bits_lo, bits_hi) = bits.split_at(bits.len() - width);
                    assert!(bits_lo.iter().all(|&bit| !bit));
                    bits = bits_hi;
                }
                let mut n = 0;
                for &bit in bits {
                    n <<= 1;
                    if bit {
                        n |= 1;
                    }
                }
                n
            }
        }
    };
}

impl_bit_convertible_unsigned!(u8);
impl_bit_convertible_unsigned!(u16);
impl_bit_convertible_unsigned!(u32);
impl_bit_convertible_unsigned!(u64);
impl_bit_convertible_unsigned!(u128);
impl_bit_convertible_unsigned!(usize);

macro_rules! impl_bit_convertible_signed {
    ($t:ident, $u:ident) => {
        impl BitConvertible for $t {
            /// Returns a `Vec` containing the bits of `self` in ascending order: least- to most-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, the last bit is the sign
            /// bit: `false` if `self` is non-negative and `true` if `self` is negative.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(0i8.to_bits_asc(), &[]);
            /// assert_eq!(2i16.to_bits_asc(), &[false, true, false]);
            /// assert_eq!(
            ///     (-123i32).to_bits_asc(),
            ///     &[true, false, true, false, false, false, false, true]
            /// );
            /// ```
            fn to_bits_asc(&self) -> Vec<bool> {
                let mut bits = Vec::new();
                let mut x = *self;
                if *self >= 0 {
                    while x != 0 {
                        bits.push(x.get_bit(0));
                        x >>= 1;
                    }
                    if !bits.is_empty() {
                        bits.push(false);
                    }
                } else {
                    while x != -1 {
                        bits.push(x.get_bit(0));
                        x >>= 1;
                    }
                    bits.push(true);
                }
                bits
            }

            /// Returns a `Vec` containing the bits of `self` in ascending order: most- to least-
            /// significant. If `self` is 0, the `Vec` is empty; otherwise, the first bit is the
            /// sign bit: `false` if `self` is non-negative and `true` if `self` is negative.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(n)
            ///
            /// where n = `self.significant_bits()`
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(0i8.to_bits_desc(), &[]);
            /// assert_eq!(2i16.to_bits_desc(), &[false, true, false]);
            /// assert_eq!(
            ///     (-123i32).to_bits_desc(),
            ///     &[true, false, false, false, false, true, false, true]
            /// );
            /// ```
            fn to_bits_desc(&self) -> Vec<bool> {
                let mut bits = Vec::new();
                if *self >= 0 {
                    if *self == 0 {
                        return bits;
                    }
                    bits.push(false);
                    bits.push(true);
                    if *self == 1 {
                        return bits;
                    }
                    let mut mask =
                        $t::power_of_two($t::WIDTH - LeadingZeros::leading_zeros(*self) - 2);
                    while mask != 0 {
                        bits.push(*self & mask != 0);
                        mask >>= 1;
                    }
                } else {
                    bits.push(true);
                    if *self == -1 {
                        return bits;
                    }
                    bits.push(false);
                    if *self == -2 {
                        return bits;
                    }
                    let mut mask =
                        $t::power_of_two($t::WIDTH - LeadingZeros::leading_zeros(!*self) - 2);
                    while mask != 0 {
                        bits.push(*self & mask != 0);
                        mask >>= 1;
                    }
                }
                bits
            }

            /// Converts a slice of bits into a value. The input bits are in ascending order: least-
            /// to most-significant. The function panics if the input represents a number that can't
            /// fit in $t.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $t.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(i8::from_bits_asc(&[]), 0);
            /// assert_eq!(i16::from_bits_asc(&[false, true, false]), 2);
            /// assert_eq!(
            ///     i32::from_bits_asc(&[true, false, true, false, false, false, false, true]),
            ///     -123
            /// );
            /// ```
            fn from_bits_asc(bits: &[bool]) -> $t {
                if bits.is_empty() {
                    0
                } else if !*bits.last().unwrap() {
                    $t::exact_from($u::from_bits_asc(bits))
                } else {
                    let trailing_trues = bits.iter().rev().take_while(|&&bit| bit).count();
                    let significant_bits = bits.len() - trailing_trues;
                    assert!(significant_bits < usize::exact_from($t::WIDTH));
                    let mut u = !$u::low_mask(u64::exact_from(significant_bits));
                    let mut mask = 1;
                    for &bit in &bits[..significant_bits] {
                        if bit {
                            u |= mask;
                        }
                        mask <<= 1;
                    }
                    $t::wrapping_from(u)
                }
            }

            /// Converts a slice of bits into a value. The input bits are in ascending order: least-
            /// to most-significant. The function panics if the input represents a number that can't
            /// fit in $t.
            ///
            /// Time: worst case O(n)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// where n = `bits.len()`
            ///
            /// # Panics
            /// Panics if the bits represent a value that isn't representable by $t.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitConvertible;
            ///
            /// assert_eq!(i8::from_bits_desc(&[]), 0);
            /// assert_eq!(i16::from_bits_desc(&[false, true, false]), 2);
            /// assert_eq!(
            ///     i32::from_bits_desc(&[true, false, false, false, false, true, false, true]),
            ///     -123
            /// );
            /// ```
            fn from_bits_desc(bits: &[bool]) -> $t {
                if bits.is_empty() {
                    0
                } else if !bits[0] {
                    $t::exact_from($u::from_bits_desc(bits))
                } else {
                    let leading_trues = bits.iter().take_while(|&&bit| bit).count();
                    let significant_bits = u64::exact_from(bits.len() - leading_trues);
                    assert!(significant_bits < $t::WIDTH);
                    let mut mask = $u::power_of_two(significant_bits);
                    let mut u = !(mask - 1);
                    for &bit in &bits[leading_trues..] {
                        mask >>= 1;
                        if bit {
                            u |= mask;
                        }
                    }
                    $t::wrapping_from(u)
                }
            }
        }
    };
}

impl_bit_convertible_signed!(i8, u8);
impl_bit_convertible_signed!(i16, u16);
impl_bit_convertible_signed!(i32, u32);
impl_bit_convertible_signed!(i64, u64);
impl_bit_convertible_signed!(i128, u128);
impl_bit_convertible_signed!(isize, usize);
