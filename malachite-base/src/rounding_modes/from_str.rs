// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rounding_modes::RoundingMode::{self, *};
use alloc::string::String;
use alloc::string::ToString;
use core::str::FromStr;

impl FromStr for RoundingMode {
    type Err = String;

    /// Converts a string to a [`RoundingMode`].
    ///
    /// If the string does not represent a valid [`RoundingMode`], an `Err` is returned with the
    /// unparseable string.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ = `src.len()`.
    ///
    /// The worst case occurs when the input string is invalid and must be copied into an `Err`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::{self, *};
    /// use std::str::FromStr;
    ///
    /// assert_eq!(RoundingMode::from_str("Down"), Ok(Down));
    /// assert_eq!(RoundingMode::from_str("Up"), Ok(Up));
    /// assert_eq!(RoundingMode::from_str("Floor"), Ok(Floor));
    /// assert_eq!(RoundingMode::from_str("Ceiling"), Ok(Ceiling));
    /// assert_eq!(RoundingMode::from_str("Nearest"), Ok(Nearest));
    /// assert_eq!(RoundingMode::from_str("Exact"), Ok(Exact));
    /// assert_eq!(RoundingMode::from_str("abc"), Err("abc".to_string()));
    /// ```
    #[inline]
    fn from_str(src: &str) -> Result<RoundingMode, String> {
        match src {
            "Down" => Ok(Down),
            "Up" => Ok(Up),
            "Floor" => Ok(Floor),
            "Ceiling" => Ok(Ceiling),
            "Nearest" => Ok(Nearest),
            "Exact" => Ok(Exact),
            _ => Err(src.to_string()),
        }
    }
}
