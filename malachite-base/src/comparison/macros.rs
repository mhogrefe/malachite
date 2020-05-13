/// This macro computes the minimum of a list of expressions. The list must be nonempty, the
/// expressions must all have the same type, and that type must implement `Ord`. Each expression is
/// only evaluated once.
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

/// This macro computes the maximum of a list of expressions. The list must be nonempty, the
/// expressions must all have the same type, and that type must implement `Ord`. Each expression is
/// only evaluated once.
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
