/// This trait defines two functions, `increment` and `decrement`, that undo each other's effect,
/// and have the property that if a < b, b is reachable from a by a finite number of increments and
/// a is reachable from b by a finite number of decrements. If the type has a maximum value,
/// incrementing it should panic; if it has a minimum value, decrementing it should panic.
pub trait Crementable: PartialEq + PartialOrd {
    /// Changes `self` to the smallest value greater than its old value. Panics if no greater value
    /// exists.
    fn increment(&mut self);

    /// Changes `self` to the greatest value smaller than its old value. Panics if no smaller value
    /// exists.
    fn decrement(&mut self);
}
