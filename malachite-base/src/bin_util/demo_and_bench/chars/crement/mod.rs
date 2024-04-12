// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    char_to_contiguous_range::register(runner);
    contiguous_range_to_char::register(runner);
    crement::register(runner);
}

pub mod char_to_contiguous_range;
pub mod contiguous_range_to_char;
#[allow(clippy::module_inception)]
pub mod crement;
