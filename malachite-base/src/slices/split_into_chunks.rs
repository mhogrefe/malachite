/// This macro splits an immutable slice into adjacent immutable chunks. There are |`$xs_i`| + 1
/// chunks; the first |`$xs_i`| have length `$n`, and the remainder, which is assigned to
/// `$xs_last`, has length `$xs.len()` - `$n` * |`$xs_i`| (which may be longer than `$n`). If
/// `$xs.len()` < `$n` * |`$xs_i`|, the generated code panics at runtime.
#[macro_export]
macro_rules! split_into_chunks {
    ($xs: expr, $n: expr, [$($xs_i: ident),*], $xs_last: ident) => {
        let remainder = &$xs[..];
        let n = $n;
        $(
            let ($xs_i, remainder) = remainder.split_at(n);
        )*
        let $xs_last = remainder;
    }
}

/// This macro splits a mutable slice into adjacent mutable chunks. There are |`$xs_i`| + 1 chunks;
/// the first |`$xs_i`| have length `$n`, and the remainder, which is assigned to `$xs_last`, has
/// length `$xs.len()` - `$n` * |`$xs_i`| (which may be longer than `$n`). If
/// `$xs.len()` < `$n` * |`$xs_i`|, the generated code panics at runtime.
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
