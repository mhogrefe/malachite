use unions::{Union2, Union3, Union4, Union5, Union6, Union7, Union8};

macro_rules! exhaustive_unions {
    (
        $union: ident,
        $lex_struct: ident,
        $exhaustive_struct: ident,
        $lex_fn: ident,
        $exhaustive_fn: ident,
        $n: expr,
        $([$i: expr, $t: ident, $it: ident, $variant: ident, $xs: ident, $xs_done:ident]),*
    ) => {
        /// Generates all $n$-unions with elements from $n$ iterators, in lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators. All of
        /// the first variant's elements are generated first, followed by the second variant's
        /// elements, and so on.
        ///
        /// This `struct` is created by `lex_union[n]s`. See its documentation for more.
        #[derive(Clone, Debug)]
        pub struct $lex_struct<$($t, $it: Iterator<Item=$t>),*> {
            i: u64,
            $($xs: $it,)*
        }

        impl<$($t, $it: Iterator<Item=$t>),*> Iterator for $lex_struct<$($t, $it),*> {
            type Item = $union<$($t),*>;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match self.i {
                        $(
                            $i => {
                                let next = self.$xs.next().map($union::$variant);
                                if next.is_some() {
                                    return next;
                                }
                            },
                        )*
                        _ => return None,
                    }
                    self.i += 1;
                }
            }
        }

        /// Generates all $n$-unions with elements from $n$ iterators, in lexicographic order.
        ///
        /// The order is lexicographic with respect to the order of the element iterators. All of
        /// the first variant's elements are generated first, followed by the second variant's
        /// elements, and so on. This means that all of the iterators, except possibly the last one,
        /// must be finite. For functions that support multiple infinite element iterators, try
        /// `exhaustive_union[n]s`.
        ///
        /// If the last iterator is finite, the output length is the sum of the lengths of all the
        /// input iterators. If the last iterator is infinite, the output is also infinite.
        ///
        /// If all of the input iterators are empty, the output is also empty.
        ///
        /// # Complexity per iteration
        ///
        /// $$
        /// T(i, n) = O(\max(T_1(i, n), T_2(i, n), \ldots, T_{n-1}(i, n)))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(\max(M_1(i, n), M_2(i, n), \ldots, M_{n-1}(i, n)))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the number of input iterators, and
        /// $T_j$ and $M_j$ are the time and additional memory functions of the $j$th input
        /// iterator.
        ///
        /// # Examples
        ///
        /// See the documentation of the `unions::exhaustive` module.
        #[inline]
        pub fn $lex_fn<$($t, $it: Iterator<Item=$t>),*>($($xs: $it),*) ->
                $lex_struct<$($t, $it),*> {
            $lex_struct {
                i: 0,
                $($xs,)*
            }
        }

        /// Generates all $n$-unions with elements from $n$ iterators.
        ///
        /// This `struct` is created by `exhaustive_union[n]s`. See its documentation for more.
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct<$($t, $it: Iterator<Item=$t>),*> {
            done: bool,
            i: u64,
            $(
                $xs: $it,
                $xs_done: bool,
            )*
        }

        impl<$($t, $it: Iterator<Item=$t>),*> Iterator for $exhaustive_struct<$($t, $it),*> {
            type Item = $union<$($t),*>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.done {
                    None
                } else {
                    let original_i = self.i;
                    loop {
                        let mut next = None;
                        match self.i {
                            $(
                                $i => if !self.$xs_done {
                                    next = self.$xs.next().map($union::$variant);
                                    self.$xs_done = next.is_none();
                                },
                            )*
                            _ => unreachable!(),
                        }
                        self.i += 1;
                        if self.i == $n {
                            self.i = 0;
                        }
                        if next.is_some() {
                            return next;
                        }
                        if self.i == original_i {
                            self.done = true;
                            return None;
                        }
                    }
                }
            }
        }

        /// Generates all $n$-unions with elements from $n$ iterators.
        ///
        /// The input iterators are advanced in a round-robin fashion. First an element from the
        /// first variant's iterator is selected, followed by an element from the second variant's
        /// iterator, and so on until an element has been selected from each iterator. Then another
        /// element from the first iterator is selected, etc. Iterators that have been exhausted are
        /// skipped. `exhaustive_union2s` behaves just like `Itertools::interleave`.
        ///
        /// If all input iterators are finite, the output length is the sum of the lengths of the
        /// iterators. If any iterator is infinite, the output is also infinite.
        ///
        /// If all of the input iterators are empty, the output is also empty.
        ///
        /// # Complexity per iteration
        ///
        /// $$
        /// T(i, n) = O(n + \max(T_1(i, n), T_2(i, n), \ldots, T_{n-1}(i, n)))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + \max(M_1(i, n), M_2(i, n), \ldots, M_{n-1}(i, n)))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the number of input iterators, and
        /// $T_j$ and $M_j$ are the time and additional memory functions of the $j$th input
        /// iterator.
        ///
        /// # Examples
        ///
        /// See the documentation of the `unions::exhaustive` module.
        #[inline]
        pub fn $exhaustive_fn<$($t, $it: Iterator<Item=$t>),*>($($xs: $it),*) ->
                $exhaustive_struct<$($t, $it),*> {
            $exhaustive_struct {
                done: false,
                i: 0,
                $(
                    $xs,
                    $xs_done: false,
                )*
            }
        }
    }
}

exhaustive_unions!(
    Union2,
    LexUnion2s,
    ExhaustiveUnion2s,
    lex_union2s,
    exhaustive_union2s,
    2,
    [0, X, I, A, xs, xs_done],
    [1, Y, J, B, ys, ys_done]
);
exhaustive_unions!(
    Union3,
    LexUnion3s,
    ExhaustiveUnion3s,
    lex_union3s,
    exhaustive_union3s,
    3,
    [0, X, I, A, xs, xs_done],
    [1, Y, J, B, ys, ys_done],
    [2, Z, K, C, zs, zs_done]
);
exhaustive_unions!(
    Union4,
    LexUnion4s,
    ExhaustiveUnion4s,
    lex_union4s,
    exhaustive_union4s,
    4,
    [0, X, I, A, xs, xs_done],
    [1, Y, J, B, ys, ys_done],
    [2, Z, K, C, zs, zs_done],
    [3, W, L, D, ws, ws_done]
);
exhaustive_unions!(
    Union5,
    LexUnion5s,
    ExhaustiveUnion5s,
    lex_union5s,
    exhaustive_union5s,
    5,
    [0, X, I, A, xs, xs_done],
    [1, Y, J, B, ys, ys_done],
    [2, Z, K, C, zs, zs_done],
    [3, W, L, D, ws, ws_done],
    [4, V, M, E, vs, vs_done]
);
exhaustive_unions!(
    Union6,
    LexUnion6s,
    ExhaustiveUnion6s,
    lex_union6s,
    exhaustive_union6s,
    6,
    [0, X, I, A, xs, xs_done],
    [1, Y, J, B, ys, ys_done],
    [2, Z, K, C, zs, zs_done],
    [3, W, L, D, ws, ws_done],
    [4, V, M, E, vs, vs_done],
    [5, U, N, F, us, us_done]
);
exhaustive_unions!(
    Union7,
    LexUnion7s,
    ExhaustiveUnion7s,
    lex_union7s,
    exhaustive_union7s,
    7,
    [0, X, I, A, xs, xs_done],
    [1, Y, J, B, ys, ys_done],
    [2, Z, K, C, zs, zs_done],
    [3, W, L, D, ws, ws_done],
    [4, V, M, E, vs, vs_done],
    [5, U, N, F, us, us_done],
    [6, T, O, G, ts, ts_done]
);
exhaustive_unions!(
    Union8,
    LexUnion8s,
    ExhaustiveUnion8s,
    lex_union8s,
    exhaustive_union8s,
    8,
    [0, X, I, A, xs, xs_done],
    [1, Y, J, B, ys, ys_done],
    [2, Z, K, C, zs, zs_done],
    [3, W, L, D, ws, ws_done],
    [4, V, M, E, vs, vs_done],
    [5, U, N, F, us, us_done],
    [6, T, O, G, ts, ts_done],
    [7, S, P, H, ss, ss_done]
);
