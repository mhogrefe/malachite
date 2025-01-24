// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::comparison::traits::{Max, Min};
use crate::named::Named;

impl Min for bool {
    /// The minimum value of a [`bool`]: `false`.
    const MIN: bool = false;
}

impl Max for bool {
    /// The maximum value of a [`bool`]: `true`.
    const MAX: bool = true;
}

impl_named!(bool);
