// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    from_columns::register(runner);
    from_elements::register(runner);
    from_elements_column_major::register(runner);
    from_rows::register(runner);
    #[cfg(feature = "serde")]
    serde::register(runner);
    string::register(runner);
    to_elements::register(runner);
}

mod from_columns;
mod from_elements;
mod from_elements_column_major;
mod from_rows;
#[cfg(feature = "serde")]
mod serde;
mod string;
mod to_elements;
