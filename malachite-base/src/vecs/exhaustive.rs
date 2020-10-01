use iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
use iterators::iterator_cache::IteratorCache;
use itertools::Itertools;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::SignificantBits;
use std::iter::{once, Once};
use std::mem::swap;

fn set_slice_to_none<T>(xs: &mut [Option<T>]) {
    for x in xs {
        *x = None;
    }
}

macro_rules! exhaustive_fixed_length_vecs {
    (
        $exhaustive_struct: ident,
        $exhaustive_custom_fn: ident,
        $exhaustive_1_to_1_fn: ident,
        $([$i: expr, $it: ident, $xs: ident, $xs_done: ident, $outputs: ident]),*
    ) => {
        /// Generates all `Vec`s of a given length with elements from $m$ iterators.
        ///
        /// The fixed length $n$ of the `Vec`s is greater than or equal to $m$.
        ///
        /// This struct is macro-generated. The value of $m$ is in the struct's name.
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct<T: Clone, $($it: Iterator<Item=T>,)*> {
            done: bool,
            next: Vec<Option<T>>,
            distributor: BitDistributor,
            $(
                $xs: IteratorCache<$it>,
                $xs_done: bool,
                $outputs: Vec<usize>,
            )*
            oi_map: Vec<usize>,
        }

        impl<T: Clone, $($it: Iterator<Item=T>,)*> Iterator for $exhaustive_struct<T, $($it,)*> {
            type Item = Vec<T>;

            fn next(&mut self) -> Option<Vec<T>> {
                if self.done {
                    None
                } else {
                    loop {
                        let mut some_are_valid = false;
                        let mut all_are_valid = true;
                        $(
                            let mut no_x = false;
                            for &output_index in &self.$outputs {
                                if let Some(x) = self.$xs.get(
                                    self.distributor.get_output(output_index)
                                ) {
                                    self.next[output_index] = Some(x.clone());
                                    some_are_valid = true;
                                } else {
                                    no_x = true;
                                    all_are_valid = false;
                                    if some_are_valid {
                                        break;
                                    }
                                }
                            }
                            if no_x {
                                if !self.$xs_done {
                                    self.$xs_done = true;
                                    let xs_len = self.$xs.known_len().unwrap();
                                    if xs_len == 0 {
                                        self.done = true;
                                        return None;
                                    }
                                    self.distributor.set_max_bits(
                                        &self.$outputs,
                                        usize::wrapping_from(xs_len.significant_bits())
                                    );
                                    set_slice_to_none(&mut self.next);
                                    continue;
                                } else if some_are_valid {
                                    set_slice_to_none(&mut self.next);
                                    self.distributor.increment_counter();
                                    continue;
                                }
                            }
                        )*
                        if !some_are_valid {
                            self.done = true;
                            return None;
                        } else if all_are_valid {
                            break;
                        } else {
                            set_slice_to_none(&mut self.next);
                            self.distributor.increment_counter();
                        }
                    }
                    let mut out = vec![None; self.next.len()];
                    swap(&mut self.next, &mut out);
                    self.distributor.increment_counter();
                    Some(out.into_iter().map(Option::unwrap).collect())
                }
            }
        }

        //TODO doc
        pub fn $exhaustive_custom_fn<T: Clone, $($it: Iterator<Item=T>,)*> (
            $($xs: $it,)*
            output_types: &[(BitDistributorOutputType, usize)],
        ) -> $exhaustive_struct<T, $($it,)*> {
            $(
                let _max_input_index = $i;
            )*
            let oi_map: Vec<usize> = output_types.iter().map(|(_, i)| *i).collect();
            let oi_sorted_unique = oi_map.iter().cloned().unique().sorted().collect::<Vec<_>>();
            assert_eq!(*oi_sorted_unique.first().unwrap(), 0);
            assert_eq!(*oi_sorted_unique.last().unwrap(), _max_input_index);
            $exhaustive_struct {
                done: false,
                next: vec![None; output_types.len()],
                distributor: BitDistributor::new(output_types.iter().map(|(ot, _)| *ot)
                    .collect::<Vec<_>>().as_slice()),
                $(
                    $xs: IteratorCache::new($xs),
                    $xs_done: false,
                    $outputs: output_types.iter().enumerate()
                        .filter_map(|(o, (_, i))| if *i == $i { Some(o) } else { None }).collect(),
                )*
                oi_map
            }
        }

        /// Generates all length-$n$ `Vec`s of a given length with elements from $n$ iterators.
        ///
        /// This function is macro-generated. The value of $n$ is in the function's name.
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite, the output length is the product of their
        /// lengths. If any of `xs`, `ys`, `zs`, ... are infinite, the output is also infinite.
        ///
        /// If any of `xs`, `ys`, `zs`, ... is empty, the output is also empty.
        ///
        /// # Complexity per iteration
        ///
        /// If all of `xs`, `ys`, `zs`, ... are finite:
        ///
        /// $T(i, n) = O(n)$
        ///
        /// $M(i, n) = O(n)$
        ///
        /// If $k$ of `xs`, `ys`, `zs`, ... are infinite:
        ///
        /// $$
        /// T(i, n) = O(n + \sum_{j=0}^{k-1}T_j(\sqrt[n]{i}))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(n + \sum_{j=0}^{k-1}M_j(\sqrt[n]{i}))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and
        /// $T_0, T_1, \ldots T_{k-1}$ and $M_0, M_1, \ldots M_{k-1}$ are the time and additional
        /// memory functions of the infinite input iterators.
        ///
        /// # Examples
        ///
        /// See the documentation of the `exhaustive` module.
        #[inline]
        pub fn $exhaustive_1_to_1_fn<T: Clone, $($it: Iterator<Item=T>,)*> (
            $($xs: $it,)*
        ) -> $exhaustive_struct<T, $($it,)*> {
            $exhaustive_custom_fn(
                $($xs,)*
                &[$((BitDistributorOutputType::normal(1), $i),)*]
            )
        }
    }
}

exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs2,
    exhaustive_fixed_length_vecs_2_inputs,
    exhaustive_length_2_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs3,
    exhaustive_fixed_length_vecs_3_inputs,
    exhaustive_length_3_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs4,
    exhaustive_fixed_length_vecs_4_inputs,
    exhaustive_length_4_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2],
    [3, L, ws, ws_done, outputs_3]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs5,
    exhaustive_fixed_length_vecs_5_inputs,
    exhaustive_length_5_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2],
    [3, L, ws, ws_done, outputs_3],
    [4, M, vs, vs_done, outputs_4]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs6,
    exhaustive_fixed_length_vecs_6_inputs,
    exhaustive_length_6_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2],
    [3, L, ws, ws_done, outputs_3],
    [4, M, vs, vs_done, outputs_4],
    [5, N, us, us_done, outputs_5]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs7,
    exhaustive_fixed_length_vecs_7_inputs,
    exhaustive_length_7_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2],
    [3, L, ws, ws_done, outputs_3],
    [4, M, vs, vs_done, outputs_4],
    [5, N, us, us_done, outputs_5],
    [6, O, ts, ts_done, outputs_6]
);
exhaustive_fixed_length_vecs!(
    ExhaustiveFixedLengthVecs8,
    exhaustive_fixed_length_vecs_8_inputs,
    exhaustive_length_8_vecs,
    [0, I, xs, xs_done, outputs_0],
    [1, J, ys, ys_done, outputs_1],
    [2, K, zs, zs_done, outputs_2],
    [3, L, ws, ws_done, outputs_3],
    [4, M, vs, vs_done, outputs_4],
    [5, N, us, us_done, outputs_5],
    [6, O, ts, ts_done, outputs_6],
    [7, P, ss, ss_done, outputs_7]
);

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct ExhaustiveFixedLengthVecsFromSingleG<I: Iterator>
where
    I::Item: Clone,
{
    done: bool,
    next: Vec<Option<I::Item>>,
    distributor: BitDistributor,
    xs: IteratorCache<I>,
    xs_done: bool,
}

impl<I: Iterator> Iterator for ExhaustiveFixedLengthVecsFromSingleG<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        if self.done {
            None
        } else {
            loop {
                let mut some_are_valid = false;
                let mut all_are_valid = true;
                for (i, n) in self.next.iter_mut().enumerate() {
                    if let Some(x) = self.xs.get(self.distributor.get_output(i)) {
                        *n = Some(x.clone());
                        some_are_valid = true;
                    } else {
                        all_are_valid = false;
                        if some_are_valid {
                            break;
                        }
                    }
                }
                if !all_are_valid {
                    if !self.xs_done {
                        self.xs_done = true;
                        let xs_len = self.xs.known_len().unwrap();
                        if xs_len == 0 {
                            self.done = true;
                            return None;
                        }
                        self.distributor
                            .set_max_bits(&[0], usize::wrapping_from(xs_len.significant_bits()));
                        set_slice_to_none(&mut self.next);
                        continue;
                    } else if some_are_valid {
                        set_slice_to_none(&mut self.next);
                        self.distributor.increment_counter();
                        continue;
                    }
                }
                if !some_are_valid {
                    self.done = true;
                    return None;
                } else if all_are_valid {
                    break;
                } else {
                    set_slice_to_none(&mut self.next);
                    self.distributor.increment_counter();
                }
            }
            let mut out = vec![None; self.next.len()];
            swap(&mut self.next, &mut out);
            self.distributor.increment_counter();
            Some(out.into_iter().map(Option::unwrap).collect())
        }
    }
}

fn exhaustive_fixed_length_vecs_from_single_g<I: Iterator>(
    len: usize,
    xs: I,
) -> ExhaustiveFixedLengthVecsFromSingleG<I>
where
    I::Item: Clone,
{
    ExhaustiveFixedLengthVecsFromSingleG {
        done: false,
        next: vec![None; len],
        distributor: BitDistributor::new(&vec![BitDistributorOutputType::normal(1); len]),
        xs: IteratorCache::new(xs),
        xs_done: false,
    }
}

/// Generates all `Vec`s of a given length with elements from a single iterator.
///
/// This `struct` is created by the `exhaustive_fixed_length_vecs_from_single` method. See its
/// documentation for more.
#[derive(Clone, Debug)]
pub enum ExhaustiveFixedLengthVecsFromSingle<I: Iterator>
where
    I::Item: Clone,
{
    Zero(Once<Vec<I::Item>>),
    One(I),
    GreaterThanOne(ExhaustiveFixedLengthVecsFromSingleG<I>),
}

impl<I: Iterator> Iterator for ExhaustiveFixedLengthVecsFromSingle<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        match self {
            ExhaustiveFixedLengthVecsFromSingle::Zero(ref mut xs) => xs.next(),
            ExhaustiveFixedLengthVecsFromSingle::One(ref mut xs) => xs.next().map(|x| vec![x]),
            ExhaustiveFixedLengthVecsFromSingle::GreaterThanOne(ref mut xs) => xs.next(),
        }
    }
}

/// Generates all `Vec`s of a given length with elements from a single iterator.
///
/// If `xs` is finite, the output length is $\ell^n$, where $\ell$ is `xs.count()` and $n$ is `len`.
/// If `xs` is infinite, the output is also infinite.
///
/// If `len` is 0, the output consists of one empty list.
///
/// If `xs` is empty, the output is also empty, unless `len` is 0.
///
/// # Complexity per iteration
///
/// If `xs` is finite:
///
/// $T(i, n) = O(n)$
///
/// $M(i, n) = O(n)$
///
/// If `xs` is infinite:
///
/// $T(i, n) = O(n + T^\prime(\sqrt[n]{i}))$
///
/// $M(i, n) = O(n + M^\prime(\sqrt[n]{i}))$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $T^\prime$ and $M^\prime$ are the
/// time and additional memory functions of `xs`.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
/// use malachite_base::vecs::exhaustive::exhaustive_fixed_length_vecs_from_single;
///
/// let xss = exhaustive_fixed_length_vecs_from_single(2, exhaustive_unsigneds::<u8>())
///     .take(20)
///     .collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[0, 0], &[0, 1], &[1, 0], &[1, 1], &[0, 2], &[0, 3], &[1, 2], &[1, 3], &[2, 0],
///         &[2, 1], &[3, 0], &[3, 1], &[2, 2], &[2, 3], &[3, 2], &[3, 3], &[0, 4], &[0, 5],
///         &[1, 4], &[1, 5]
///     ]
/// );
/// ```
pub fn exhaustive_fixed_length_vecs_from_single<I: Iterator>(
    len: usize,
    xs: I,
) -> ExhaustiveFixedLengthVecsFromSingle<I>
where
    I::Item: Clone,
{
    match len {
        0 => ExhaustiveFixedLengthVecsFromSingle::Zero(once(Vec::new())),
        1 => ExhaustiveFixedLengthVecsFromSingle::One(xs),
        len => ExhaustiveFixedLengthVecsFromSingle::GreaterThanOne(
            exhaustive_fixed_length_vecs_from_single_g(len, xs),
        ),
    }
}
