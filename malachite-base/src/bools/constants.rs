use comparison::traits::{Max, Min};

/// The minimum value of a `bool`, false.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Min for bool {
    const MIN: bool = false;
}

/// The maximum value of a `bool`, true.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
impl Max for bool {
    const MAX: bool = true;
}
