use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

macro_rules! union {
    ($name: ident, $single: ty, $([$t: ident, $cons: ident, $c: expr, $x: ident]),*) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum $name<$($t),*> {
            $($cons($t)),*
        }

        impl<T> $single {
            #[allow(clippy::missing_const_for_fn)] // Can't be const because of destructor
            pub fn collapse(self) -> T {
                match self {
                    $(
                        $name::$cons($x) => $x
                    ),*
                }
            }
        }

        impl<$($t: Display),*> Display for $name<$($t),*> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                match self {
                    $(
                        $name::$cons($x) => f.write_fmt(format_args!("{}({})", $c, $x))
                    ),*
                }
            }
        }

        impl<$($t: FromStr),*> FromStr for $name<$($t),*> {
            type Err = Union2<String, $name<$($t::Err),*>>;

            #[inline]
            fn from_str(src: &str) -> Result<$name<$($t),*>, Self::Err> {
                if src.is_empty() {
                    return Err(Union2::A(String::new()));
                }
                let (head, tail) = src.split_at(1);
                let tail = if let Some(tail) = tail.strip_prefix('(') {
                    tail
                } else {
                    return Err(Union2::A(src.to_string()));
                };
                let tail = if let Some(tail) = tail.strip_suffix(')') {
                    tail
                } else {
                    return Err(Union2::A(src.to_string()));
                };
                match head.chars().next().unwrap() {
                    $(
                        $c => $t::from_str(tail)
                                .map($name::$cons).map_err(|e| Union2::B($name::$cons(e))),
                    )*
                    _ => Err(Union2::A(src.to_string()))
                }
            }
        }
    }
}

union!(Union2, Union2<T, T>, [A, A, 'A', a], [B, B, 'B', b]);
union!(Union3, Union3<T, T, T>, [A, A, 'A', a], [B, B, 'B', b], [C, C, 'C', c]);
union!(Union4, Union4<T, T, T, T>, [A, A, 'A', a], [B, B, 'B', b], [C, C, 'C', c], [D, D, 'D', d]);
union!(
    Union5,
    Union5<T, T, T, T, T>,
    [A, A, 'A', a],
    [B, B, 'B', b],
    [C, C, 'C', c],
    [D, D, 'D', d],
    [E, E, 'E', e]
);
union!(
    Union6,
    Union6<T, T, T, T, T, T>,
    [A, A, 'A', a],
    [B, B, 'B', b],
    [C, C, 'C', c],
    [D, D, 'D', d],
    [E, E, 'E', e],
    [F, F, 'F', f]
);
union!(
    Union7,
    Union7<T, T, T, T, T, T, T>,
    [A, A, 'A', a],
    [B, B, 'B', b],
    [C, C, 'C', c],
    [D, D, 'D', d],
    [E, E, 'E', e],
    [F, F, 'F', f],
    [G, G, 'G', g]
);
union!(
    Union8,
    Union8<T, T, T, T, T, T, T, T>,
    [A, A, 'A', a],
    [B, B, 'B', b],
    [C, C, 'C', c],
    [D, D, 'D', d],
    [E, E, 'E', e],
    [F, F, 'F', f],
    [G, G, 'G', g],
    [H, H, 'H', h]
);
