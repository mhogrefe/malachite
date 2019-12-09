use malachite_base::comparison::Max;
use malachite_base::limbs::limbs_leading_zero_limbs;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{CheckedFrom, WrappingFrom};

use integer::Integer;
use natural::arithmetic::add::limbs_slice_add_limb_in_place;
use natural::conversion::to_limbs::LimbIterator;
use natural::logic::not::limbs_not_in_place;
use natural::Natural;
use platform::Limb;

/// Given the limbs of the absolute value of an `Integer`, in ascending order, returns the two's
/// complement limbs. The input limbs should not be all zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::conversion::to_twos_complement_limbs::limbs_twos_complement;
///
/// assert_eq!(limbs_twos_complement(&[1, 2, 3]), &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc]);
/// assert_eq!(limbs_twos_complement(&[0xffff_ffff, 0xffff_fffd, 0xffff_fffc]), &[1, 2, 3]);
/// ```
pub fn limbs_twos_complement(limbs: &[Limb]) -> Vec<Limb> {
    let i = limbs_leading_zero_limbs(limbs);
    let mut result_limbs = vec![0; i];
    if i != limbs.len() {
        result_limbs.push(limbs[i].wrapping_neg());
        for limb in &limbs[i + 1..] {
            result_limbs.push(!limb);
        }
    }
    result_limbs
}

/// Given the limbs of a non-negative `Integer`, in ascending order, checks whether the most
/// significant bit is `false`; if it isn't, appends an extra zero bit. This way the `Integer`'s
/// non-negativity is preserved in its limbs.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
///
/// let mut limbs = vec![1, 2, 3];
/// limbs_maybe_sign_extend_non_negative_in_place(&mut limbs);
/// assert_eq!(limbs, &[1, 2, 3]);
///
/// let mut limbs = vec![1, 2, 0xffff_ffff];
/// limbs_maybe_sign_extend_non_negative_in_place(&mut limbs);
/// assert_eq!(limbs, &[1, 2, 0xffff_ffff, 0]);
/// ```
pub fn limbs_maybe_sign_extend_non_negative_in_place(limbs: &mut Vec<Limb>) {
    if !limbs.is_empty() && limbs.last().unwrap().get_highest_bit() {
        // Sign-extend with an extra 0 limb to indicate a positive Integer
        limbs.push(0);
    }
}

/// Given the limbs of the absolute value of an `Integer`, in ascending order, converts the limbs to
/// two's complement. Returns whether there is a carry left over from the two's complement
/// conversion process.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
///
/// let mut limbs = &mut [1, 2, 3];
/// assert!(!limbs_twos_complement_in_place(limbs));
/// assert_eq!(limbs, &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc]);
///
/// let mut limbs = &mut [0, 0, 0];
/// assert!(limbs_twos_complement_in_place(limbs));
/// assert_eq!(limbs, &[0, 0, 0]);
/// ```
pub fn limbs_twos_complement_in_place(limbs: &mut [Limb]) -> bool {
    limbs_not_in_place(limbs);
    limbs_slice_add_limb_in_place(limbs, 1)
}

/// Given the limbs of the absolute value of a negative `Integer`, in ascending order, converts the
/// limbs to two's complement and checks whether the most significant bit is `true`; if it isn't,
/// appends an extra `Limb::MAX` bit. This way the `Integer`'s negativity is preserved in its limbs.
/// The limbs cannot be empty or contain only zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` contains only zeros.
///
/// # Examples
/// ```
/// use malachite_nz::integer::conversion::to_twos_complement_limbs::*;
///
/// let mut limbs = vec![1, 2, 3];
/// limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut limbs);
/// assert_eq!(limbs, &[0xffff_ffff, 0xffff_fffd, 0xffff_fffc]);
///
/// let mut limbs = vec![0, 0xffff_ffff];
/// limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut limbs);
/// assert_eq!(limbs, &[0, 1, 0xffff_ffff]);
/// ```
pub fn limbs_twos_complement_and_maybe_sign_extend_negative_in_place(limbs: &mut Vec<Limb>) {
    assert!(!limbs_twos_complement_in_place(limbs));
    if !limbs.last().unwrap().get_highest_bit() {
        // Sign-extend with an extra !0 limb to indicate a negative Integer
        limbs.push(Limb::MAX);
    }
}

/// A double-ended iterator over the two's complement limbs of the negative of a `Natural`. The
/// forward order is ascending (least-significant first). There may be at most one implicit
/// most-significant `Limb::MAX` limb.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NegativeLimbIterator<'a> {
    pub(crate) limbs: LimbIterator<'a>,
    first_nonzero_index: Option<usize>,
}

impl<'a> NegativeLimbIterator<'a> {
    fn get(&self, index: usize) -> Limb {
        if index >= self.limbs.len() {
            // We're indexing into the infinite suffix of Limb::MAXs
            Limb::MAX
        } else {
            for i in 0..index {
                if self.limbs[i] != 0 {
                    return !self.limbs[index];
                }
            }
            self.limbs[index].wrapping_neg()
        }
    }
}

impl<'a> Iterator for NegativeLimbIterator<'a> {
    type Item = Limb;

    /// A function to iterate through the two's complement limbs of the negative of a `Natural` in
    /// ascending order (least-significant first).
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn next(&mut self) -> Option<Limb> {
        let previous_i = u64::wrapping_from(self.limbs.i);
        self.limbs.next().map(|limb| {
            if let Some(first_nonzero_index) = self.first_nonzero_index {
                if previous_i <= u64::wrapping_from(first_nonzero_index) {
                    limb.wrapping_neg()
                } else {
                    !limb
                }
            } else {
                if limb != 0 {
                    self.first_nonzero_index = Some(usize::checked_from(previous_i).unwrap());
                }
                limb.wrapping_neg()
            }
        })
    }

    /// A function that returns the length of the negative limbs iterator; that is, the `Natural`'s
    /// negative limb count (this is the same as its limb count). The format is (lower bound,
    /// Option<upper bound>), but in this case it's trivial to always have an exact bound.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.limbs.limb_count, Some(self.limbs.limb_count))
    }
}

impl<'a> DoubleEndedIterator for NegativeLimbIterator<'a> {
    /// A function to iterate through the two's complement limbs of the negative of a `Natural` in
    /// descending order (most-significant first). This is worst-case linear since the first
    /// `next_back` call needs to determine the index of the least-significant nonzero limb.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    fn next_back(&mut self) -> Option<Limb> {
        let previous_j = self.limbs.j;
        self.limbs.next_back().map(|limb| {
            if self.first_nonzero_index.is_none() {
                let mut i = 0;
                while self.limbs[i] == 0 {
                    i += 1;
                }
                self.first_nonzero_index = Some(i);
            }
            let first_nonzero_index = self.first_nonzero_index.unwrap();
            if previous_j <= u64::wrapping_from(first_nonzero_index) {
                limb.wrapping_neg()
            } else {
                !limb
            }
        })
    }
}

trait SignExtendedLimbIterator: DoubleEndedIterator<Item = Limb> {
    const EXTENSION: Limb;

    fn needs_sign_extension(&self) -> bool;

    fn iterate_forward(&mut self, extension_checked: &mut bool) -> Option<Limb> {
        let next = self.next();
        if next.is_none() {
            if *extension_checked {
                None
            } else {
                *extension_checked = true;
                if self.needs_sign_extension() {
                    Some(Self::EXTENSION)
                } else {
                    None
                }
            }
        } else {
            next
        }
    }

    fn iterate_backward(&mut self, extension_checked: &mut bool) -> Option<Limb> {
        if !*extension_checked {
            *extension_checked = true;
            if self.needs_sign_extension() {
                return Some(Self::EXTENSION);
            }
        }
        self.next_back()
    }
}

impl<'a> SignExtendedLimbIterator for LimbIterator<'a> {
    const EXTENSION: Limb = 0;

    fn needs_sign_extension(&self) -> bool {
        self[self.limb_count - 1].get_highest_bit()
    }
}

impl<'a> SignExtendedLimbIterator for NegativeLimbIterator<'a> {
    const EXTENSION: Limb = Limb::MAX;

    fn needs_sign_extension(&self) -> bool {
        let mut i = 0;
        while self.limbs[i] == 0 {
            i += 1;
        }
        let last_limb_index = self.limbs.limb_count - 1;
        let last_limb = self.limbs[last_limb_index];
        let twos_complement_limb = if i == last_limb_index {
            last_limb.wrapping_neg()
        } else {
            !last_limb
        };
        !twos_complement_limb.get_highest_bit()
    }
}

/// A double-ended iterator over the twos-complement limbs of an `Integer`. The forward order is
/// ascending (least-significant first). The most significant bit of the most significant limb
/// corresponds to the sign of the `Integer`; `false` for non-negative and `true` for negative. This
/// means that there may be a single most-significant sign-extension limb that is 0 or `Limb::MAX`.
///
/// This struct also supports retrieving limbs by index. This functionality is completely
/// independent of the iterator's state. Indexing the implicit leading limbs is allowed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TwosComplementLimbIterator<'a> {
    Zero,
    Positive(LimbIterator<'a>, bool),
    Negative(NegativeLimbIterator<'a>, bool),
}

impl<'a> TwosComplementLimbIterator<'a> {
    /// A function to retrieve twos-complement limbs by index. Indexing at or above the limb count
    /// returns zero or `Limb::MAX` limbs, depending on the sign of `self`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    /// use std::u32;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.twos_complement_limbs().get(0), 0);
    ///
    ///     // 2^64 - 10^12 = 4294967063 * 2^32 + 727379968
    ///     let negative_trillion = -Integer::trillion();
    ///     let limbs = negative_trillion.twos_complement_limbs();
    ///     assert_eq!(limbs.get(0), 727379968);
    ///     assert_eq!(limbs.get(1), 4294967063);
    ///     assert_eq!(limbs.get(2), u32::MAX);
    ///     assert_eq!(limbs.get(100), u32::MAX);
    /// }
    /// ```
    pub fn get(&self, index: usize) -> Limb {
        match *self {
            TwosComplementLimbIterator::Zero => 0,
            TwosComplementLimbIterator::Positive(ref limbs, _) => limbs[index],
            TwosComplementLimbIterator::Negative(ref limbs, _) => limbs.get(index),
        }
    }
}

impl<'a> Iterator for TwosComplementLimbIterator<'a> {
    type Item = Limb;

    /// A function to iterate through the twos-complement limbs of an `Integer` in ascending order
    /// (least-significant first). The last limb may be a sign-extension limb.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.twos_complement_limbs().next(), None);
    ///
    ///     // 2^64 - 10^12 = 4294967063 * 2^32 + 727379968
    ///     let negative_trillion = -Integer::trillion();
    ///     let mut limbs = negative_trillion.twos_complement_limbs();
    ///     assert_eq!(limbs.next(), Some(727379968));
    ///     assert_eq!(limbs.next(), Some(4294967063));
    ///     assert_eq!(limbs.next(), None);
    /// }
    /// ```
    fn next(&mut self) -> Option<Limb> {
        match *self {
            TwosComplementLimbIterator::Zero => None,
            TwosComplementLimbIterator::Positive(ref mut limbs, ref mut extension_checked) => {
                limbs.iterate_forward(extension_checked)
            }
            TwosComplementLimbIterator::Negative(ref mut limbs, ref mut extension_checked) => {
                limbs.iterate_forward(extension_checked)
            }
        }
    }
}

impl<'a> DoubleEndedIterator for TwosComplementLimbIterator<'a> {
    /// A function to iterate through the twos-complement limbs of an `Integer` in descending order
    /// (most-significant first). The first limb may be a sign-extension limb.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.twos_complement_limbs().next_back(), None);
    ///
    ///     // 2^64 - 10^12 = 4294967063 * 2^32 + 727379968
    ///     let negative_trillion = -Integer::trillion();
    ///     let mut limbs = negative_trillion.twos_complement_limbs();
    ///     assert_eq!(limbs.next_back(), Some(4294967063));
    ///     assert_eq!(limbs.next_back(), Some(727379968));
    ///     assert_eq!(limbs.next_back(), None);
    /// }
    /// ```
    fn next_back(&mut self) -> Option<Limb> {
        match *self {
            TwosComplementLimbIterator::Zero => None,
            TwosComplementLimbIterator::Positive(ref mut limbs, ref mut extension_checked) => {
                limbs.iterate_backward(extension_checked)
            }
            TwosComplementLimbIterator::Negative(ref mut limbs, ref mut extension_checked) => {
                limbs.iterate_backward(extension_checked)
            }
        }
    }
}

impl Integer {
    /// Returns the limbs of an `Integer`, in ascending order,
    /// so that less significant limbs have lower indices in the output vector. The limbs are in
    /// two's complement, and the most significant bit of the limbs indicates the sign; if the bit
    /// is zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// trailing zero limbs if the `Integer` is positive or trailing !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This function borrows `self`. If taking ownership of `self` is possible,
    /// `into_twos_complement_limbs_asc` is more efficient.
    ///
    /// This method is more efficient than `to_twos_complement_limbs_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.to_twos_complement_limbs_asc().is_empty());
    ///     assert_eq!(Integer::from(123).to_twos_complement_limbs_asc(), &[123]);
    ///     assert_eq!(Integer::from(-123).to_twos_complement_limbs_asc(), &[4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().to_twos_complement_limbs_asc(), &[3567587328, 232]);
    ///     assert_eq!((-Integer::trillion()).to_twos_complement_limbs_asc(),
    ///         &[727379968, 4294967063]);
    /// }
    /// ```
    pub fn to_twos_complement_limbs_asc(&self) -> Vec<Limb> {
        let mut limbs = self.abs.to_limbs_asc();
        if self.sign {
            limbs_maybe_sign_extend_non_negative_in_place(&mut limbs);
        } else {
            limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut limbs);
        }
        limbs
    }

    /// Returns the limbs of an `Integer`, in descending order, so
    /// that less significant limbs have higher indices in the output vector. The limbs are in two's
    /// complement, and the most significant bit of the limbs indicates the sign; if the bit is
    /// zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// leading zero limbs if the `Integer` is non-negative or leading !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This is similar to how BigIntegers in Java are represented.
    ///
    /// This function borrows `self`. If taking ownership of `self` is possible,
    /// `into_twos_complement_limbs_desc` is more efficient.
    ///
    /// This method is less efficient than `to_twos_complement_limbs_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.to_twos_complement_limbs_desc().is_empty());
    ///     assert_eq!(Integer::from(123).to_twos_complement_limbs_desc(), &[123]);
    ///     assert_eq!(Integer::from(-123).to_twos_complement_limbs_desc(), &[4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().to_twos_complement_limbs_desc(), &[232, 3567587328]);
    ///     assert_eq!((-Integer::trillion()).to_twos_complement_limbs_desc(),
    ///         &[4294967063, 727379968]);
    /// }
    /// ```
    pub fn to_twos_complement_limbs_desc(&self) -> Vec<Limb> {
        let mut limbs = self.to_twos_complement_limbs_asc();
        limbs.reverse();
        limbs
    }

    /// Returns the limbs of an `Integer`, in ascending order,
    /// so that less significant limbs have lower indices in the output vector. The limbs are in
    /// two's complement, and the most significant bit of the limbs indicates the sign; if the bit
    /// is zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// trailing zero limbs if the `Integer` is positive or trailing !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This function takes ownership of `self`. If it's necessary to borrow `self` instead, use
    /// `to_twos_complement_limbs_asc`.
    ///
    /// This method is more efficient than `into_twos_complement_limbs_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.into_twos_complement_limbs_asc().is_empty());
    ///     assert_eq!(Integer::from(123).into_twos_complement_limbs_asc(), &[123]);
    ///     assert_eq!(Integer::from(-123).into_twos_complement_limbs_asc(), &[4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().into_twos_complement_limbs_asc(), &[3567587328, 232]);
    ///     assert_eq!((-Integer::trillion()).into_twos_complement_limbs_asc(),
    ///         &[727379968, 4294967063]);
    /// }
    /// ```
    pub fn into_twos_complement_limbs_asc(self) -> Vec<Limb> {
        let mut limbs = self.abs.into_limbs_asc();
        if self.sign {
            limbs_maybe_sign_extend_non_negative_in_place(&mut limbs);
        } else {
            limbs_twos_complement_and_maybe_sign_extend_negative_in_place(&mut limbs);
        }
        limbs
    }

    /// Returns the limbs of an `Integer`, in descending order, so
    /// that less significant limbs have higher indices in the output vector. The limbs are in two's
    /// complement, and the most significant bit of the limbs indicates the sign; if the bit is
    /// zero, the `Integer` is positive, and if the bit is one it is negative. There are no
    /// leading zero limbs if the `Integer` is non-negative or leading !0 limbs if `Integer` is
    /// negative, except as necessary to include the correct sign bit. Zero is a special case: it
    /// contains no limbs.
    ///
    /// This is similar to how BigIntegers in Java are represented.
    ///
    /// This function takes ownership of `self`. If it's necessary to borrow `self` instead, use
    /// `to_twos_complement_limbs_desc`.
    ///
    /// This method is less efficient than `into_twos_complement_limbs_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.into_twos_complement_limbs_desc().is_empty());
    ///     assert_eq!(Integer::from(123).into_twos_complement_limbs_desc(), &[123]);
    ///     assert_eq!(Integer::from(-123).into_twos_complement_limbs_desc(), &[4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().into_twos_complement_limbs_desc(),
    ///         &[232, 3567587328]);
    ///     assert_eq!((-Integer::trillion()).into_twos_complement_limbs_desc(),
    ///         &[4294967063, 727379968]);
    /// }
    /// ```
    pub fn into_twos_complement_limbs_desc(self) -> Vec<Limb> {
        let mut limbs = self.into_twos_complement_limbs_asc();
        limbs.reverse();
        limbs
    }

    /// Returns a double-ended iterator over the twos-complement limbs of an `Integer`. The forward
    /// order is ascending, so that less significant limbs appear first. There may be a
    /// most-significant sign-extension limb.
    ///
    /// If it's necessary to get a `Vec` of all the twos_complement limbs, consider using
    /// `to_twos_complement_limbs_asc`,
    /// `to_twos_complement_limbs_desc`, `into_twos_complement_limbs_asc`, or
    /// `into_twos_complement_limbs_desc` instead.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert!(Integer::ZERO.twos_complement_limbs().next().is_none());
    ///     assert_eq!(Integer::from(123).twos_complement_limbs().collect::<Vec<u32>>(), &[123]);
    ///     assert_eq!(Integer::from(-123).twos_complement_limbs().collect::<Vec<u32>>(),
    ///         &[4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().twos_complement_limbs().collect::<Vec<u32>>(),
    ///         &[3567587328, 232]);
    ///     // Sign-extension for a non-negative `Integer`
    ///     assert_eq!(Integer::from(0xffff_ffffu32).twos_complement_limbs().collect::<Vec<u32>>(),
    ///         &[0xffff_ffff, 0]);
    ///     assert_eq!((-Integer::trillion()).twos_complement_limbs().collect::<Vec<u32>>(),
    ///         &[727379968, 4294967063]);
    ///     // Sign-extension for a negative `Integer`
    ///     assert_eq!((-Integer::from(0xffff_ffffu32)).twos_complement_limbs()
    ///         .collect::<Vec<u32>>(), &[1, 0xffff_ffff]);
    ///
    ///     assert!(Integer::ZERO.twos_complement_limbs().rev().next().is_none());
    ///     assert_eq!(Integer::from(123).twos_complement_limbs().rev().collect::<Vec<u32>>(),
    ///         &[123]);
    ///     assert_eq!(Integer::from(-123).twos_complement_limbs().rev().collect::<Vec<u32>>(),
    ///         &[4294967173]);
    ///     // 10^12 = 232 * 2^32 + 3567587328
    ///     assert_eq!(Integer::trillion().twos_complement_limbs().rev().collect::<Vec<u32>>(),
    ///         &[232, 3567587328]);
    ///     // Sign-extension for a non-negative `Integer`
    ///     assert_eq!(Integer::from(0xffff_ffffu32).twos_complement_limbs().rev()
    ///         .collect::<Vec<u32>>(), &[0, 0xffff_ffff]);
    ///     assert_eq!((-Integer::trillion()).twos_complement_limbs().rev().collect::<Vec<u32>>(),
    ///         &[4294967063, 727379968]);
    ///     // Sign-extension for a negative `Integer`
    ///     assert_eq!((-Integer::from(0xffff_ffffu32)).twos_complement_limbs().rev()
    ///         .collect::<Vec<u32>>(), &[0xffff_ffff, 1]);
    /// }
    /// ```
    pub fn twos_complement_limbs(&self) -> TwosComplementLimbIterator {
        if *self == 0 as Limb {
            TwosComplementLimbIterator::Zero
        } else if self.sign {
            TwosComplementLimbIterator::Positive(self.abs.limbs(), false)
        } else {
            TwosComplementLimbIterator::Negative(self.abs.negative_limbs(), false)
        }
    }
}

impl Natural {
    /// Returns a double-ended iterator over the two's complement limbs of the negative of a
    /// `Natural`. The forward order is ascending, so that less significant limbs appear first.
    /// There may be at most one trailing `Limb::MAX` limb going forward, or leading `Limb::MAX`
    /// limb going backward. The `Natural` cannot be zero.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    fn negative_limbs(&self) -> NegativeLimbIterator {
        assert_ne!(*self, 0 as Limb, "Cannot get negative limbs of 0.");
        NegativeLimbIterator {
            limbs: self.limbs(),
            first_nonzero_index: None,
        }
    }
}
