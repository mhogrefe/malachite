// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Until the next release, the traits that moved to `rational::number_theory::traits` are also
// re-exported here, so that paths through this module keep resolving. These re-exports will then be
// removed.
pub use crate::rational::number_theory::traits::*;
