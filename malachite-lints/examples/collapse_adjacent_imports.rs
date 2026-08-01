// The lint fires on the shape of the imports alone; nothing here needs to be used.
#![allow(unused_imports)]

// Two imports differing only in the final component: flagged.
use malachite_float::ComparableFloat;
use malachite_float::Float;
// A run of three, one of them renamed: flagged once, as a single run.
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::traits::Two;
use malachite_base::num::basic::traits::Zero as Nought;
// A brace group counts as a final component, so a lone import beside one is flagged.
use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::arithmetic::traits::{Pow, Sign};
// Different prefixes: fine.
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
// A glob subsumes its siblings rather than merging with them: fine.
use malachite_q::Rational;
use malachite_q::*;
// An attribute applies to the whole item, so it cannot move into a shared brace group: fine.
#[allow(unused_imports)]
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;

mod visibility {
    // `pub use` and `use` cannot share a brace group: fine.
    pub use malachite_base::num::conversion::traits::ExactFrom;
    use malachite_base::num::conversion::traits::WrappingFrom;
    // Two imports of the same visibility, though: flagged.
    pub use malachite_base::num::logic::traits::BitAccess;
    pub use malachite_base::num::logic::traits::SignificantBits;
}

fn main() {
    // Imports inside a block are scanned the same way: flagged.
    use malachite_base::num::float::FmtRyuString;
    use malachite_base::num::float::NiceFloat;
    // A statement between two imports means they are not adjacent: fine.
    use malachite_base::num::basic::integers::PrimitiveInt;
    let _ = 0u8;
    use malachite_base::num::basic::integers::USIZE_IS_U32;
}
