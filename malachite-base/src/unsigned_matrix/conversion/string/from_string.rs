// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use alloc::vec::Vec;
use core::str::FromStr;

// Reads a dimension written in decimal digits, which may have leading zeros but no sign.
fn parse_dimension(s: &str) -> Option<usize> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    usize::from_str(s).ok()
}

impl<T: PrimitiveUnsigned> FromStr for UnsignedMatrix<T> {
    type Err = ();

    /// Converts a string to an [`UnsignedMatrix`].
    ///
    /// This reads back what [`Display`](core::fmt::Display) writes: the elements of each row
    /// separated by `", "`, the rows separated by `"; "`, and the whole enclosed in square
    /// brackets, as in `[1, 2; 3, 4]`. Every row must have the same number of elements. A matrix
    /// with no rows or no columns is written `[]_{h,w}`, where `h` and `w` are its height and width
    /// in decimal, at least one of which must be zero; `[]` alone is rejected, since it does not
    /// say which empty matrix it is.
    ///
    /// Each element is read the way `T::from_str` reads one, so it may have leading zeros and a
    /// single leading `'+'`, but no `'-'`, and must fit in a `T`. The dimensions may have leading
    /// zeros but no sign. Any other spacing, such as `[1,2]` or `[1, 2;3, 4]`, is rejected, as is
    /// an empty element or row.
    ///
    /// If the string does not represent an [`UnsignedMatrix`], an `Err` is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((m.height(), m.width()), (2, 3));
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[]_{4,0}").unwrap();
    /// assert_eq!((m.height(), m.width()), (4, 0));
    ///
    /// // An element may be written as `T::from_str` accepts it.
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_str("[0, +2; 007, 1]")
    ///         .unwrap()
    ///         .to_string(),
    ///     "[0, 2; 7, 1]"
    /// );
    ///
    /// assert!(UnsignedMatrix::<u64>::from_str("[-1]").is_err());
    /// assert!(UnsignedMatrix::<u8>::from_str("[256]").is_err());
    /// assert!(UnsignedMatrix::<u64>::from_str("[]").is_err());
    /// assert!(UnsignedMatrix::<u64>::from_str("[]_{2,3}").is_err());
    /// assert!(UnsignedMatrix::<u64>::from_str("[1, 2; 3]").is_err());
    /// assert!(UnsignedMatrix::<u64>::from_str("[1,2]").is_err());
    /// assert!(UnsignedMatrix::<u64>::from_str("[1, 2;3, 4]").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, ()> {
        if let Some(dimensions) = s.strip_prefix("[]_{").and_then(|s| s.strip_suffix('}')) {
            let (height, width) = dimensions.split_once(',').ok_or(())?;
            let height = parse_dimension(height).ok_or(())?;
            let width = parse_dimension(width).ok_or(())?;
            if height != 0 && width != 0 {
                return Err(());
            }
            return Ok(Self {
                elements: Vec::new(),
                height,
                width,
            });
        }
        let inner = s
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .ok_or(())?;
        if inner.is_empty() {
            return Err(());
        }
        let mut elements = Vec::new();
        let mut height = 0;
        let mut width = None;
        for row in inner.split("; ") {
            let old_len = elements.len();
            for x in row.split(", ") {
                elements.push(T::from_str(x).map_err(|_| ())?);
            }
            let row_width = elements.len() - old_len;
            if *width.get_or_insert(row_width) != row_width {
                return Err(());
            }
            height += 1;
        }
        Ok(Self {
            elements,
            height,
            width: width.unwrap(),
        })
    }
}
