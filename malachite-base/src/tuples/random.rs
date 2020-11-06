use random::Seed;
use std::iter::{repeat, Repeat};

/// Generates random units; repeats `()`.
///
/// $P(()) = 1$.
///
/// The output length is infinite.
///
/// # Complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::tuples::random::random_units;
///
/// assert_eq!(random_units().take(10).collect::<Vec<_>>(), &[(); 10]);
/// ```
pub fn random_units() -> Repeat<()> {
    repeat(())
}

// hack for macro
#[inline]
fn next_helper<I: Iterator>(x: &mut I, _i: usize) -> Option<I::Item> {
    x.next()
}

macro_rules! random_tuples {
    (
        $random_struct: ident,
        $random_struct_from_single: ident,
        $random_fn: ident,
        $random_fn_from_single: ident,
        $single_out: tt,
        $([$i: expr, $t: ident, $it: ident, $xs: ident, $xs_gen:ident]),*
    ) => {
        /// Generates random $n$-tuples using elements from $n$ iterators.
        ///
        /// This struct is macro-generated. The value of $n$ is in the struct's name.
        #[derive(Clone, Debug)]
        pub struct $random_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
            $($xs: $it,)*
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> Iterator for $random_struct<$($t, $it,)*>
        {
            type Item = ($($t,)*);

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                Some(($(self.$xs.next().unwrap()),*))
            }
        }

        /// Generates random $n$-tuples with elements from $n$ iterators.
        ///
        /// This function is macro-generated. The value of $n$ is in the function's name.
        ///
        /// The probability of a particular $n$-tuple being generated is the product of the
        /// probabilities of each of its elements.
        ///
        /// `xs`, `ys`, `zs`, ... must be infinite.
        ///
        /// # Expected complexity per iteration
        ///
        /// $$
        /// T(i, n) = O(\sum_{j=0}^{n-1}T_j(i))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(\sum_{j=0}^{n-1}M_j(i))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the number of input iterators, and
        /// $T_0, T_1, \ldots T_{n-1}$ and $M_0, M_1, \ldots M_{n-1}$ are the time and additional
        /// memory functions of the input iterators.
        ///
        /// # Examples
        ///
        /// See the documentation of the `tuples::random` module.
        pub fn $random_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            seed: Seed,
            $($xs_gen: &dyn Fn(Seed) -> $it,)*
        ) -> $random_struct<$($t, $it,)*> {
            $random_struct {
                $($xs: $xs_gen(seed.fork(stringify!($xs))),)*
            }
        }

        /// Generates random $n$-tuples using elements from a single iterator.
        ///
        /// This struct is macro-generated. The value of $n$ is in the struct's name.
        #[derive(Clone, Debug)]
        pub struct $random_struct_from_single<I: Iterator> {
            xs: I
        }

        #[allow(clippy::type_complexity)]
        impl<I: Iterator> Iterator for $random_struct_from_single<I> {
            type Item = $single_out;

            #[inline]
            fn next(&mut self) -> Option<$single_out> {
                Some(($(next_helper(&mut self.xs, $i).unwrap(),)*))
            }
        }

        /// Generates random $n$-tuples using elements from a single iterator.
        ///
        /// This function is macro-generated. The value of $n$ is in the function's name.
        ///
        /// The probability of a particular $n$-tuple being generated is the product of the
        /// probabilities of each of its elements.
        ///
        /// `xs` must be infinite.
        ///
        /// # Expected complexity per iteration
        ///
        /// $T(i, n) = O(nT^\prime(i))$
        ///
        /// $M(i, n) = O(nM^\prime(i))$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the tuple's width, and $T^\prime$
        /// and $M^\prime$ are the time and additional memory functions of `xs`.
        ///
        /// # Examples
        ///
        /// See the documentation of the `tuples::random` module.
        #[inline]
        pub fn $random_fn_from_single<I: Iterator>(xs: I) -> $random_struct_from_single<I> {
            $random_struct_from_single { xs }
        }
    }
}

random_tuples!(
    RandomPairs,
    RandomPairsFromSingle,
    random_pairs,
    random_pairs_from_single,
    (I::Item, I::Item),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen]
);
random_tuples!(
    RandomTriples,
    RandomTriplesFromSingle,
    random_triples,
    random_triples_from_single,
    (I::Item, I::Item, I::Item),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen],
    [2, Z, K, zs, zs_gen]
);
random_tuples!(
    RandomQuadruples,
    RandomQuadruplesFromSingle,
    random_quadruples,
    random_quadruples_from_single,
    (I::Item, I::Item, I::Item, I::Item),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen],
    [2, Z, K, zs, zs_gen],
    [3, W, L, ws, ws_gen]
);
random_tuples!(
    RandomQuintuples,
    RandomQuintuplesFromSingle,
    random_quintuples,
    random_quintuples_from_single,
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen],
    [2, Z, K, zs, zs_gen],
    [3, W, L, ws, ws_gen],
    [4, V, M, vs, vs_gen]
);
random_tuples!(
    RandomSextuples,
    RandomSextuplesFromSingle,
    random_sextuples,
    random_sextuples_from_single,
    (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen],
    [2, Z, K, zs, zs_gen],
    [3, W, L, ws, ws_gen],
    [4, V, M, vs, vs_gen],
    [5, U, N, us, us_gen]
);
random_tuples!(
    RandomSeptuples,
    RandomSeptuplesFromSingle,
    random_septuples,
    random_septuples_from_single,
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item
    ),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen],
    [2, Z, K, zs, zs_gen],
    [3, W, L, ws, ws_gen],
    [4, V, M, vs, vs_gen],
    [5, U, N, us, us_gen],
    [6, T, O, ts, ts_gen]
);
random_tuples!(
    RandomOctuples,
    RandomOctuplesFromSingle,
    random_octuples,
    random_octuples_from_single,
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item
    ),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen],
    [2, Z, K, zs, zs_gen],
    [3, W, L, ws, ws_gen],
    [4, V, M, vs, vs_gen],
    [5, U, N, us, us_gen],
    [6, T, O, ts, ts_gen],
    [7, S, P, ss, ss_gen]
);

macro_rules! random_custom_tuples {
    (
        $random_struct: ident,
        $out_t: ty,
        $random_fn: ident,
        $([$t: ident, $it: ident, $xs: ident, $xs_gen: ident, $([$x: ident, $x_ord: ident]),*]),*
    ) => {
        /// Generates random $n$-tuples with elements from $m$ iterators, where $m \leq n$.
        ///
        /// The mapping from iterators to tuple slots is indicated by the struct name; for example,
        /// in `RandomTriplesXYX` there are two iterators, `X`, and `Y`; `X` generates the elements
        /// in the first and third slots of the output triples, and `Y` generates the elements in
        /// the second slots.
        ///
        /// This struct is macro-generated.
        #[derive(Clone, Debug)]
        pub struct $random_struct<$($t: Clone, $it: Iterator<Item = $t>,)*> {
            $($xs: $it,)*
        }

        impl<$($t: Clone, $it: Iterator<Item = $t>,)*> Iterator for $random_struct<$($t, $it,)*>
        {
            type Item = $out_t;

            fn next(&mut self) -> Option<Self::Item> {
                $(
                    $(
                        let $x = self.$xs.next().unwrap();
                    )*
                )*
                Some(($($($x_ord,)*)*))
            }
        }

        /// Generates random $n$-tuples with elements from $m$ iterators, where $m \leq n$.
        ///
        /// The mapping from iterators to tuple slots is indicated by the function name; for
        /// example, `random_triples_xyx` takes two iterators, `xs`, and `ys`; `xs` generates the
        /// elements in the first and third slots of the output triples, and `ys` generates the
        /// elements in the second slots.
        ///
        /// The probability of a particular $n$-tuple being generated is the product of the
        /// probabilities of each of its elements.
        ///
        /// `xs`, `ys`, `zs`, ... must be infinite.
        ///
        /// # Expected complexity per iteration
        ///
        /// Let $j$ be the largest index of any output associated with `xs`, $X$ the set of outputs
        /// with indices higher than $j$, $P$ the product of the lengths of all the iterators
        /// associated with the outputs in $X$, including multiplicities, and $T^\prime$ and
        /// $M^\prime$ the time and additional memory complexities of `xs`.
        ///
        /// We have
        ///
        /// $$
        /// T(i, n) = O(\sum_{j=0}^{n-1}T_j(i))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(\sum_{j=0}^{n-1}M_j(i))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the number of input iterators, and
        /// $T_j$ and $M_j$ are the time and additional memory functions of the iterator
        /// corresponding to the $j$th output.
        ///
        /// # Examples
        ///
        /// See the documentation of the `tuples::random` module.
        pub fn $random_fn<$($t: Clone, $it: Iterator<Item = $t>,)*>(
            seed: Seed,
            $($xs_gen: &dyn Fn(Seed) -> $it,)*
        ) -> $random_struct<$($t, $it,)*> {
            $random_struct {
                $($xs: $xs_gen(seed.fork(stringify!($xs))),)*
            }
        }
    }
}

random_custom_tuples!(
    RandomTriplesXXY,
    (X, X, Y),
    random_triples_xxy,
    [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1]],
    [Y, J, ys, ys_gen, [y_2, y_2]]
);
random_custom_tuples!(
    RandomTriplesXYX,
    (X, Y, X),
    random_triples_xyx,
    [X, I, xs, xs_gen, [x_0, x_0], [x_2, y_1]],
    [Y, J, ys, ys_gen, [y_1, x_2]]
);
random_custom_tuples!(
    RandomTriplesXYY,
    (X, Y, Y),
    random_triples_xyy,
    [X, I, xs, xs_gen, [x_0, x_0]],
    [Y, J, ys, ys_gen, [y_1, y_1], [y_2, y_2]]
);
