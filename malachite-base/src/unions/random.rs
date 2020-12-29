use num::random::{random_unsigned_range, RandomUnsignedRange};
use random::Seed;
use unions::{Union2, Union3, Union4, Union5, Union6, Union7, Union8};

macro_rules! random_unions {
    (
        $union: ident,
        $random_struct: ident,
        $random_fn: ident,
        $n: expr,
        $([$i: expr, $t: ident, $it: ident, $variant: ident, $xs: ident, $xs_gen: ident]),*
    ) => {
        #[derive(Clone, Debug)]
        pub struct $random_struct<$($t, $it: Iterator<Item=$t>),*> {
            indices: RandomUnsignedRange<usize>,
            $(
                $xs: $it,
            )*
        }

        impl<$($t, $it: Iterator<Item=$t>),*> Iterator for $random_struct<$($t, $it),*> {
            type Item = $union<$($t),*>;

            fn next(&mut self) -> Option<Self::Item> {
                match self.indices.next().unwrap() {
                    $(
                        $i => self.$xs.next().map($union::$variant),
                    )*
                    _ => unreachable!(),
                }
            }
        }

        pub fn $random_fn<$($t, $it: Iterator<Item=$t>),*>(
            seed: Seed, $($xs_gen: &dyn Fn(Seed) -> $it),*
        ) -> $random_struct<$($t, $it),*> {
            $random_struct {
                indices: random_unsigned_range(seed.fork("indices"), 0, $n),
                $(
                    $xs: $xs_gen(seed.fork(stringify!($xs))),
                )*
            }
        }
    }
}
random_unions!(
    Union2,
    RandomUnion2s,
    random_union2s,
    2,
    [0, X, I, A, xs, xs_gen],
    [1, Y, J, B, ys, ys_gen]
);
random_unions!(
    Union3,
    RandomUnion3s,
    random_union3s,
    3,
    [0, X, I, A, xs, xs_gen],
    [1, Y, J, B, ys, ys_gen],
    [2, Z, K, C, zs, zs_gen]
);
random_unions!(
    Union4,
    RandomUnion4s,
    random_union4s,
    4,
    [0, X, I, A, xs, xs_gen],
    [1, Y, J, B, ys, ys_gen],
    [2, Z, K, C, zs, zs_gen],
    [3, W, L, D, ws, ws_gen]
);
random_unions!(
    Union5,
    RandomUnion5s,
    random_union5s,
    5,
    [0, X, I, A, xs, xs_gen],
    [1, Y, J, B, ys, ys_gen],
    [2, Z, K, C, zs, zs_gen],
    [3, W, L, D, ws, ws_gen],
    [4, V, M, E, vs, vs_gen]
);
random_unions!(
    Union6,
    RandomUnion6s,
    random_union6s,
    6,
    [0, X, I, A, xs, xs_gen],
    [1, Y, J, B, ys, ys_gen],
    [2, Z, K, C, zs, zs_gen],
    [3, W, L, D, ws, ws_gen],
    [4, V, M, E, vs, vs_gen],
    [5, U, N, F, us, us_gen]
);
random_unions!(
    Union7,
    RandomUnion7s,
    random_union7s,
    7,
    [0, X, I, A, xs, xs_gen],
    [1, Y, J, B, ys, ys_gen],
    [2, Z, K, C, zs, zs_gen],
    [3, W, L, D, ws, ws_gen],
    [4, V, M, E, vs, vs_gen],
    [5, U, N, F, us, us_gen],
    [6, T, O, G, ts, ts_gen]
);
random_unions!(
    Union8,
    RandomUnion8s,
    random_union8s,
    8,
    [0, X, I, A, xs, xs_gen],
    [1, Y, J, B, ys, ys_gen],
    [2, Z, K, C, zs, zs_gen],
    [3, W, L, D, ws, ws_gen],
    [4, V, M, E, vs, vs_gen],
    [5, U, N, F, us, us_gen],
    [6, T, O, G, ts, ts_gen],
    [7, S, P, H, ss, ss_gen]
);
