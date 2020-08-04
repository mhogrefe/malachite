use comparison::traits::{Max, Min};
use named::Named;

impl Min for bool {
    /// The minimum value of a `bool`: $\text{false}$.
    const MIN: bool = false;
}

impl Max for bool {
    /// The maximum value of a `bool`: $\text{true}$.
    const MAX: bool = true;
}

impl_named!(bool);
