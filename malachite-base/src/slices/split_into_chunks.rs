//TODO

// This doesn't use `chunks_exact` because sometimes `xs_last` is longer than `n`.
#[macro_export]
macro_rules! split_into_chunks {
    ($xs: expr, $n: expr, [$($xs_i: ident),*], $xs_last: ident) => {
        let remainder = &$xs;
        let n = $n;
        $(
            let ($xs_i, remainder) = remainder.split_at(n);
        )*
        let $xs_last = remainder;
    }
}

// This doesn't use `chunks_exact_mut` because sometimes `xs_last` is longer than `n`.
#[macro_export]
macro_rules! split_into_chunks_mut {
    ($xs: expr, $n: expr, [$($xs_i: ident),*], $xs_last: ident) => {
        let remainder = &mut $xs[..];
        let n = $n;
        $(
            let ($xs_i, remainder) = remainder.split_at_mut(n);
        )*
        let $xs_last = remainder;
    }
}
