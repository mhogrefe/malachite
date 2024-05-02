// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Computes the minimum of a list of expressions.
///
/// The list must be nonempty, the expressions must all have the same type, and that type must
/// implement [`Ord`]. Each expression is only evaluated once.
///
/// # Examples
/// ```
/// use malachite_base::min;
///
/// assert_eq!(min!(3), 3);
/// assert_eq!(min!(3, 1), 1);
/// assert_eq!(min!(3, 1, 4), 1);
/// ```
#[macro_export]
macro_rules! min {
    ($first: expr $(,$next: expr)*) => {
        {
            let mut min = $first;
            $(
                let next = $next;
                if next < min {
                    min = next;
                }
            )*
            min
        }
    };
}

/// Computes the maximum of a list of expressions.
///
/// The list must be nonempty, the expressions must all have the same type, and that type must
/// implement [`Ord`]. Each expression is only evaluated once.
///
/// # Examples
/// ```
/// use malachite_base::max;
///
/// assert_eq!(max!(3), 3);
/// assert_eq!(max!(3, 1), 3);
/// assert_eq!(max!(3, 1, 4), 4);
/// ```
#[macro_export]
macro_rules! max {
    ($first: expr $(,$next: expr)*) => {
        {
            let mut max = $first;
            $(
                let next = $next;
                if next > max {
                    max = next;
                }
            )*
            max
        }
    };
}
