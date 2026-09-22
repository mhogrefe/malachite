# Changelog

All Malachite crates are versioned in lockstep and released together, so this single file covers
the whole workspace. Within each release, entries are grouped by crate. Entries are added to the
Unreleased section as work lands; at release time the section is stamped with the version and
date. The 0.10.0 section was reconstructed retroactively; releases before 0.10.0 are only
documented by git history.

## Unreleased

### Breaking and behavioral changes

- `Rational::to_height`, `Rational::into_height`, and `Rational::height_significant_bits` are no
  longer inherent methods; they are now the methods of the new `Height` trait, which several other
  types also implement. Code that calls them needs
  `use malachite_base::num::arithmetic::traits::Height;`, and nothing else changes — the
  signatures and the results are the same.

### malachite-base

- A new `ToLatex` trait, for converting a value to a LaTeX math-mode fragment, along with the
  `LatexWrapper` struct that its `to_latex` method returns. The output is a fragment rather than a
  complete expression, carrying no `$`, `\(`, `\[`, or environment of its own and leaving those
  to the caller, so that one fragment can be embedded in another. `fmt_latex`, the method
  implementors define, takes a `Formatter` rather than returning a `String`, so that a value
  built out of smaller values writes its parts into a single buffer instead of allocating once per
  level of nesting. So far it is implemented for the primitive integers, where the fragment is
  identical to the `Display` output, and for the primitive floats, where the infinities become
  `\infty` and `-\infty`, `NaN` becomes `\text{NaN}`, the zeros keep their signs as `0.0` and
  `-0.0`, and a finite value starts from its shortest round-tripping `NiceFloat` representation,
  with an exponent rewritten in the form `1.0 \times 10^{-45}`. It is also implemented for the
  unit type, which becomes `()`; for `bool`, which becomes `\text{T}` or `\text{F}`; for
  `Ordering`, which becomes the relation symbol it stands for, `<`, `=`, or `>`; and for
  `RoundingMode`, which becomes its name in uppercase, such as `\text{FLOOR}` — there being no
  conventional mathematical symbol for a rounding mode.
- `ToLatex` for `char`, `&str`, and `String`. A fragment depicts the string rather than translating
  it, and adds no quotation marks: ordinary characters are gathered into `\text{...}` groups and
  typeset as themselves, with LaTeX's special characters escaped, while a character that LaTeX
  spells with a math-mode macro is written as that macro outside any group, so that `"100% α"`
  becomes `\text{100\% }\alpha`. Runs of spaces are preserved rather than collapsed, text-mode
  ligatures are broken so that `"--flag"` does not acquire an en dash, and control words are braced
  so that `"\n"` cannot become the undefined `\textbackslashn`. A run of superscript or subscript
  characters becomes a single script, so that `"2¹⁰"` is two raised to the tenth rather than two
  raised to the first and then to the zeroth, and `"H₂O"` becomes `\text{H}_2\text{O}`; a run of one
  kind does not run into the next, since the two would stack rather than sit side by side. The three
  superscript digits that Unicode puts in Latin-1 Supplement (`¹²³`) are respelled in math mode to
  match the other seven, which pylatexenc spells in text mode, so that a run of them is unbroken. A
  character with no LaTeX spelling is written literally, which needs an engine and a font that can
  render it. The 1553-entry character table is adapted from
  [pylatexenc](https://github.com/phfaist/pylatexenc) (MIT, © 2015-2023 Philippe Faist), which in
  turn adapted it from latexcodec (MIT, © 2011-2014 Matthias C. M. Troffaes); both notices are
  reproduced in the generated table.
- A new `ToTypst` trait, for converting a value to a Typst math-mode fragment, along with the
  `TypstWrapper` struct that its `to_typst` method returns. It mirrors `ToLatex`, with an
  implementation for every type that has one: the primitive integers and floats, `()`, `bool`,
  `Ordering`, `RoundingMode`, `char`, `&str`, `String`, and `Option<T>` whenever `T: ToTypst`. Typst
  reads Unicode natively and a quoted string typesets its contents as text, so a string needs no
  character table at all: it is one string literal, with only `\` and `"` escaped and control
  characters spelled rather than written, and `"100% α"` comes out as itself. A run of superscript
  or subscript characters is the exception, and becomes a real script, since a text font often lacks
  the Superscripts and Subscripts block. Where LaTeX writes `\text{NaN}`, `\infty`, `\bot`, and
  `\left[5\right]`, Typst writes `"NaN"`, `infinity`, `bot`, and `[5]`, the last of which Typst
  grows to fit its contents without being asked. Every fragment the tests produce is compiled by
  Typst itself, so a fragment Typst would reject cannot pass.
- `ToLatex::to_latex_string` and `ToTypst::to_typst_string`, which give the fragment as a `String`
  in one call rather than as a wrapper to be turned into one. A new `use_to_string_variant` lint
  prefers them, and the rest of the `to_*_string` family, over the long ways of saying the same
  thing: a `to_latex` or `to_typst` wrapper immediately turned into a `String`, and a `format!`
  whose whole format string is a single `{:?}`, `{:b}`, `{:o}`, `{:x}`, or `{:X}`. A method's own
  definition is left alone, since `to_binary_string` is written with `format!("{self:b}")` and
  cannot be asked to call itself.
- `ToLatex` and `ToTypst` for `Never` and for `NiceFloat<T>`. A `Never` cannot be instantiated, so
  its implementations can never be called; they exist so that a type parameter bounded by either
  trait may be `Never`, which is what lets `Option<Never>` have a fragment. A `NiceFloat`'s fragment
  is the wrapped float's own, since the float implementations already build their output from the
  `NiceFloat` representation.
- `ToLatex` and `ToTypst` for slices and for `Vec<T>`, whenever the element type has them. The
  elements' fragments are separated by commas and wrapped in square brackets, as `Option`'s are, so
  that a sequence's fragment is built out of its elements' own: `vec![1u8, 2, 3]` becomes `\left[1,
  2, 3\right]` and `[1, 2, 3]`. The brackets are not decoration; without them `[1, 2]` and `[[1],
  [2]]` would share a fragment. LaTeX's grow with `\left` and `\right`, and Typst's grow on their
  own.
- `ToLatex` and `ToTypst` for `HashSet<T>` and `BTreeSet<T>`, whenever the element type has them.
  These are as the slice implementations are, but wrapped in braces, as a set is written in
  mathematics: `BTreeSet::from([3u8, 1, 2])` becomes `\left\{1, 2, 3\right\}` and `{1, 2, 3}`. A
  `HashSet`'s elements are sorted first, which is why its implementations ask for `Ord` where a
  `HashSet` does not: a `HashSet` iterates in an order that depends on its hasher, so without
  sorting two equal sets could have different fragments, and the same set could have a different
  fragment in the next run. Sorting also makes a `HashSet`'s fragment agree with the `BTreeSet` of
  the same elements.
- `ToLatex` and `ToTypst` for `HashMap<K, V>` and `BTreeMap<K, V>`, whenever the key and value types
  have them. Each entry is written as its key, a "maps to" arrow, and its value; the entries are
  separated by commas and wrapped in braces, as a map is a set of associations:
  `BTreeMap::from([(1u8, 10u8), (2, 20)])` becomes `\left\{1 \mapsto 10, 2 \mapsto 20\right\}` and
  `{1 |-> 10, 2 |-> 20}`. Two-line matrix notation was considered and rejected, since it means a
  permutation specifically. As for `HashSet`, a `HashMap`'s entries are sorted by key first, so its
  implementations ask for `Ord` where a `HashMap` does not.
- `ToLatex` and `ToTypst` for tuples of one to eight elements, whenever every element type has them.
  The elements' fragments are separated by commas and wrapped in parentheses, so `(1u8, 2u8)`
  becomes `\left(1, 2\right)` and `(1, 2)`; the elements need not share a type, and a tuple may hold
  another. Rust has no way to write one implementation for every arity, so these stop at eight, as
  the rest of this crate's tuple machinery does. Since the orphan rule keeps another crate from
  implementing either trait for a longer tuple, the new `latex_tuple!` and `typst_tuple!` macros
  write those fragments instead, as do the `latex_tuple` and `typst_tuple` functions they call,
  which take their elements as trait objects. That works because `fmt_latex` and `fmt_typst` are
  object-safe.
- `ToLatex` and `ToTypst` for `FoerSequence<T>`, whenever the element type has them. The elements
  are bracketed and comma-separated as a sequence's are, and the repeating part, if there is one,
  goes under a vinculum: `[1, 2, [3, 4]]` becomes `\left[1, 2, \overline{3, 4}\right]` and `[1, 2,
  overline(3 comma 4)]`. Inside the Typst vinculum the elements are separated by Typst's `comma`
  symbol rather than by a literal comma, since `overline` takes a single body and a literal comma
  there would be read as an argument separator.
- `ToLatex` and `ToTypst` for the union types, whenever every variant's type has them. The fragment
  is the variant's letter, upright, followed by the wrapped value's own fragment in parentheses, so
  `Union2::A(2)` becomes `\text{A}\left(2\right)` and `"A"(2)`; the letter is what keeps two
  variants holding equal values apart. The implementations are written by `union_struct!`, so a
  `Union3` or longer union defined outside this crate gets them too. They use fully qualified paths,
  so the macro asks its callers for no new imports.
- `ToLatex` and `ToTypst` for `&T` whenever `T` has them, so a reference is invisible: it writes
  what its referent would, which is what lets a collection of references be written at all. `&str`
  and slices keep implementations of their own rather than reaching the blanket one, since their
  referents are unsized; they write the same fragments either way.
- `ToLatex` and `ToTypst` for arrays `[T; N]`, which write what the slice of them would. Before this
  an array had no implementation at all: the `to_latex` and `to_typst` methods require `Self:
  Sized`, so an array could not reach the slice implementation by coercion.
- `ToLatex` and `ToTypst` for `Factors`, writing a prime factorization as a product of prime powers:
  the factorization of 90 becomes `2 \times 3^2 \times 5` and `2 times 3^2 times 5`. An exponent of
  1 is left off, as it is when a factorization is written by hand, and the factorization of 1, which
  has no prime factors, becomes `1`: the empty product, which is what it multiplies out to.
- `ToLatex` and `ToTypst` for `Natural`, `Integer`, `GaussianInteger`, and the
  `ComparableGaussianInteger` and `ComparableGaussianIntegerRef` wrappers. Each fragment is what
  `Display` gives, which is already what math mode wants: decimal digits for a `Natural`, those with
  a leading minus for an `Integer`, and for a `GaussianInteger` the usual `2-3i`, with coefficients
  of 1 and -1 elided and a purely real or imaginary value written as one term. The imaginary unit is
  a plain `i`, set in italics as most mathematical writing sets it. The wrappers write what the
  value they wrap does, since they exist to give an ordering.
- `ToLatex` and `ToTypst` for `Rational`, `GaussianRational`, and the `ComparableGaussianRational`
  and `ComparableGaussianRationalRef` wrappers. A value whose denominator is 1 is written as its
  numerator alone, and otherwise as a fraction: `\frac{22}{7}` and `frac(22, 7)`. A sign is written
  outside the fraction, as `-\frac{2}{3}` rather than `\frac{-2}{3}`, since it belongs to the value
  and not to its numerator. An imaginary term puts the imaginary unit in the numerator, so 5/6 times
  i is `\frac{5i}{6}` rather than a fraction with an `i` hung off it, matching how `Display` writes
  `5i/6`; coefficients of 1 are elided, giving `\frac{i}{2}`.
- `ToLatex` and `ToTypst` for `Float` and the `ComparableFloat` and `ComparableFloatRef` wrappers,
  written as the primitive floats are. A NaN becomes `\text{NaN}` and the infinities become `\infty`
  and `-\infty`; a finite `Float` is written as `Display` writes it, with the exponent, if there is
  one, lifted into a real power of ten, so `1.3e30` becomes `1.3 \times 10^{30}`. As with `Display`,
  the digit count follows the `Float`'s precision rather than its value, and the two zeros are kept
  apart.
- A new `vars` module, holding schemes for naming variables. A polynomial's variables are numbered
  rather than named, so something has to turn those numbers into names and names back into numbers;
  the new `VarScheme` trait is that something, and it lets one polynomial be shown as $x_0 + x_1$,
  or as $x + y$, or with whatever names a caller has in mind, while the polynomial itself knows
  nothing about any of them. A scheme names a variable in three languages — as plain text, as
  LaTeX, and as Typst — of which only the plain name is read back, by `parse_var`. `VarScheme::var`
  gives a `Var` handle, a scheme together with an index, which implements `Display`, `ToLatex`, and
  `ToTypst`, so that a variable can be written wherever any of those is expected; `Var::new` does
  the same for a scheme behind a `dyn`, which the trait supports, so a scheme may be chosen at run
  time. The trait's contract is that a name is never empty, holds no reserved character, belongs to
  one variable alone, and reads back as that variable; the new `char_is_reserved` says which
  characters are reserved, namely the digits, `+`, `-`, `*`, `/`, `^`, `(`, `)`, `,`, and
  whitespace. Keeping those out of the names is what lets a polynomial be written without
  separators and still be read back, since a name is then exactly a longest run of unreserved
  characters. The design follows the `Var` and `ParsableVar` classes of Azurite, a separate project
  of the author's, where the same four conditions are theorems rather than a contract; the `n` that
  bounds a variable type there becomes the scheme's `capacity`, which a scheme with no bound
  reports as `None`.
- Nine naming schemes, one per module under `vars`: `IndexedVars` and `IndexedCapsVars`, which name
  variables `x₀, x₁, x₂, …` and `X₀, X₁, X₂, …`, writing the index as a run of Unicode subscript
  digits and, in LaTeX and Typst, as a real subscript, braced or parenthesized only when it is more
  than one digit long; `AbcVars` and `AbcCapsVars`, the 26 letters in their usual order; `XyzVars`
  and `XyzCapsVars`, the same 26 letters ordered as a mathematician reaches for them, `x, y, z, w,
  v, …, a`; `GreekVars` and `GreekCapsVars`, the 24 Greek letters, skipping the final sigma, which
  is a second form of a letter already named, and the one code point Unicode leaves unassigned
  among the capitals; and `ListVars`, which takes its names from the caller and checks them once,
  when it is made, so that the scheme itself cannot fail. Only `IndexedVars` and `IndexedCapsVars`
  have no capacity, since an index is written out rather than looked up. In LaTeX a Greek letter is
  written with its macro where it has one and as the Latin letter it looks like where it does not,
  since LaTeX has no `\omicron` or `\Alpha`; the spelling is read off the same character table that
  `ToLatex for char` consults, rather than written out a second time. In Typst a Greek letter is
  written as itself, which Typst reads natively. A name that is a single ASCII letter is written
  bare in both languages, so that both set it in math italics as a variable should be, and a longer
  name is set upright; that is what the trait's two markup methods do by default, which is why
  `ListVars` needs only three methods and a scheme written outside this crate needs only the same
  three.
- `chars::scripts`, with `fmt_subscript_digits` and `parse_subscript_digits`, which write and read a
  number as a run of Unicode subscript digits. The two are inverse: parsing accepts exactly what
  writing produces and nothing else, rejecting the empty string, ASCII digits, a leading zero, and a
  number too large for a `u64`, so that a number has one spelling rather than several. `IndexedVars`
  is built on them.
- `ToLatex` for `Option<T>` whenever `T: ToLatex`. `None` becomes `\bot`, and `Some` wraps its
  value in square brackets written with `\left` and `\right`, so that they grow to fit a value
  taller than one line: `Some(5)` becomes `\left[5\right]`. The brackets are not decoration:
  without them `Some(None)` and `None` would share a fragment, and distinct values would be
  indistinguishable. Braces would have served as well, but this crate's documentation already
  spells the Iverson bracket with them.
- Exhaustive generators of ordered [`Vec`]s, the fourth quadrant alongside all `Vec`s, unique
  `Vec`s, and ordered unique `Vec`s: the elements of each `Vec` appear in the order of the source
  iterator but may repeat, so the `Vec`s are the multisets of the elements, each written in source
  order. `exhaustive_ordered_vecs_fixed_length` generates those of one length by pairing each
  ordered unique `Vec` with the compositions that assign its elements multiplicities, which is how
  `exhaustive_unique_vecs` is built from ordered unique `Vec`s and permutations, and
  `exhaustive_ordered_vecs_from_length_iterator` runs that for each length an iterator produces, the
  way `exhaustive_vecs_from_length_iterator` does; `exhaustive_ordered_vecs`, with `_min_length`,
  `_length_range`, and `_length_inclusive_range` variants, are built on it, so that the lengths of
  their output grow logarithmically, as the all-`Vec`s family's do. (Interleaving subsets rather
  than lengths, whatever the index sequence, gives one subset a constant share of the output and
  makes lengths grow linearly instead.) `shortlex_ordered_vecs` and its `_min_length`,
  `_length_range`, and `_length_inclusive_range` variants generate them in order of increasing
  length and lexicographically within each length; and `lex_ordered_vecs_fixed_length`,
  `_length_range`, and `_length_inclusive_range` generate them lexicographically. There is no
  unbounded lexicographic variant, since with repetition allowed it would produce `[x]`, `[x, x]`,
  `[x, x, x]`, and so on forever without reaching a second element — the same reason the all-`Vec`s
  family has `shortlex_vecs` but no `lex_vecs`. The count for a fixed length $k$ from $n$ elements
  is $\binom{n+k-1}{k}$, and over a nonempty source the unbounded generators are infinite.

- Random generators of ordered [`Vec`]s, matching the new exhaustive ones: `random_ordered_vecs`,
  with `_fixed_length`, `_from_length_iterator`, `_min_length`, `_length_range`, and
  `_length_inclusive_range` variants. As for random ordered unique [`Vec`]s, "ordered" here means
  sorted by [`Ord`] rather than by a source iterator's order, so each generator draws a random
  [`Vec`] and sorts it. Unlike the unique versions, these place no demand on the element iterator:
  since elements may repeat, the requested length is always reachable, and no iteration can hang
  waiting for distinct values.

- Exhaustive generators of [`HashMap`]s and [`BTreeMap`]s, in a new `maps` module. Each map is a set
  of keys paired with an assignment of values to them, so they are generated by pairing the ordered
  unique key [`Vec`]s with the length-matched value [`Vec`]s. Besides the plain generators there are
  `_fixed_size`, `_min_size`, `_size_range`, and `_size_inclusive_range` variants restricting the
  number of keys; `_fixed_unique_value_count`, `_unique_value_count_range`, and
  `_unique_value_count_inclusive_range` variants restricting the number of distinct values; and
  `_size_and_unique_value_count_inclusive_range`, which restricts both. A map with $k$ entries has
  between 1 and $k$ distinct values, or 0 if $k$ is 0, so the two restrictions are intersected with
  that range rather than applied independently.
- Random generators of [`HashMap`]s and [`BTreeMap`]s, matching the new exhaustive ones:
  `random_hash_maps` and `random_b_tree_maps`, with `_fixed_size`, `_from_size_iterator`,
  `_min_size`, `_size_range`, and `_size_inclusive_range` variants, plus
  `_fixed_unique_value_count`, `_unique_value_count_range`, `_unique_value_count_inclusive_range`,
  and `_size_and_unique_value_count_inclusive_range`, which restrict the number of distinct values
  as the exhaustive generators do. Sizes come from a geometric distribution, as lengths do
  elsewhere, except where a size range is given and they are uniform. A map with $k$ entries has
  between 1 and $k$ distinct values, or 0 if $k$ is 0, so the size is drawn first and the
  distinct-value count is then drawn uniformly from those the size admits; when both are restricted,
  the sizes are drawn only from those that admit an allowed count. Maps with the same size and
  distinct-value count are not equally likely — those whose values are spread evenly over the keys
  are favored — but every such map is reachable. Each entry consumes exactly one draw from the value
  iterator, and a repeated key costs a key draw but no value draw; pairing keys with values by the
  map's own iteration order instead would make a [`HashMap`]'s contents depend on the hasher, and so
  differ between runs. As for random sets, the key iterator must be able to produce as many distinct
  keys as the largest size requested — and, for the unique-value-count generators, the value
  iterator as many distinct values as the largest count requested — or the generator will hang.

- `exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range`, generating the [`Vec`]s of a
  fixed length whose number of distinct elements lies in a range. Filtering unrestricted [`Vec`]s
  would not do: asking for one distinct element among length-$k$ [`Vec`]s over $n$ elements passes
  roughly one in $n^{k-1}$. Instead each [`Vec`] is built from a restricted growth string, which
  says which positions share an element, and a [`Vec`] of distinct elements, which says what the
  string's symbols stand for in order of first appearance; every such [`Vec`] arises from exactly
  one such pair.
- `restricted_growth_strings`, generating the [`Vec`]s $(a_0, \ldots, a_{k-1})$ with $a_0 = 0$ and
  $a_i \leq 1 + \max(a_0, \ldots, a_{i-1})$ that use a given number of distinct symbols. These
  correspond to the partitions of $k$ labeled objects into a given number of nonempty blocks, so the
  output length is a Stirling number of the second kind.
- `exhaustive_vecs_with_last` and its `_min_length`, `_length_range`, and
  `_length_inclusive_range` variants, along with `exhaustive_vecs_with_last_fixed_length` and
  `exhaustive_vecs_with_last_from_length_iterator`: a family that builds each [`Vec`] from two
  iterators, one supplying every element but the last and the other supplying the last. It exists
  for the sequences whose final element is special — the coefficients of a polynomial, whose
  leading coefficient may not be zero, are the motivating case. Generating all [`Vec`]s and
  filtering is far too slow, since the fraction that survives shrinks with the length; pairing a
  [`Vec`] of the earlier elements with a last element is unbalanced, since the pair generator
  cannot see that one half is a [`Vec`] and grows it far more slowly than the other. These
  generators instead reuse the length-two tuple machinery that `exhaustive_vecs_fixed_length`
  already uses, with an output-type map that sends every slot but the last to the first iterator
  and the last slot to the second, so that a last element is exactly as expensive as any other and
  the bit-distribution logic is shared rather than reimplemented. The lengths are stepped by a
  `bit_distributor_sequence` rather than a `ruler_sequence`: with the ruler sequence half of the
  output has length one, which for polynomials means half of them are constants.
- `random_vecs_with_last` and its `_min_length`, `_length_range`, and `_length_inclusive_range`
  variants, along with `random_vecs_with_last_fixed_length` and
  `random_vecs_with_last_from_length_iterator`, the random counterparts of the above. The lengths
  come from a geometric distribution, as they do for `random_vecs`, except where a length range is
  given and they are uniform.
- A new `UnsignedPolynomial<T>` type, a univariate polynomial whose coefficients are unsigned
  primitive integers, held as a
  [`Vec`] in ascending order of degree with no trailing zero — so the zero polynomial has no
  coefficients at all, and `Eq`, `Hash`, and `Debug` can be derived. It is the small-coefficient
  member of the polynomial family, meant for polynomials modulo a small modulus, and it mirrors
  `NaturalPolynomial` in malachite-nz. `from_coefficients_asc` normalizes a [`Vec`] by trimming
  trailing zeros; `coefficients_asc` lends the coefficients back and `into_coefficients_asc`
  hands them over; `degree` returns an `Option<u64>`, `None` for the zero polynomial, since zero
  has no degree rather than a degree of zero; `coefficient` reads a coefficient at any index,
  giving zero past the degree however far past, and `leading_coefficient` reads the last one,
  giving zero for the zero polynomial. Unlike its bignum counterparts, which lend a reference,
  both return a [`u64`] by value. `mutate_coefficient` allows a coefficient to be changed in
  place, growing the polynomial with zeros if the index is past the degree and trimming it
  afterwards, the way `Rational::mutate_numerator` does. `Zero` is implemented, with a `const`
  `ZERO`, and `one` and `two` are provided as functions rather than constants, since a nonzero
  constant polynomial owns a heap allocation. `From` is implemented for everything a [`u64`] can
  be converted from.
- `Display` and `FromStr` for `UnsignedPolynomial`, along with `to_string_with` and `from_string_with`,
  which name the variable with any `VarScheme` rather than the default `x`. The format is the one
  Azurite uses: terms in decreasing degree joined with `+`, an exponent of one elided, a
  coefficient of one elided, and `*` between a coefficient and its variable, so that `x^2+3*x+2`
  reads back as itself. The zero polynomial is `0`. `FromStr` accepts the terms in any order and
  tolerates a leading zero or an explicit `^1`, but rejects anything `Display` would never write,
  such as a zero coefficient in a term, two terms of the same degree, or a space.
- `ToLatex` and `ToTypst` for `UnsignedPolynomial`, with `to_latex_string_with` and
  `to_typst_string_with` for a named variable. Neither language writes the `*`, and LaTeX braces
  an exponent of more than one digit while Typst parenthesizes it.
- Exhaustive and random `UnsignedPolynomial` generators, in `unsigned_polynomial::exhaustive` and
  `unsigned_polynomial::random`: `exhaustive_unsigned_polynomials` and `random_unsigned_polynomials`, each with
  `_with_degree`, `_min_degree`, `_degree_range`, and `_degree_inclusive_range` variants and a
  `_from_iterators` form that takes the coefficient and leading-coefficient iterators; and
  `striped_random_unsigned_polynomials`, with the same variants, whose coefficients have long runs of
  equal bits. They are built on the `_with_last` [`Vec`] generators above. The degree-bounded
  generators never produce the zero polynomial, which has no degree and so falls in no range.
  Unlike the bignum generators, none of them takes a mean bit count: a [`u64`] coefficient is
  uniform over its whole range.
- A new `enable_serde` feature, and with it malachite-base's first
  [serde](https://serde.rs/) support: `Serialize` and `Deserialize` for `UnsignedPolynomial`. A
  polynomial is its coefficients, so the encoding is the list of them and nothing around it —
  `x^2+3*x+2` is `[2,3,1]`, and the zero polynomial, which has no coefficients, is `[]`.
  Deserializing goes through `TryFrom` rather than building the value directly, so that it can
  re-check the one thing that makes a polynomial's representation unique: a list whose last
  coefficient is zero is rejected rather than quietly trimmed, since accepting it would build a
  polynomial that an equal one would not match.
- Fixed a bug in `random_hash_sets` and the rest of the `sets::random` [`HashSet`] generators,
  which under `no_std` returned a `hashbrown::HashSet` and under `std` — where the rest of the
  crate returns `std::collections::HashSet` — returned one too. The module gated its import on the
  `test_build` feature where its siblings gate on `std`, three otherwise character-identical
  blocks apart. It went unnoticed because malachite-base dev-depends on itself with `test_build`
  on, so the crate's own doctests unified the feature on and saw the right type; a downstream
  crate with default features got `exhaustive_hash_sets` and `random_hash_sets` returning
  different types. The fix also makes 31 `vecs::random` doctests reachable that had been silently
  skipped, and `build.sh` gained a step that runs the malachite-base doctests with `random` but
  without `test_build`, which is the configuration that would have caught it.
- `Ord` and `PartialOrd` for `UnsignedPolynomial`, comparing two polynomials by how they behave for
  large arguments: the greater one is the one that is eventually greater, $f(p, q) = \lim_{x \to
  \infty} \operatorname{cmp}(p(x), q(x))$. The coefficients are read as the numbers they are, so
  the values compared are the ones a polynomial over the integers would take, not ones reduced by
  any modulus. The limit always exists, since $p - q$ has finitely many roots and past the largest
  of them its sign is its leading coefficient's and never changes again; that also makes the order
  total and agreeing with `Eq`. No evaluation is needed to find it — a higher degree eventually
  outgrows a lower one whatever the coefficients, so the degrees decide first and the zero
  polynomial is least, and equal degrees are decided by the highest-degree coefficient at which the
  two differ. This is the order that makes the polynomials an ordered ring, and restricted to the
  constants it is the order on the [`u64`]s. It is deliberately not a well-order, and no order
  compatible with addition can be: $x > x - 1 > x - 2 > \ldots$ descends forever.
- Two new traits, `Height` and `HeightRef`. A value's height is the largest of the magnitudes it
  is built from: for a rational number $p/q$ in lowest terms it is $\max(|p|, q)$, the measure in
  which Diophantine approximation bounds are usually stated, and for something built out of
  several such values — a polynomial, or a complex number — it is the largest of their heights.
  `Height` has `to_height`, `into_height`, and `height_significant_bits`, the last of which is
  usually cheaper than materializing the height, bit length being monotone. `HeightRef` adds
  `height_ref`, which lends the height rather than building it; it is a separate trait because
  not every height is a value the type already holds.
- `Height` for `UnsignedPolynomial`: the largest of its coefficients, with the zero polynomial, having
  no coefficients, having height 0. This is `fmpz_poly_height` from `fmpz_poly/norms.c`, FLINT
  3.6.0, for nonnegative coefficients. Its `Output` is a [`u64`] rather than a
  [`Natural`](malachite_nz::natural::Natural), and it does not implement `HeightRef`, there being
  no clone to avoid.
- `ModIsReduced` and `ModPowerOf2IsReduced` for `UnsignedPolynomial`. A polynomial is reduced modulo
  $m$ when every one of its coefficients is, and asking that of every coefficient is asking it of
  the largest — so both are questions about the polynomial's height. `mod_is_reduced` compares the
  height against the modulus, and `mod_power_of_2_is_reduced` is
  `height_significant_bits() <= pow`, the same shape as the primitive integer implementation,
  which never builds the height at all since bit length is monotone. The zero polynomial, having
  no coefficients, is reduced modulo everything, including $2^0$.
- `exhaustive_unsigned_polynomials_reduced_mod_power_of_2`,
  `random_unsigned_polynomials_reduced_mod_power_of_2`, and
  `striped_random_unsigned_polynomials_reduced_mod_power_of_2`, generating the `UnsignedPolynomial`s that
  are reduced modulo $2^k$ — those whose coefficients are all less than $2^k$, for which
  `mod_power_of_2_is_reduced` returns `true`. Restricting the coefficients does not bound the
  degree, so the output is still infinite and degrees still spread out; only the coefficients are
  confined. The striped generator needs no separate restriction, a striped bit chunk $k$ bits wide
  being exactly a value below $2^k$: the chunk width is what makes the polynomial reduced. All
  three panic on a `pow` of 0, the only polynomial reduced modulo $2^0$ being the zero polynomial,
  which leaves no leading coefficient to choose.
- `exhaustive_unsigned_polynomials_reduced_mod`, `random_unsigned_polynomials_reduced_mod`, and
  `striped_random_unsigned_polynomials_reduced_mod`, the same for an arbitrary modulus rather than a
  power of 2: the `UnsignedPolynomial`s whose coefficients are all less than $m$, for which
  `mod_is_reduced` returns `true`. Where $m$ is a power of 2 the exhaustive one generates exactly
  what the power-of-2 version does, in the same order. The striped one differs from its power-of-2
  counterpart in needing two restrictions rather than one — an arbitrary $m$ is not a bit-width
  boundary, so the coefficients are striped values drawn from a range instead of bit chunks whose
  width already bounds them. All three panic on an `m` below 2: nothing is reduced modulo 0, and
  the only polynomial reduced modulo 1 is the zero polynomial, which leaves no leading coefficient
  to choose.
- `ModPowerOf2` and `ModPowerOf2Assign` for `UnsignedPolynomial`, reducing every coefficient modulo
  $2^k$. Both by-value and by-reference forms are provided, as for [`Natural`]. The result is
  always reduced, so `mod_power_of_2_is_reduced` holds for it and reducing again changes nothing.
  Reducing can lower the degree, and can give the zero polynomial: a leading coefficient that is a
  multiple of $2^k$ becomes zero, and a polynomial holds no trailing zero coefficients, so
  $4x^2 + 3$ modulo $4$ is the constant $3$ rather than a quadratic with a zero leading
  coefficient, and $4x^2 + 4x + 4$ modulo $4$ is zero. Interior zeros are untouched: only the
  leading ones are dropped.
- `Rem<u64>`, `RemAssign<u64>`, `Mod<u64>` and `ModAssign<u64>` for `UnsignedPolynomial`. Each reduces
  every coefficient modulo `m`, in by-value and by-reference forms. A `UnsignedPolynomial`'s
  coefficients are never negative, so `mod_op` and `%` agree everywhere and the mod-family names
  are the same operation. $p \% m$ is the polynomial whose $i$th coefficient is $p_i \% m$, which
  is `fmpz_poly_scalar_mod_fmpz` from `fmpz_poly/scalar_mod_fmpz.c`, FLINT 3.6.0, for nonnegative
  coefficients — and it is the remainder of dividing by the constant polynomial `m` under the
  convention that applies over the integers, where a remainder is bounded coefficient by
  coefficient rather than by degree. Over a field the answer would be 0 instead, a remainder there
  having to be of lower degree than the divisor. As for `ModPowerOf2`, reducing can lower the
  degree and can give the zero polynomial, since a leading coefficient that is a multiple of `m`
  becomes zero and a polynomial holds no trailing zero coefficients. Where `m` is a power of 2
  this agrees with `mod_power_of_2`, which reaches the same answer by masking rather than
  dividing. Panics on a zero divisor — including for the zero polynomial, which has no
  coefficients for the division to fail on.  ### malachite-nz
- A new `NaturalPolynomial` type, a univariate polynomial whose coefficients are [`Natural`]s, held
  as a [`Vec`] in ascending order of degree with no trailing zero — so the zero polynomial has no
  coefficients at all, which is what makes the representation unique and lets `Eq`, `Hash`, and
  `Debug` be derived. `from_coefficients_asc` normalizes a [`Vec`] by trimming trailing zeros;
  `coefficients_asc` lends the coefficients back and `into_coefficients_asc` hands them over;
  `degree` returns an `Option<u64>`, `None` for the zero polynomial, since zero has no degree
  rather than a degree of zero; `coefficient` lends a coefficient at any index, giving a reference
  to zero past the degree however far past, and `leading_coefficient` lends the last one, giving
  zero for the zero polynomial. `mutate_coefficient` allows a coefficient to be changed in place,
  growing the polynomial with zeros if the index is past the degree and trimming it afterwards, the
  way `Rational::mutate_numerator` does. `Zero` is implemented, with a `const` `ZERO`, and `one`
  and `two` are provided as functions rather than constants, since a nonzero constant polynomial
  owns a heap allocation. `From` is implemented for everything a [`Natural`] can be converted from.
- `Display` and `FromStr` for `NaturalPolynomial`, along with `to_string_with` and
  `from_string_with`, which name the variable with any `VarScheme` rather than the default `x`; and
  `ToLatex` and `ToTypst`, with `to_latex_string_with` and `to_typst_string_with`. The format is
  the one Azurite uses: terms in decreasing degree joined with `+`, an exponent of one elided, a
  coefficient of one elided, and `*` between a coefficient and its variable, so that `x^2+3*x+2`
  reads back as itself; the zero polynomial is `0`. `FromStr` accepts the terms in any order and
  tolerates a leading zero or an explicit `^1`, but rejects anything `Display` would never write.
  Neither LaTeX nor Typst writes the `*`; LaTeX braces an exponent of more than one digit and Typst
  parenthesizes it.
- Exhaustive and random `NaturalPolynomial` generators, in `natural_polynomial::exhaustive` and
  `natural_polynomial::random`: `exhaustive_natural_polynomials` and `random_natural_polynomials`,
  each with `_with_degree`, `_min_degree`, `_degree_range`, and `_degree_inclusive_range` variants
  and a `_from_iterators` form that takes the coefficient and leading-coefficient iterators; and
  `striped_random_natural_polynomials`, with the same variants. They are built on the `_with_last`
  [`Vec`] generators new to malachite-base. The degree-bounded generators never produce the zero
  polynomial, which has no degree and so falls in no range.
- A new `IntegerPolynomial` type, the same thing over [`Integer`]s, with the same functions plus
  `negative_one`. Its string format differs only in its signs: a term is joined to the one before
  it with `+` unless it already begins with `-`, a coefficient of `-1` is written as a bare `-`,
  and `FromStr` splits on a `-` while keeping it with the term that follows. Exhaustive, random,
  and striped random generators mirror the [`Natural`] ones.
- `From<UnsignedPolynomial>` for `NaturalPolynomial`, and `From<UnsignedPolynomial>` and
  `From<NaturalPolynomial>` for `IntegerPolynomial`: the widening conversions among the polynomial
  types, each of which loses nothing and cannot fail, since every [`u64`] is a [`Natural`] and
  every [`Natural`] is an [`Integer`]. The coefficients are converted one by one and the leading
  one stays nonzero, so the degree is unchanged and the written form is identical. The narrowing
  directions are not provided, since they can fail.
- `Serialize` and `Deserialize` for `NaturalPolynomial` and `IntegerPolynomial`, under the
  existing `enable_serde` feature. As for `UnsignedPolynomial`, the encoding is the coefficient list
  and nothing around it, each coefficient written the way a [`Natural`] or an [`Integer`] is, so
  that `x^2-3*x+2` is `["0x2","-0x3","0x1"]`; and a list whose last coefficient is zero is
  rejected rather than trimmed.
- `Ord` and `PartialOrd` for `NaturalPolynomial`, comparing two polynomials by how they behave for
  large arguments: the greater one is the one that is eventually greater, $f(p, q) = \lim_{x \to
  \infty} \operatorname{cmp}(p(x), q(x))$. The limit always exists, since $p - q$ has finitely many
  roots and past the largest of them its sign is its leading coefficient's and never changes again;
  that also makes the order total and agreeing with `Eq`. No evaluation is needed to find it — a
  higher degree eventually outgrows a lower one whatever the coefficients, so the degrees decide
  first and the zero polynomial is least, and equal degrees are decided by the highest-degree
  coefficient at which the two differ. This is the order that makes the polynomials an ordered
  ring, and restricted to the constants it is the order on the [`Natural`]s. It is deliberately not
  a well-order, and no order compatible with addition can be: $x > x - 1 > x - 2 > \ldots$ descends
  forever. A well-order weighing size against degree is a separate thing, and will live on a
  wrapper type rather than displace this one.
- `Ord` and `PartialOrd` for `IntegerPolynomial`, comparing two polynomials by how they behave for
  large arguments: the greater one is the one that is eventually greater, $f(p, q) = \lim_{x \to
  \infty} \operatorname{cmp}(p(x), q(x))$. Unlike over the [`Natural`]s, a higher degree alone does
  not settle it. A polynomial of higher degree does dominate, so the difference's leading
  coefficient is its own — but that coefficient may be negative, in which case the dominating
  polynomial runs off to $-\infty$ and is the *smaller* of the two, so $-x^3 < x^2$ and the zero
  polynomial sits above every polynomial with a negative leading coefficient. Equal degrees are
  decided by the highest-degree coefficient at which the two differ. This is the order that makes
  the polynomials an ordered ring, and restricted to the constants it is the order on the
  [`Integer`]s.
- `ShortlexIntegerPolynomial` and `ShortlexIntegerPolynomialRef`, wrappers supplying a second
  order: polynomials are compared first by degree and then, in case of a tie, by their coefficients
  from highest to lowest, with the zero polynomial first. This is FLINT's order for polynomials —
  what `fmpq_poly_cmp` implements, and the only polynomial ordering FLINT has, since there is no
  `fmpz_poly_cmp`. It differs from the bare [`Ord`] exactly where the degrees differ and the
  dominating polynomial's leading coefficient is negative: shortlex has $-x^3 > x^2$ where the
  asymptotic order has $-x^3 < x^2$. Neither is a well-order, and ordering by degree first does not
  make one: $x > x - 1 > x - 2 > \ldots$ all have degree 1, so the chain descends forever under
  either. No order restricting to the usual order on the constants can be a well-order, the
  [`Integer`]s not being well-ordered. The wrappers follow `ComparableGaussianInteger`: one owns
  its value, one borrows it, both dereference to the polynomial, and the owning one serializes
  transparently.
- `Height` and `HeightRef` for `NaturalPolynomial` and `IntegerPolynomial`: the largest of the
  magnitudes of the coefficients, the zero polynomial having height 0, which is
  `fmpz_poly_height` from `fmpz_poly/norms.c`, FLINT 3.6.0. The height is one of the coefficients,
  so `height_ref` lends it; an [`Integer`] holds its magnitude as a [`Natural`], so that works for
  signed coefficients too. `into_height` moves the coefficient out rather than cloning it.
- `Height` and `HeightRef` for `GaussianInteger`: the larger of the magnitudes of its real and
  imaginary parts, which is again already held and so can be lent.
- `ModIsReduced` and `ModPowerOf2IsReduced` for `NaturalPolynomial`, as for `UnsignedPolynomial`:
  both are questions about the height, since a polynomial is reduced exactly when its largest
  coefficient is. `mod_is_reduced` borrows the height through `HeightRef` rather than cloning it.
- `random_naturals_less_than_power_of_2` and `striped_random_naturals_less_than_power_of_2`,
  generating [`Natural`]s below $2^k$. A [`Natural`] below $2^k$ is one with no more than $k$
  significant bits, and such a [`Natural`] can be built directly out of that many random bits, so
  unlike `random_naturals_less_than`, which draws a [`Natural`] of the right bit length and
  rejects it when it is too large, nothing here is ever drawn and thrown away. The striped version
  takes its bits from a `StripedBitSource`.
- `exhaustive_natural_polynomials_reduced_mod_power_of_2`,
  `random_natural_polynomials_reduced_mod_power_of_2`, and
  `striped_random_natural_polynomials_reduced_mod_power_of_2`, generating the
  `NaturalPolynomial`s that are reduced modulo $2^k$ — those whose coefficients are all less than
  $2^k$, for which `mod_power_of_2_is_reduced` returns `true`. The random ones are built on the
  two generators above, so a coefficient is reduced by construction rather than by rejection.
  Restricting the coefficients does not bound the degree, so the output is still infinite. Unlike
  `random_natural_polynomials`, these take no mean bit count: the coefficients are uniform over
  the whole of $[0, 2^k)$. All three panic on a `pow` of 0, the only polynomial reduced modulo
  $2^0$ being the zero polynomial, which leaves no leading coefficient to choose. The exhaustive
  one enumerates the same polynomials in the same order as
  `exhaustive_unsigned_polynomials_reduced_mod_power_of_2`.
- `exhaustive_natural_polynomials_reduced_mod`, `random_natural_polynomials_reduced_mod`, and
  `striped_random_natural_polynomials_reduced_mod`, the same for an arbitrary modulus rather than
  a power of 2: the `NaturalPolynomial`s whose coefficients are all less than $m$, for which
  `mod_is_reduced` returns `true`. Unlike the power-of-2 versions, a coefficient here cannot be
  built reduced — $m$ is not a bit-width boundary, so a [`Natural`] of the right bit length may
  still be too large and has to be drawn again, though at most half of the draws are wasted since
  $m$ is more than half of the next power of 2. Where $m$ is a power of 2 the exhaustive one
  enumerates the same polynomials in the same order as the power-of-2 version, and as
  `exhaustive_unsigned_polynomials_reduced_mod`. The two random generators take `m` by reference, since
  they clone it once per coefficient source rather than consuming it. All three panic on an `m`
  below 2.
- `ModPowerOf2` and `ModPowerOf2Assign` for `NaturalPolynomial`, reducing every coefficient modulo
  $2^k$, in by-value and by-reference forms. As for `UnsignedPolynomial`, the result is always reduced
  and reducing can lower the degree, since a leading coefficient that is a multiple of $2^k$
  becomes zero and a polynomial holds no trailing zero coefficients. Unlike `UnsignedPolynomial`, there
  is no power wide enough to leave every polynomial alone: a [`Natural`] coefficient can exceed
  any $2^k$, so a $k$ past 64 is as meaningful as a small one, and there is no width short circuit
  to take.

### malachite-q

- A new `RationalPolynomial` type, a univariate polynomial whose coefficients are [`Rational`]s.
  It is represented the way FLINT's `fmpq_poly_t` is: an `IntegerPolynomial` numerator and a
  single positive [`Natural`] denominator, reduced so that the denominator shares no factor with
  the content of the numerator, with the zero polynomial given the denominator 1. One denominator
  for the whole polynomial rather than one per coefficient keeps the representation unique, so
  `Eq`, `Hash`, and `Debug` can be derived, and makes arithmetic a matter of integer polynomial
  arithmetic and one rational reduction. `numerator_ref` and `denominator_ref` lend the two parts,
  `into_numerator_and_denominator` hands them over, and `from_numerator_and_denominator` builds a
  polynomial from a pair, canonicalizing it.
- The `RationalPolynomial` coefficient functions. Because no [`Rational`] coefficient exists in
  memory to point at, these differ from their `NaturalPolynomial` counterparts: `coefficient` and
  `leading_coefficient` return a [`Rational`] by value rather than a reference, and where
  `NaturalPolynomial` lends a slice through `coefficients_asc`, `RationalPolynomial` has
  `to_coefficients_asc` and `into_coefficients_asc`, both of which build a [`Vec`].
  `from_coefficients_asc` takes a [`Vec`] of [`Rational`]s, clearing their denominators into one;
  `mutate_coefficient` materializes the coefficient, runs the closure on it, and rebuilds.
  `degree`, `Zero`, `one`, `two`, `negative_one`, and `From` behave as they do for the other
  polynomial types.
- `Display`, `FromStr`, `ToLatex`, and `ToTypst` for `RationalPolynomial`, with the same
  `_with` variants, writing each coefficient as a [`Rational`] does: `1/2*x+1/3` in plain text,
  `\frac{1}{2}x+\frac{1}{3}` in LaTeX, and `frac(1, 2)x+frac(1, 3)` in Typst. Exhaustive, random,
  and striped random generators mirror the ones in malachite-nz.
- `From<UnsignedPolynomial>`, `From<NaturalPolynomial>`, and `From<IntegerPolynomial>` for
  `RationalPolynomial`, completing the widening conversions among the polynomial types. The
  `IntegerPolynomial` one is free: a `RationalPolynomial` is a numerator and a denominator, and an
  `IntegerPolynomial` is already the numerator it needs, so it is moved rather than copied and the
  denominator is 1 — a pair that is canonical whatever the numerator is, since everything is
  coprime with 1, so no content or GCD is computed. The other two convert their coefficients and
  then take that path.
- `Serialize` and `Deserialize` for `RationalPolynomial`, under the existing `enable_serde`
  feature. Unlike the other polynomial types, this one writes both of the parts it is made of
  rather than a coefficient list: `1/2*x+1/3` is `{"n":["0x2","0x3"],"d":"0x6"}`. Writing the
  coefficients instead would make the encoding uniform with the others, but it would also throw
  away the shared denominator and make deserializing clear every coefficient's denominator over
  again. Deserializing re-checks all three conditions that make the pair canonical — a nonzero
  denominator, a denominator of 1 for the zero polynomial, and a numerator whose content shares
  no factor with the denominator — the way deserializing a [`Rational`] does.
- `Ord` and `PartialOrd` for `RationalPolynomial`, comparing two polynomials by how they behave
  for large arguments, as the other polynomial types do: $f(p, q) = \lim_{x \to \infty}
  \operatorname{cmp}(p(x), q(x))$. As over the [`Integer`]s, a dominating polynomial with a
  negative leading coefficient runs off to $-\infty$ and is the smaller of the two.
  Nothing is reduced along the way and the difference is never formed. Writing the two as $P/a$
  and $Q/b$ with $a, b > 0$, the deciding sign is that of the leading coefficient of $bP - aQ$,
  and the $ab$ underneath it is positive and so irrelevant — so a coefficient comparison is one of
  $b P_i$ against $a Q_i$, with no GCD, no LCM, and no common denominator built. The screens, in
  order: degrees settle it with no arithmetic at all; equal denominators cancel, which covers
  every pair of integer-coefficient polynomials; a denominator of 1 scales nothing; within a
  coefficient, differing signs settle it outright, since a positive denominator leaves a sign
  alone; and the two products' bit counts settle it whenever they are more than one apart, a
  product's bit count being the sum of its factors' give or take one. Everything derived from the
  denominators is computed once, outside the loop.
- `ShortlexRationalPolynomial` and `ShortlexRationalPolynomialRef`, wrappers supplying the order
  FLINT gives polynomials: first by degree, then by coefficients from highest to lowest, with the
  zero polynomial first. This is what `fmpq_poly_cmp` implements, so it is the wrapper rather than
  the bare type that matches FLINT. It differs from the bare [`Ord`] exactly where the degrees
  differ and the dominating polynomial's leading coefficient is negative. Once the degrees agree
  the two orders are identical, so they share one coefficient comparison — the reduction-free one
  above, which is also the shape `_fmpq_poly_cmp` uses.
- `Height` and `HeightRef` for `Rational` — the trait forms of the methods it already had, plus
  the new `height_ref`, which lends whichever of the numerator and denominator is the larger
  rather than cloning it — and for `GaussianRational`, whose height is the larger of its two
  parts' heights and so is one of four stored magnitudes.
- `Height` for `RationalPolynomial`: the largest of the heights of its coefficients, with the zero
  polynomial taking the height of the rational number 0, which is 1. This is the one type in the
  family that cannot implement `HeightRef`. The coefficients share one denominator, and a
  coefficient's numerator may still have a factor in common with it even when the polynomial as a
  whole is canonical — `1/2*x+1/3` is stored as `(3*x+2)/6`, so the largest number it holds is 6,
  while its height is 3 — so every coefficient has to be reduced before its height is known, and
  the answer is not among the values the polynomial holds.

### Documentation

- Fixed ten `Float` logarithm doc comments whose set braces did not render. Rustdoc runs doc
  comments through Markdown before KaTeX sees them, and Markdown drops a backslash before ASCII
  punctuation, so a `\{` written in a math span reached KaTeX as a bare `{`, which groups silently
  instead of being typeset; `$x\in\{\pm\infty,\pm0.0\}$` was rendering without its braces. A new
  `math-escape-check.py`, run as part of `additional-lints.sh`, now fails the lint sweep on any
  math span whose backslash would not survive Markdown — including the `\%`, `\$`, `\#`, and `\&`
  cases, where the bare character starts a comment, ends the span, or is a macro parameter.

## 0.12.0 — 2026-09-20

### Breaking and behavioral changes

- `PrimitiveInt` and `PrimitiveFloat` have gained supertraits, so a type outside Malachite that
  implements either of them must now implement those too. Both gained `AbsSquared`,
  `AbsSquaredAssign`, `Conjugate`, `ConjugateAssign`, `IsGaussianInteger`, `IsReal`, `IsUnit`,
  `CanonicalizeUnit`, `CanonicalizeUnitAssign`, and `CanonicalUnitIPow`; `PrimitiveInt` also
  gained `IsPowerOf2`, and `PrimitiveFloat` also gained `DottieNumber`. Code that uses these
  traits only as bounds is unaffected, and gains the new methods on every primitive type.
- The `malachite` crate now enables the `floats` feature by default, so `Float` and the `float`
  module are re-exported at its root. This costs nothing to compile, since `malachite-float` was
  already being built by every default build, but a glob import of `malachite` alongside another
  `Float` is now ambiguous.
- The `malachite` crate's `std`, `random`, `enable_serde`, and `32_bit_limbs` features no longer
  drag the optional sub-crates into the build; each now configures whichever of
  `naturals_and_integers`, `rationals`, and `floats` is enabled. Those three build on one another
  in turn, so asking for `floats` alone is now enough to get a usable `Float`. A build combining
  `--no-default-features` with only a modifier feature, such as `--features std`, now gets
  malachite-base alone instead of the whole workspace.

### malachite-base

- New traits for complex types downstream (nothing in malachite-base implements them): the
  constant traits `I` and `NegativeI`, and the conversion traits `ImaginaryFrom` and
  `ImaginaryInto`.
- New `AbsSquared` and `AbsSquaredAssign` traits for computing the squared absolute value of a
  number, $|x|^2$, implemented for all numeric types. For real types this is the same as
  squaring; for `GaussianInteger` and `GaussianRational`, `abs_squared` is the sum of the
  squares of the real and imaginary parts (the norm), returned as an `Integer` or `Rational`
  respectively, and `abs_squared_assign` replaces the value with the purely real $|x|^2$
  embedded in the same type. Both traits are supertraits of `PrimitiveInt` and
  `PrimitiveFloat`.
- New `Conjugate` and `ConjugateAssign` traits for computing the complex conjugate of a number,
  implemented for all numeric types. A real number is its own conjugate, so for the real types
  these are the identity; the trivial implementations let generic code use conjugation
  uniformly. For `GaussianInteger` and `GaussianRational` the sign of the imaginary part is
  flipped. Both traits are supertraits of `PrimitiveInt` and `PrimitiveFloat`.
- New `IsGaussianInteger` and `IsReal` traits alongside `IsInteger`, implemented for all
  primitive types (and, in the other crates, all bignum types). For every type,
  `x.is_integer() == x.is_gaussian_integer() && x.is_real()`; for floating-point types, `NaN`
  and the infinities are neither real nor Gaussian integers.
- New `MulI`, `MulIAssign`, `DivI`, and `DivIAssign` traits for multiplying or dividing a number
  by $i$, the imaginary unit — quarter turns in the complex plane, which need no multiplication.
  Nothing in malachite-base implements them; `GaussianInteger` and `GaussianRational` do.
- New `IsUnit`, `CanonicalUnitIPow`, `CanonicalizeUnit`, and `CanonicalizeUnitAssign` traits,
  ports of FLINT's `fmpzi_is_unit`, `fmpzi_canonical_unit_i_pow`, and `fmpzi_canonicalise_unit`:
  a unit test, and canonicalization of a complex number under multiplication by $\pm 1$ and
  $\pm i$, choosing the associate whose argument lies in $(-\pi/4, \pi/4]$. They are
  implemented for all numeric types, so that generic code can normalize associates uniformly:
  for a real type the units are $\pm 1$ (1 alone for unsigned types, and every finite nonzero
  value for floats), the canonical form is the absolute value, and the power of $i$ is 2 for
  negative values and 0 otherwise. All four are supertraits of `PrimitiveInt` and
  `PrimitiveFloat`.
- `IsPowerOf2` is now implemented for the signed primitive integers (negative values are never
  powers of 2), and is a supertrait of `PrimitiveInt` rather than only of `PrimitiveUnsigned`.

### malachite-nz

- Multiplying a `Natural`, `Integer`, `Rational`, or `Float` by itself through aliased
  references (`&x * &x`) now routes to the squaring algorithm, which is faster, and adding a
  `Float` to itself through aliased references routes to a doubling shift. This extends an
  existing convention: several operations, such as `Integer` and `Natural` addition and
  `Natural`'s modular operations, already detect aliased operands and take shortcuts.
- A new `GaussianInteger` type, parallel to `Natural` and `Integer`: a pair of public `Integer`
  fields `real` and `imaginary`, always valid. So far it has the constants 0, 1, 2, -1, i, and
  -i; `Display` and `FromStr` (strict about term structure, permissive about degenerate
  coefficients like `"1i"` and `"0i"`); blanket `From` and `ImaginaryFrom` conversions from
  every type that converts to `Integer`; serde support; and exhaustive, random, and
  striped-random generators, wired into the demo, benchmark, and property-test machinery.
  `GaussianInteger` also implements `IsInteger`, `IsGaussianInteger`, and `IsReal` (and
  `Named`), and `Natural` and `Integer` implement the two new traits (trivially).
- The first arithmetic operations for the Gaussian types: `Neg` and `NegAssign` (negating both
  parts), `Conjugate` and `ConjugateAssign` (flipping the sign of the imaginary part), and
  componentwise addition and subtraction — `Add`, `Sub`, `AddAssign`, and `SubAssign`, in all
  the usual ownership variants — and multiplication (`Mul` and `MulAssign`) for
  `GaussianInteger` and `GaussianRational`. `GaussianInteger` multiplication uses FLINT's
  `fmpzi_mul` strategy: double-word arithmetic when all four parts fit in a signed word, a
  three-multiplication Karatsuba scheme for large balanced operands, and the fused
  `mul_add_mul`/`mul_sub_mul` kernels otherwise; `GaussianRational` multiplication uses the
  fused kernels. Both types also implement `Square` and `SquareAssign`; `GaussianInteger`
  squaring uses FLINT's `fmpzi_sqr` strategy, which prefers squarings over general
  multiplications and short-circuits purely real and purely imaginary values, and
  `GaussianRational` squaring uses the same $a^2 - b^2$, $2ab$ scheme, profiting from the fact
  that squaring a reduced fraction requires no GCD computations. Multiplying a Gaussian value
  by itself through aliased references routes to the squaring algorithm automatically. Both
  types also implement iterator `Sum` and `Product` (by value and by reference), mirroring
  their component types' strategies: `GaussianInteger` sums by accumulating with `+=` like
  `Integer`, `GaussianRational` sums in a balanced binary-tree order like `Rational`, which
  tends to keep intermediate denominators small, and both types multiply in a balanced
  binary-tree order, short-circuiting to zero when any factor is zero, like all four real
  bignum types.
- `OrdAbs` and `PartialOrdAbs` implementations for `GaussianInteger` (and, in malachite-q, for
  `GaussianRational`), comparing absolute values — distances from the origin. Componentwise and
  crosswise part comparisons decide most cases; the squared absolute values are only computed
  when both pairings strictly conflict.
- The full `PartialEq` matrix for the Gaussian types, completing the equality-operator
  convention that mixed-type comparisons get a full matrix: `GaussianInteger` can be compared
  with `Integer`, `Natural`, primitive integers, and primitive floats;
  `GaussianRational` (in malachite-q) with all of those plus `Rational` and `GaussianInteger`;
  and `Float` (in malachite-float) with both Gaussian types. All comparisons work in both
  directions. A Gaussian value equals a real one exactly when its imaginary part is zero and
  its real part is equal; no Gaussian value equals an infinity or NaN.
- The same matrix for `EqAbs`, testing whether absolute values — for complex numbers, distances
  from the origin — are equal, so that $3+4i$ is equal in absolute value to $5$. Comparisons
  against other Gaussian values delegate to the `OrdAbs` screens, and comparisons against real
  values only compute squared absolute values when both components are smaller in absolute
  value than the real operand, mirroring the `OrdAbs` strategy of computing squares only as a
  last resort. A non-integer float can never equal a Gaussian integer's absolute value (its odd
  mantissa squares to an odd numerator), and infinities and NaN are never equal in absolute
  value to anything.
- The same matrix for `PartialOrdAbs`, ordering by absolute value, so that $3+4i$ is greater in
  absolute value than $4$ and less than $6$. The screens are the ordering counterparts of the
  `EqAbs` ones: against a real value, a Gaussian value with two nonzero components is greater in
  absolute value unless both components are smaller in absolute value than the real operand,
  and only then are the squared absolute values compared; comparisons with a float square the
  float exactly (as an odd square times a power of two, or as a `Rational`) rather than
  rounding. NaN is incomparable to everything, and the infinities are greater in absolute value
  than every Gaussian value.
- `PowerOf2` and `IsPowerOf2` for the Gaussian types: `GaussianInteger::power_of_2(k)` and
  `GaussianRational::power_of_2(k)` (the latter also for negative `k`) produce purely real
  powers of 2, and `is_power_of_2` is true only for purely real, positive powers of 2 — $i$ and
  its multiples do not count. `Integer` also gains `IsPowerOf2`, which `Natural` and `Rational`
  already had; negative integers are never powers of 2 (and, in malachite-base, so does every
  signed primitive integer).
- `Shl` and `ShlAssign` for `GaussianInteger` by any unsigned primitive integer, shifting both
  parts (multiplying by a power of 2), in value, reference, and in-place variants. Signed shift
  amounts are deliberately not supported for `GaussianInteger`, since a negative amount would
  be a right shift and exact division by a power of 2 is not generally possible; in malachite-q,
  `GaussianRational` supports both unsigned and signed shift amounts, a negative amount dividing
  both parts exactly, and likewise `Shr` and `ShrAssign` by unsigned and signed amounts.
- `MulI`/`MulIAssign` and `DivI`/`DivIAssign` for both Gaussian types: multiplying by $i$ maps
  $a + bi$ to $-b + ai$ and dividing by $i$ maps it to $b - ai$, by swapping the parts and
  negating one of them.
- `Reciprocal` and `ReciprocalAssign` for `GaussianRational`: the conjugate divided by the squared
  absolute value, with purely real and purely imaginary values reducing to a single `Rational`
  reciprocal. Panics on zero, like `Rational`'s.
- `Div`, `DivAssign`, and `CheckedDiv` for `GaussianRational` in all the usual ownership
  variants. A purely real divisor divides both parts, a purely imaginary divisor does the same
  and then turns the result a quarter turn, and any other divisor multiplies by its reciprocal
  using the fused multiplication kernels. Division by zero panics; `checked_div` returns `None`.
- `IsUnit`, `CanonicalUnitIPow`, `CanonicalizeUnit`, and `CanonicalizeUnitAssign` for both
  Gaussian types, matching FLINT's choices tie for tie, and for `Natural`, `Integer` (and, in
  the other crates, `Rational` and `Float`), where canonical unit form is the absolute value.
  `GaussianInteger`'s units are $\pm 1$ and $\pm i$; `GaussianRational` is a field, so its
  units are the nonzero values.
- `SignificantBits` for both Gaussian types, summing the significant bits of the real and
  imaginary parts, and `GaussianInteger::max_significant_bits`, the larger of the two counts,
  which is FLINT's `fmpzi_bits` and the size measure its algorithm selection uses.
- `DivExact` and `DivExactAssign` for `GaussianInteger`, a port of FLINT's `fmpzi_divexact`: a
  purely real divisor divides both parts, a purely imaginary one does the same and turns the
  result a quarter turn, quotients below $2^{45}$ are recovered by rounding a double-precision
  evaluation of $x\bar{y}/N(y)$ (exact under the divisibility contract, with the operands scaled
  down above 500 bits), and larger quotients go through the exact conjugate-and-norm formula.
  Like the other `div_exact`s, an inexact division may panic or return a meaningless result.
- `DivRem` and `DivAssignRem` for `GaussianInteger`, a port of FLINT's `fmpzi_divrem`: the
  quotient is the exact quotient with each part rounded to the nearest integer, ties up, so the
  remainder satisfies $N(r) \leq N(y)/2$ (the Euclidean division of the Gaussian integers), and
  a dividend more than two bits smaller than the divisor short-cuts to quotient zero.
- The `/`, `/=`, `%`, and `%=` operators and `CheckedDiv` for `GaussianInteger`, with the same
  nearest-quotient rounding as `div_rem`; `/` skips computing the remainder.
- `GaussianInteger::remove_one_plus_i` and `remove_one_plus_i_assign`, a port of FLINT's
  `fmpzi_remove_one_plus_i`: they divide out the largest power of $1 + i$, the Gaussian prime
  above 2, by shifting out the common power of 2, fixing up the unit, and dividing once more by
  $1 + i$ when the parts share a 2-adic valuation, returning the exponent; zero stays zero with
  exponent 0.
- `Gcd` and `GcdAssign` for `GaussianInteger`, a port of FLINT's `fmpzi_gcd` without its lattice
  tier: once all four parts fit in 50 bits the Euclidean algorithm runs entirely in double
  precision, and until then it runs over an approximate nearest-quotient division. The result is
  in canonical unit form, so it is unique; $\gcd(0, 0) = 0$.
- `MulIPow` and `MulIPowAssign` traits in `malachite-base`, multiplication by $i^k$ for a `u64`
  exponent $k$ (only $k$ modulo 4 matters, and $i^{-k} = i^{3k}$), implemented for
  `GaussianInteger` and `GaussianRational` as a port of FLINT's `fmpzi_mul_i_pow_si`;
  `canonicalize_unit` is now defined through it.
- `Pow<u64>` and `PowAssign<u64>` for `GaussianInteger`, a port of FLINT's `fmpzi_pow_ui`: binary
  exponentiation over the fused squaring and multiplication, with purely real and purely
  imaginary bases reduced to an `Integer` power (times $i^n$ for the latter).
- `Pow<u64>`, `Pow<i64>`, and the matching `PowAssign`s for `GaussianRational`, structured like
  the `GaussianInteger` version; a negative exponent takes the reciprocal, and zero to a negative
  power panics, as for `Rational`.
- `ContentAndPrimitivePart`, `Content`, and `PrimitivePart` traits in `malachite-base`, for
  elements of vector spaces over the rationals with a distinguished integer lattice, implemented
  for `GaussianInteger` (content a `Natural`, the GCD of the parts) and `GaussianRational` (content
  a `Rational`, primitive part a `GaussianInteger` with coprime parts). `GaussianRational`'s power
  is computed through the split, so the intermediate values carry no denominators and there is one
  rational reduction per part at the end instead of several per squaring.
- `CheckedSqrt` for `GaussianInteger`, returning the principal square root (positive real part,
  or zero real part and non-negative imaginary part) of a perfect square and `None` otherwise. The
  root is read off the norm: $N = \sqrt{a^2 + b^2}$, then $x = \sqrt{(N + a) / 2}$ and
  $y = \pm \sqrt{(N - a) / 2}$ with the sign of $b$. `GaussianInteger::checked_sqrts` returns
  all the roots as a `Vec`: none, one for zero, or the principal root and its negative, in the
  canonical order of `ComparableGaussianInteger` (lexicographic by real part, then imaginary).
- `CheckedSqrt` and `checked_sqrts` for `GaussianRational` too, by clearing denominators: with
  $L$ the LCM of the denominators and $S = Lz$, $z$ is a square exactly when the Gaussian
  integer $SL$ is, and $\sqrt{z} = \sqrt{SL} / L$.
- `CheckedRoot<u64>` and `checked_roots` for `GaussianInteger`. A nonzero Gaussian integer has
  either no $n$th roots or exactly $\gcd(n, 4)$ of them; the principal one has argument in
  $(-\pi/g, \pi/g]$ for $g = \gcd(n, 4)$, which is the unique root for odd $n$, the
  `checked_sqrt` convention for $n \equiv 2 \pmod 4$, and the canonical unit form for
  $4 \mid n$. The odd part of the exponent is handled exactly through the norm and a Gaussian
  GCD, and the power of 2 by iterated square roots; no floating point is involved.
- `CheckedRoot<u64>` and `checked_roots` for `GaussianRational`, by clearing denominators: with
  $L$ the LCM of the denominators and $S = Lz$, any root $w$ has $Lw$ integral, so $Lw$ is the
  Gaussian integer root of $S L^{n-1}$.
- `ComparableGaussianInteger` and `ComparableGaussianIntegerRef`, wrappers around
  `GaussianInteger` (by value and by reference) that implement `Ord`, comparing
  lexicographically: first by real part, then by imaginary part. Since no total order on the
  complex numbers is compatible with arithmetic, `GaussianInteger` itself does not implement
  `Ord`; the wrappers provide a canonical order for sorting and for use as `BTreeMap` and
  `BTreeSet` keys, in the spirit of malachite-float's `ComparableFloat` and
  `ComparableFloatRef`.
- Conversions between `GaussianInteger` and the real types, completing the conversion matrix:
  `TryFrom` and `ConvertibleFrom` implementations for `Integer` (succeeding when the value is
  real), `Natural` (real and non-negative), all primitive integers (real and representable),
  and all primitive floats (real and exactly representable), plus `TryFrom` and
  `ConvertibleFrom` from primitive floats (finite integers), mirroring the corresponding
  `Rational` conversion families.

### malachite-q

- A new `GaussianRational` type, parallel to `GaussianInteger`: public `Rational` fields `real`
  and `imaginary`, always valid, with the same surface — constants, `Display` and `FromStr`
  (imaginary terms attach `i` to the numerator, as in `"i/2"` and `"2/3-5i/6"`), `From`
  conversions from every type that converts to `Rational` and componentwise conversions from
  `GaussianInteger`, a blanket `ImaginaryFrom`, serde support,
  and the full exhaustive/random/striped generator set with demo, benchmark, and property-test
  plumbing. `GaussianRational` also implements `IsInteger`, `IsGaussianInteger`, and `IsReal`
  (and `Named`), and `Rational` implements the two new traits.
- `ComparableGaussianRational` and `ComparableGaussianRationalRef`, wrappers around
  `GaussianRational` that implement `Ord` lexicographically (real part first, then imaginary
  part), mirroring malachite-nz's `ComparableGaussianInteger` wrappers: a canonical order for
  sorting and for `BTreeMap`/`BTreeSet` keys.
- Conversions between `GaussianRational` and the real types, completing the conversion matrix:
  `TryFrom` and `ConvertibleFrom` implementations for `Rational` (succeeding when the value is
  real), `GaussianInteger` (both parts integers), `Integer` (a real integer), `Natural` (a real
  non-negative integer), all primitive integers (real and representable), and all primitive
  floats (real and exactly representable), plus `TryFrom` and `ConvertibleFrom` from primitive
  floats (finite values) and from `GaussianInteger` (componentwise, added earlier in this
  cycle). `Rational` also gets `TryFrom` and `ConvertibleFrom` from `GaussianInteger`.

### malachite-float

- `Float` implements the new `IsGaussianInteger` and `IsReal` traits; a `Float` is real unless
  it is `NaN` or infinite.
- The first trigonometric function: `Cos` and `CosAssign` (new traits in malachite-base) for
  `Float`, with the usual `cos_prec_round`, `cos_prec`, `cos_round`, and `_ref`/`_assign`
  variants. Arguments of magnitude 4 or more are reduced modulo $2\pi$, so the cost grows with
  the input's exponent as well as with the precision. Inputs extremely close to an odd multiple
  of $\pi/2$ take a dedicated path that computes the distance to that multiple exactly, so the
  result is correct, and underflows correctly, even when the input agrees with the multiple to
  more than $2^{30}$ bits (a regime MPFR's wider exponent range never reaches).
- `Sin` and `SinAssign` (new traits in malachite-base) for `Float`, with the usual
  `sin_prec_round`, `sin_prec`, `sin_round`, and `_ref`/`_assign` variants: a port of `mpfr_sin`,
  which derives the sine from the cosine as $\pm\sqrt{1-\cos^2 x}$ after reducing arguments of
  magnitude 2 or more modulo $2\pi$. Inputs extremely close to a nonzero multiple of $\pi$ share
  the cosine's exact near-zero path, so the result is correct, and underflows correctly, even
  when the input agrees with the multiple to more than $2^{30}$ bits; the path is also taken as
  soon as the argument reduction detects such an input, where MPFR keeps raising its working
  precision instead.
- `sin_rational_prec_round` and `sin_rational_prec` (with `_ref` variants), the sine of a
  `Rational` as a `Float`, alongside the cosine versions. Small inputs are handled by the sine
  series in exact `Rational` arithmetic, so inputs too small to be `Float`s underflow correctly.
- `sin_with_period_prec_round`, `sin_with_period_prec`, and `sin_with_period_round` (with `_ref`
  and `_assign` variants), a port of `mpfr_sinu`: the sine of a `Float` measured in $u$ths of a
  turn. Multiples of a quarter of a turn are exact (a multiple of a half turn is a zero with the
  sign of the input, as IEEE 754-2019's `sinPi` specifies), as are the twelfths whose sine is
  $\pm1/2$, and thirds, sixths, eighths, and twentieths of a turn are computed from a single
  correctly rounded constant ($\sqrt3$, $\sqrt2$, or $\varphi$). Inputs within $2^{-2^{30}}$ of a
  half turn underflow correctly. `sin_with_period_rational_prec_round` and
  `sin_with_period_rational_prec` (with `_ref` variants) take a `Rational` instead, reaching the
  exact and closed-form cases directly, such as a twelfth or a twentieth of a turn.
  `primitive_float_sin_with_period` and `primitive_float_sin_with_period_rational` give the
  correctly rounded `f32` or `f64` results.
- `sin_pi_prec_round`, `sin_pi_prec`, and `sin_pi_round` (with `_ref` and `_assign` variants),
  `sin_pi_rational_prec_round` and `sin_pi_rational_prec` (with `_ref` variants), and
  `primitive_float_sin_pi` and `primitive_float_sin_pi_rational`: a port of `mpfr_sinpi`, the
  sine in half-turns, delegating to the `sin_with_period` family with a period of 2.
- `SinCos` and `SinCosAssign` (new traits in malachite-base) for `Float`, with the usual
  `sin_cos_prec_round`, `sin_cos_prec`, `sin_cos_round`, and `_ref`/`_assign` variants: a port of
  `mpfr_sin_cos`, computing the sine and cosine together with one argument reduction. The two
  results and their two ternary `Ordering`s are returned as a 4-tuple, and the `_assign` variants
  write the cosine to a second `&mut Float`. Inputs extremely close to a zero of either function
  take that function's exact near-zero path, so the results are correct, and underflow correctly,
  even when the input agrees with the zero to more than $2^{30}$ bits.
  `sin_cos_rational_prec_round` and `sin_cos_rational_prec` (with `_ref` variants) take a
  `Rational` instead, sharing the input rounding and, for inputs too large to be `Float`s, the
  reduction modulo $2\pi$ that dominates their cost. `primitive_float_sin_cos` and
  `primitive_float_sin_cos_rational` give the correctly rounded `f32` or `f64` pairs.
- `sin_cos_with_period_prec_round`, `sin_cos_with_period_prec`, and `sin_cos_with_period_round`
  (with `_ref` and `_assign` variants): the sine and cosine of a `Float` measured in $u$ths of a
  turn, together, with one argument reduction, one computation of $2\pi x/u$, and one `sin_cos`
  per iteration. MPFR has no such function. The results are those of the `sin_with_period` and
  `cos_with_period` families, including the exact quarter turns, the closed-form twelfths, sixths,
  and eighths, and the near-zero paths, so inputs within $2^{-2^{30}}$ of a multiple of a quarter
  turn underflow correctly. `sin_cos_with_period_rational_prec_round` and
  `sin_cos_with_period_rational_prec` (with `_ref` variants) take a `Rational` instead, and
  `primitive_float_sin_cos_with_period` and `primitive_float_sin_cos_with_period_rational` give
  the correctly rounded `f32` or `f64` pairs. `sin_cos_pi_prec_round`, `sin_cos_pi_prec`, and
  `sin_cos_pi_round` (with `_ref` and `_assign` variants), `sin_cos_pi_rational_prec_round` and
  `sin_cos_pi_rational_prec` (with `_ref` variants), and `primitive_float_sin_cos_pi` and
  `primitive_float_sin_cos_pi_rational` are the same in half-turns, delegating with a period of 2.
- `Tan` and `TanAssign` (new traits in malachite-base) for `Float`, with the usual
  `tan_prec_round`, `tan_prec`, `tan_round`, and `_ref`/`_assign` variants: a port of `mpfr_tan`,
  the sine and cosine together and their quotient in one Ziv loop. Unlike MPFR's, the result can
  overflow (an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$) or underflow (within that
  distance of a multiple of $\pi$); both are decided from exact brackets. `tan_rational_prec_round`
  and `tan_rational_prec` (with `_ref` variants) take a `Rational` instead, with a direct series
  bracket for tiny inputs, including those below the `Float` exponent range. `primitive_float_tan`
  and `primitive_float_tan_rational` give the correctly rounded `f32` or `f64` tangent.
- `tan_with_period_prec_round`, `tan_with_period_prec`, and `tan_with_period_round` (with `_ref`
  and `_assign` variants), a port of `mpfr_tanu`: the tangent of a `Float` measured in $u$ths of a
  turn. Multiples of a quarter turn are exact: a multiple of a half turn is a zero (reached from
  below, so that the function is odd), and an odd multiple of a quarter turn is a pole, returning
  an infinity; odd multiples of an eighth of a turn are $\pm1$, and thirds, sixths, and twelfths
  of a turn are computed from $\sqrt3$ or $\sqrt3/3$. Inputs within $2^{-2^{30}}$ of a multiple of
  a quarter turn overflow or underflow correctly. `tan_with_period_rational_prec_round` and
  `tan_with_period_rational_prec` (with `_ref` variants) take a `Rational` instead, reaching the
  exact and closed-form cases directly and needing no argument reduction beyond the exact one.
  `primitive_float_tan_with_period` and `primitive_float_tan_with_period_rational` give the
  correctly rounded `f32` or `f64` tangent; a primitive float is never merely close enough to a
  pole to overflow, but a `Rational` can be.
- `Atan` and `AtanAssign` (new traits in malachite-base) for `Float`, with the usual
  `atan_prec_round`, `atan_prec`, `atan_round`, and `_ref`/`_assign` variants: a port of
  `mpfr_atan`, the first of the inverse trigonometric functions. $\pm0.0$ gives $\pm0.0$, the only
  exact case, and $\pm\infty$ gives $\pm\pi/2$; the arctangent is odd and bounded, so it never
  overflows. `atan_with_period_prec_round`, `atan_with_period_prec`, `atan_with_period_round`, and
  `atan_with_period` (with `_ref` and `_assign` variants) are a port of `mpfr_atanu`, measuring it
  in $u$ths of a turn as $\arctan(x)u/(2\pi)$ — the inverse of the convention `sin_with_period`
  uses for its input — where an infinite input gives a quarter turn, $\pm1$ an eighth, and a zero
  input or a zero period a zero with the sign of $x$; those are the only exact cases, and the
  result underflows for a tiny $x$ with a small $u$. `atan_pi_prec_round`, `atan_pi_prec`,
  `atan_pi_round`, and `atan_pi` (with the usual variants) are that with $u = 2$: IEEE 754's
  `atanPi`, whose exact cases hold at every precision. Each of the three has `_rational` variants
  taking a `Rational`, which MPFR has no equivalent of, and `primitive_float_*` variants giving
  correctly rounded `f32` and `f64` results.
- `Atan2` and `Atan2Assign` (new traits in malachite-base) for `Float`, with the usual
  `atan2_prec_round`, `atan2_prec`, `atan2_round`, and `_val_ref`/`_ref_val`/`_ref_ref`/`_assign`
  variants: a port of `mpfr_atan2`, the angle of the point $(x,y)$ measured from the positive
  $x$-axis. The twenty ISO C99 special cases are honored, with the sign of a zero argument choosing
  the quadrant, and the zero results are the only exact ones. `atan2_with_period_prec_round`,
  `atan2_with_period_prec`, and `atan2_with_period_round` are a port of `mpfr_atan2u`, measuring
  the angle in $u$ths of a turn, where the axes and the quadrant diagonals are exact;
  `atan2_pi_prec_round`, `atan2_pi_prec`, and `atan2_pi_round` are that with $u = 2$, IEEE 754's
  `atan2Pi` and a port of `mpfr_atan2pi`. The result underflows for a positive $x$ with a tiny
  $|y/x|$. Two deliberate divergences from MPFR: when $u$ is zero this returns a zero with the sign
  of $y$ throughout, where `mpfr_atan2u` returns $\pm1$ for a negative $x$, contradicting its own
  definition and its own answers when $y$ is zero or infinite; and a quotient beyond the exponent
  range is answered from the turn fraction it approaches, where MPFR, whose widened range keeps the
  quotient representable, can instead spend an unbounded amount of time separating that fraction
  from the one beside it. With `_rational` and `primitive_float_*` variants throughout.
- `Asin` and `AsinAssign` (new traits in malachite-base) for `Float`, with the usual
  `asin_prec_round`, `asin_prec`, `asin_round`, and `_ref`/`_assign` variants: a port of
  `mpfr_asin`. NaN, either infinity, and any $|x|>1$ give NaN; $\pm0.0$ is exact and $\pm1$ gives
  $\pm\pi/2$. The arcsine can neither overflow nor underflow. `asin_with_period_prec_round`,
  `asin_with_period_prec`, `asin_with_period_round`, and `asin_with_period` (with the usual
  variants) are a port of `mpfr_asinu`, measuring it in $u$ths of a turn, so that $u = 360$ gives
  degrees, where $\pm1$ gives $\pm u/4$ and $\pm1/2$ gives $\pm u/12$ when $u$ is a multiple of 3;
  `asin_pi_prec_round` and friends are a port of `mpfr_asinpi`, that with $u = 2$. One deliberate
  divergence from MPFR: at $u = 0$ `mpfr_asinu` returns $+0$ for every $x$, although its own
  $x = 0$ case keeps the sign so that the function stays odd; Malachite keeps it throughout, as
  `mpfr_atanu` does. The `_rational` variants, which MPFR has no equivalent of, can underflow where
  the `Float` ones cannot, a `Rational` reaching below the exponent range. With `primitive_float_*`
  variants throughout.
- `Acos` and `AcosAssign` (new traits in malachite-base) for `Float`, with the usual
  `acos_prec_round`, `acos_prec`, `acos_round`, and `_ref`/`_assign` variants: a port of
  `mpfr_acos`. NaN, either infinity, and any $|x|>1$ give NaN; $\pm0.0$ gives $\pi/2$, $1$ gives
  $0.0$, and $-1$ gives $\pi$. The zero at $x = 1$ is the only exact case — unlike the arcsine, a
  zero input is not one, $\pi/2$ never being representable — and overflow is not possible, the
  result lying in $[0,\pi]$. `acos_with_period_prec_round`, `acos_with_period_prec`,
  `acos_with_period_round`, and `acos_with_period` (with the usual variants) are a port of
  `mpfr_acosu`, measuring it in $u$ths of a turn, where a zero input gives $u/4$, $1$ gives $0.0$
  following IEEE 754-2019's `acosPi`, $-1$ gives $u/2$, and $\pm1/2$ gives $u/6$ or $u/3$ when $u$
  is a multiple of 3; `acos_pi_prec_round` and friends are a port of `mpfr_acospi`, that with
  $u = 2$. The `_rational` variants can underflow where the `Float` ones cannot, a `Rational` being
  able to lie within $2^{-2^{31}}$ of 1. With `primitive_float_*` variants throughout.
- `Asec` and `AsecAssign` (new traits in malachite-base) for `Float`, with the usual
  `asec_prec_round`, `asec_prec`, `asec_round`, and `_ref`/`_assign` variants. MPFR has no
  arcsecant, nor any of the forms below. NaN and every $|x|<1$, including the zeros, give NaN;
  $\pm\infty$ gives $\pi/2$, the value the secant grows toward; $1$ gives $0.0$, the only exact
  case; and $-1$ gives $\pi$. `asec_with_period_prec_round`, `asec_with_period_prec`,
  `asec_with_period_round`, and `asec_with_period` (with the usual variants) measure it in $u$ths
  of a turn, with the arccosine's exact cases seen through the reciprocal: $\pm\infty$ gives $u/4$,
  $1$ gives $0.0$, $-1$ gives $u/2$, and $\pm2$ give $u/6$ and $u/3$ when $u$ is a multiple of 3.
  `asec_pi_prec_round` and friends are that with $u = 2$, where $\pm2$ are no longer exact, a third
  of a half-turn not being representable. The `_rational` variants can underflow, a `Rational`
  being able to lie within $2^{-2^{31}}$ of 1. With `primitive_float_*` variants throughout.
- `Acsc` and `AcscAssign` (new traits in malachite-base) for `Float`, with the usual
  `acsc_prec_round`, `acsc_prec`, `acsc_round`, and `_ref`/`_assign` variants. MPFR has no
  arccosecant, nor any of the forms below. The arccosecant is odd. NaN and every $|x|<1$, including
  the zeros, give NaN; $\pm\infty$ give $\pm0.0$, the only exact cases; and $\pm1$ give $\pm\pi/2$.
  Neither overflow nor underflow is possible, a `Float`'s bounded exponent keeping $1/|x|$ above
  twice the smallest positive one. `acsc_with_period_prec_round`, `acsc_with_period_prec`,
  `acsc_with_period_round`, and `acsc_with_period` (with the usual variants) measure it in $u$ths
  of a turn, with the arcsine's exact cases seen through the reciprocal: a zero period gives a zero
  with the sign of $x$, $\pm1$ give $\pm u/4$, and $\pm2$ give $\pm u/12$ when $u$ is a multiple of
  3. `acsc_pi_prec_round` and friends are that with $u = 2$, where $\pm2$ are no longer exact, a
  sixth of a half-turn not being representable. Unlike the arccosecant alone, the periodic forms
  underflow, a small $u$ carrying the quotient below the smallest positive `Float`. With
  `_rational` and `primitive_float_*` variants throughout.
- `Acot` and `AcotAssign` (new traits in malachite-base) for `Float`, with the usual
  `acot_prec_round`, `acot_prec`, `acot_round`, and `_ref`/`_assign` variants. MPFR has no
  arccotangent, nor any of the forms below. This is the odd branch,
  $\operatorname{acot} x = \arctan(1/x)$, with range $(-\pi/2,\pi/2]$: the one that makes the
  arcsecant, arccosecant and arccotangent a uniform family of inverses of reciprocal arguments,
  that inverts `cot` on its own signed behaviour ($\cot(\pm0)=\pm\infty$ and so
  $\operatorname{acot}(\pm\infty)=\pm0$), and that Mathematica uses; the continuous branch
  $\pi/2-\arctan x$ with range $(0,\pi)$ is not provided. NaN gives NaN; $\pm\infty$ give $\pm0.0$,
  the only exact cases; $\pm0.0$ give $\pm\pi/2$, the sign choosing the side of the jump; and
  $\pm1$ give $\pm\pi/4$. Neither overflow nor underflow is possible.
  `acot_with_period_prec_round`, `acot_with_period_prec`, `acot_with_period_round`, and
  `acot_with_period` (with the usual variants) measure it in $u$ths of a turn, where a zero period
  gives a zero with the sign of $x$, $\pm0.0$ give $\pm u/4$ — the jump's two sides, which a period
  makes exact — and $\pm1$ give $\pm u/8$. `acot_pi_prec_round` and friends are that with $u = 2$,
  which unlike the arcsecant's and arccosecant's half-turns loses no exact case, a half and a
  quarter each needing one bit. The periodic forms underflow for a large enough $|x|$. A `Rational`
  zero has no sign, so the `_rational` variants give it the positive side. With `primitive_float_*`
  variants throughout.
- `Cot` and `CotAssign` (new traits in malachite-base) for `Float`, with the usual
  `cot_prec_round`, `cot_prec`, `cot_round`, and `_ref`/`_assign` variants: a port of `mpfr_cot`,
  MPFR's generic reciprocal template with the tangent. MPFR's tangent is itself a quotient of a
  sine and a cosine, so the cotangent is taken as $\cos x/\sin x$ directly, which saves the middle
  rounding and treats the two ends of the exponent range alike. Unlike the secant and the cosecant,
  the cotangent is not bounded away from zero, so it both overflows, within $2^{-2^{30}}$ of a
  multiple of $\pi$, and underflows, within $2^{-2^{30}}$ of an odd multiple of $\pi/2$; each end
  is decided from an exact bracket. MPFR's shortcut for a tiny input is kept and is load-bearing:
  there $\cot x$ is $1/x - x/3 + \ldots$, so rounding $1/x$ settles the result, except when $x$ is
  a power of 2 and $1/x$ is exact, where the true value lies one step short of it, toward zero.
  Without it the Ziv loop would face an exactly representable quotient that no working precision
  could certify. `cot(\pm0.0)` is $\pm\infty$, and the function is odd.
  `primitive_float_cot` gives the correctly rounded `f32` or `f64` cotangent, which neither the
  standard library nor `libm` provides; it overflows for a small enough input.
  `cot_rational_prec_round` and `cot_rational_prec` (with `_ref` variants) take a `Rational`
  instead, with a direct bracket for a tiny input, inverting the tangent's own series bracket; that
  also covers inputs below the `Float` exponent range, which no other path could round, and the
  powers of 2, whose reciprocals are exactly representable and which the Ziv loop could therefore
  never certify. `primitive_float_cot_rational` gives the correctly rounded `f32` or `f64`
  cotangent of a `Rational`.
- `cot_with_period_prec_round`, `cot_with_period_prec`, `cot_with_period_round`, and
  `cot_with_period` (with `_ref` and `_assign` variants), the cotangent of a `Float` measured in
  $u$ths of a turn. MPFR has no `cotu`; this is `cot` with the sine and cosine taken in turns, which
  reduces the argument exactly and so reaches the exact and closed-form cases the radian version
  cannot see. These are the tangent's, reciprocated: odd multiples of $1/8$ of a turn give exactly
  $\pm1$, odd multiples of $1/4$ give exactly $\pm0.0$, and thirds and sixths give
  $\pm\sqrt3/3$ while twelfths give $\pm\sqrt3$. Multiples of a half turn are the poles, where
  the sine is a zero carrying the sign of $x$ and the cosine is $\pm1$, so the infinity takes the
  sign of $x$ at an even multiple and the opposite at an odd one; keeping that identity is what
  makes the function odd. `primitive_float_cot_with_period` gives the correctly rounded `f32` or
  `f64` cotangent in $u$ths of a turn.
  `cot_with_period_rational_prec_round` and `cot_with_period_rational_prec` (with `_ref` variants)
  take a `Rational` instead, reaching the exact and closed-form cases directly and needing no
  argument reduction beyond the exact one. A `Rational` fraction of a turn can be small enough, or
  close enough to a multiple of a half turn, to overflow, and as close to an odd quarter turn to
  underflow; each end is decided from an exact bracket.
  `primitive_float_cot_with_period_rational` gives the correctly rounded `f32` or `f64` cotangent
  of a `Rational` fraction of a turn.
- `cot_pi_prec_round`, `cot_pi_prec`, `cot_pi_round`, and `cot_pi` (with `_ref` and `_assign`
  variants), the cotangent of a `Float` measured in half-turns, delegating to `cot_with_period`
  with a period of 2; MPFR has no `cotpi` to match its `sinpi` and `cospi`. Integers are poles and
  give $\pm\infty$, with the sign of $x$ at an even integer and the opposite at an odd one;
  half-integers give $\pm0.0$, odd multiples of $1/4$ give $\pm1$, odd multiples of $1/6$ give
  $\pm\sqrt3$, and multiples of $1/3$ that are not integers give $\pm\sqrt3/3$.
  `cot_pi_rational_prec_round` and `cot_pi_rational_prec` (with `_ref` variants) take a `Rational`
  instead, and `primitive_float_cot_pi` and `primitive_float_cot_pi_rational` give the correctly
  rounded `f32` or `f64` cotangent.
- `Csc` and `CscAssign` (new traits in malachite-base) for `Float`, with the usual
  `csc_prec_round`, `csc_prec`, `csc_round`, and `_ref`/`_assign` variants: a port of `mpfr_csc`,
  MPFR's generic reciprocal template with the sine. The cosecant never underflows, since its
  magnitude is at least 1, but unlike MPFR's it can overflow: within $2^{-2^{30}}$ of a multiple of
  $\pi$, and for any input whose reciprocal alone leaves the range. MPFR's shortcut for a tiny
  input is kept, where $\csc x$ is $1/x + x/6 + \ldots$ and rounding $1/x$ settles the result
  except when $x$ is a power of 2; without it the Ziv loop could never certify an exactly
  representable reciprocal. `csc(\pm0.0)` is $\pm\infty$, and the function is odd.
  `primitive_float_csc` gives the correctly rounded `f32` or `f64` cosecant, which overflows for a
  small enough input.
  `csc_rational_prec_round` and `csc_rational_prec` (with `_ref` variants) take a `Rational`
  instead, with a direct bracket for a tiny input, inverting a bracket on the sine; that also
  covers inputs below the `Float` exponent range, which no other path could round.
  `primitive_float_csc_rational` gives the correctly rounded `f32` or `f64` cosecant of a
  `Rational`.
- `csc_with_period_prec_round`, `csc_with_period_prec`, `csc_with_period_round`, and
  `csc_with_period` (with `_ref` and `_assign` variants), the cosecant of a `Float` measured in
  $u$ths of a turn. MPFR has no `cscu`; this is `csc` with the sine taken in turns, which reduces
  the argument exactly and so reaches the exact and closed-form cases the radian version cannot
  see: odd quarter turns give $\pm1$, odd twelfths give $\pm2$, eighths give $\pm\sqrt2$, thirds
  and sixths give $\pm2\sqrt3/3$, and twentieths give $\pm2\varphi$ or $\pm2(\varphi-1)$, where
  $\varphi$ is the golden ratio. Multiples of a half turn are poles, where the sine is a zero
  carrying the sign of the input and the cosecant is its reciprocal, an infinity with that sign;
  keeping that identity is what makes the function odd everywhere, at the cost of period $u$ at a
  pole alone. Unlike the secant's, this cosecant can overflow away from a pole, since a tiny angle
  has a huge cosecant; such a result is decided from an exact bracket on the sine.
  `primitive_float_csc_with_period` gives the correctly rounded `f32` or `f64` cosecant in $u$ths
  of a turn, which does overflow for a small enough angle.
  `csc_with_period_rational_prec_round` and `csc_with_period_rational_prec` (with `_ref` variants)
  take a `Rational` instead, reaching the exact and closed-form cases directly and needing no
  argument reduction beyond the exact one. The radian version's shortcut for a tiny input has no
  counterpart here: in turns the angle is never a `Float`, so by Niven's theorem the sine past the
  closed-form cases is irrational and its reciprocal is never exactly representable, which is what
  stalls the radian loop. A `Rational` fraction of a turn can be small enough, or close enough to a
  multiple of a half turn, that the sine falls below the `Float` exponent range; the bracket reads
  that as the overflow it is. `primitive_float_csc_with_period_rational` gives the correctly
  rounded `f32` or `f64` cosecant of a `Rational` fraction of a turn.
- `csc_pi_prec_round`, `csc_pi_prec`, `csc_pi_round`, and `csc_pi` (with `_ref` and `_assign`
  variants), the cosecant of a `Float` measured in half-turns, delegating to `csc_with_period` with
  a period of 2; MPFR has no `cscpi` to match its `sinpi` and `cospi`. Integers are poles and give
  $\pm\infty$ with the sign of $x$, half-integers give $\pm1$, odd multiples of $1/6$ give
  $\pm2$, odd multiples of $1/4$ give $\pm\sqrt2$, and multiples of $1/3$ that are not integers
  give $\pm2\sqrt3/3$. `csc_pi_rational_prec_round` and `csc_pi_rational_prec` (with `_ref`
  variants) take a `Rational` instead, and `primitive_float_csc_pi` and
  `primitive_float_csc_pi_rational` give the correctly rounded `f32` or `f64` cosecant.
- `Sec` and `SecAssign` (new traits in malachite-base) for `Float`, with the usual
  `sec_prec_round`, `sec_prec`, `sec_round`, and `_ref`/`_assign` variants: a port of `mpfr_sec`,
  which instantiates MPFR's generic reciprocal template with the cosine. The secant never
  underflows, since its magnitude is at least 1, but unlike MPFR's it can overflow, for an input
  within $2^{-2^{30}}$ of an odd multiple of $\pi/2$; such a result is decided from an exact
  bracket on the cosine. `primitive_float_sec` gives the correctly rounded `f32` or `f64` secant,
  which neither the standard library nor `libm` provides.
  `sec_rational_prec_round` and `sec_rational_prec` (with `_ref` variants) take a `Rational`
  instead, with a direct series bracket for a tiny input: there the cosine rounds toward zero to
  the `Float` just below 1, whose reciprocal ties back to 1 at every working precision, so the Ziv
  loop would not terminate without it. `primitive_float_sec_rational` gives the correctly rounded
  `f32` or `f64` secant of a `Rational`.
- `sec_with_period_prec_round`, `sec_with_period_prec`, `sec_with_period_round`, and
  `sec_with_period` (with `_ref` and `_assign` variants), the secant of a `Float` measured in $u$ths
  of a turn. MPFR has no `secu`; this is `sec` with the cosine taken in turns, which reduces the
  argument exactly and so reaches the exact and closed-form cases the radian version cannot see:
  even multiples of a half turn give $1$ and odd ones $-1$, thirds and sixths give $\pm2$, eighths
  give $\pm\sqrt2$, twelfths give $\pm2\sqrt3/3$, and fifths and tenths give $\pm2\varphi$ or
  $\pm2(\varphi-1)$, where $\varphi$ is the golden ratio. Odd multiples of a quarter turn are poles,
  where the cosine is $+0.0$ and the secant is its reciprocal, $\infty$; keeping that identity is
  what makes the function even everywhere. `primitive_float_sec_with_period` gives the correctly
  rounded `f32` or `f64` secant in $u$ths of a turn; like the tangent's, it can only reach an
  infinity at an exact pole, never through overflow.
  `sec_with_period_rational_prec_round` and `sec_with_period_rational_prec` (with `_ref` variants)
  take a `Rational` instead, reaching the exact and closed-form cases directly and needing no
  argument reduction beyond the exact one. `primitive_float_sec_with_period_rational` gives the
  correctly rounded `f32` or `f64` secant of a `Rational` fraction of a turn.
- `sec_pi_prec_round`, `sec_pi_prec`, `sec_pi_round`, and `sec_pi` (with `_ref` and `_assign`
  variants), the secant of a `Float` measured in half-turns, delegating to `sec_with_period` with a
  period of 2; MPFR has no `secpi` to match its `sinpi` and `cospi`. Even integers give $1$ and odd
  ones $-1$, half-integers are poles and give $\infty$, odd multiples of $1/4$ give $\pm\sqrt2$,
  and multiples of $1/3$ give $\pm2$. `sec_pi_rational_prec_round` and `sec_pi_rational_prec` (with
  `_ref` variants) take a `Rational` instead, and `primitive_float_sec_pi` and
  `primitive_float_sec_pi_rational` give the correctly rounded `f32` or `f64` secant.
- `tan_pi_prec_round`, `tan_pi_prec`, `tan_pi_round`, and `tan_pi` (with `_ref` and `_assign`
  variants), a port of `mpfr_tanpi`: the tangent of a `Float` measured in half-turns, delegating to
  `tan_with_period` with a period of 2. Integers give a signed zero, half-integers are poles and
  give an infinity, odd multiples of a quarter give $\pm1$, and thirds and sixths give $\pm\sqrt3$
  or $\pm\sqrt3/3$. `tan_pi_rational_prec_round` and `tan_pi_rational_prec` (with `_ref` variants)
  take a `Rational` instead, and `primitive_float_tan_pi` and `primitive_float_tan_pi_rational`
  give the correctly rounded `f32` or `f64` tangent.
- `sin_with_period`, `cos_with_period`, `tan_with_period`, `sin_cos_with_period`, `sin_pi`,
  `cos_pi`, and `sin_cos_pi` on `Float` (each with `_ref` and `_assign` variants), rounding to the
  precision of the input and to the nearest `Float`. This is the tier that `sin`, `cos`, `tan`, and
  `sin_cos` already had through their traits; a function that takes a period cannot go through one,
  since `Sin` and its siblings take no extra argument, so these are inherent methods.
- The Dottie number, the fixed point of the cosine, as `dottie_number_prec_round` and
  `dottie_number_prec` on `Float`, correctly rounded to any precision (Newton's method with a
  certified final bracket), and as a `DottieNumber` trait with constants for primitive floats.
- `sin`, `cos`, and `sin_cos` now use MPFR's asymptotically fast tier (`mpfr_sincos_fast`, binary
  splitting of the Taylor series over chunks of the reduced argument, combined by the angle-addition
  formulas) at and above a tuned precision threshold (25285 bits), as MPFR does at its
  `MPFR_SINCOS_THRESHOLD`, bringing their cost from $O(n^{3/2})$ to $O(n \log^3 n)$ word
  operations up to log factors. The tuner (`-g tune_sincos` in the `malachite-float` binary)
  shares its crossover machinery with `malachite-nz`'s, now in
  `malachite_base::test_util::bench::tune`.
- Fixed `cos_with_period_rational_prec_round` taking a working precision of billions of bits for a
  tiny negative input, and `cos_with_period_prec_round` doing the same for a `Float` just below a
  multiple of its period: the fraction of a turn is now reduced to $[-1/2, 1/2]$, where the
  small-input shortcut applies.
- `primitive_float_sin` and `primitive_float_sin_rational`, the correctly rounded sine of an `f32`
  or `f64`, or of a `Rational` as an `f32` or `f64`.
- `primitive_float_cos` and `primitive_float_cos_rational`, the correctly rounded cosine of an
  `f32` or `f64`, or of a `Rational` as an `f32` or `f64`, alongside the existing
  `primitive_float_exp` and `primitive_float_exp_rational`.
- Fixed `Float` remainders (`rem` and `ieee_remainder` families) by a divisor of more than
  $2^{30}$ bits with an odd mantissa, which overflowed to infinity: the integer remainder was
  rounded to a `Float` before the final shift brought it back into range. `cos` of an argument
  with a near-maximal exponent reduces modulo such a $2\pi$ and was affected.
- `cos_with_period_prec_round`, `cos_with_period_prec`, and `cos_with_period_round` (with `_ref` and `_assign` variants),
  a port of `mpfr_cosu`: the cosine of a `Float` measured in $u$ths of a turn, so that `u = 360`
  is degrees. Multiples of a quarter or a sixth of a turn are exact (an odd quarter turn is
  $+0.0$, as IEEE 754-2019's `cosPi` specifies), and eighths, twelfths, fifths, and tenths of a
  turn are computed from a single correctly rounded constant ($\sqrt2$, $\sqrt3$, or $\varphi$)
  rather than from $\pi$ and a cosine. Inputs within $2^{-2^{30}}$ of an odd quarter turn
  underflow correctly. `cos_with_period_rational_prec_round` and `cos_with_period_rational_prec`
  (with `_ref` variants) take a `Rational` instead, reaching the exact and closed-form cases
  directly, such as a third or an eighth of a turn. `primitive_float_cos_with_period` and
  `primitive_float_cos_with_period_rational` give the correctly rounded `f32` or `f64` results.
- `cos_pi_prec_round`, `cos_pi_prec`, and `cos_pi_round` (with `_ref` and `_assign` variants),
  `cos_pi_rational_prec_round` and `cos_pi_rational_prec` (with `_ref` variants), and
  `primitive_float_cos_pi` and `primitive_float_cos_pi_rational`: a port of `mpfr_cospi`, the
  cosine in half-turns, delegating to the `cos_with_period` family with a period of 2.
- `cos_rational_prec_round` and `cos_rational_prec` (with `_ref` variants), the correctly
  rounded cosine of a `Rational` as a `Float`, alongside the `exp_rational_*` family. Since
  cosine is not monotonic, the result is bracketed by a Lipschitz bound around the cosine of a
  `Float` approximation rather than by bracketing the input, with exact `Rational` handling near
  odd multiples of $\pi/2$ and for inputs too large to be `Float`s.
- Conversions between `Float` and the Gaussian types: `TryFrom` and `ConvertibleFrom`
  implementations converting `GaussianInteger` and `GaussianRational` to `Float` (real and, for
  the rational case, dyadic; minimal precision) and `Float` to either Gaussian type (finite,
  and integral for `GaussianInteger`).

### Documentation

- The `FromStr` docs for `Natural`, `Integer`, and `Rational` now mention the accepted leading
  `'+'` (and, for `Rational`, the `'+'` allowed on the denominator), which the parsers had
  always accepted.

## 0.11.0 — 2026-08-27

The main themes of this release are a large batch of number-theoretic functions (CRT, modular
division and square roots, rational reconstruction, and a family of combinatorial sequences),
broad new MPFR coverage for `Float` (correctly rounded sums, products, and fused operations;
remainders and rounding functions; bit-exact random samplers; and the constants that complete
the MPFR constants section), ten transition-mapping pages on the website documenting how
Malachite corresponds to GMP, MPFR, FLINT, and num, and a substantial upgrade of the num-bigint
compatibility crate.

### Breaking and behavioral changes

- `Float::increment` and `Float::decrement` are now precision-preserving neighbor steps,
  matching IEEE `nextUp`/`nextDown`, MPFR's `mpfr_nextabove`/`nextbelow`, and Rust's
  `f64::next_up`/`next_down`. Previously they were full-ulp steps that could change a value's
  precision at binade boundaries and collapsed precision-1 powers of 2 to zero. If the old
  behavior is needed, write `x ± x.ulp()`.
- Dividing zero by zero now panics, as the documentation always claimed. Previously an
  equal-operands fast path made `Natural`/`Integer` `div_mod`, `div_rem`, and `div_exact` return
  a quotient of 1 when both operands were zero.
- Formatting a negative `Integer` (or a signed primitive through `BaseFmtWrapper`) with the `+`
  flag, a fill/alignment specifier, or a plain width now follows the standard library's rules.
  Previously `{:+}` printed a stray plus after the minus sign and any width forced zero-padding,
  ignoring fill and alignment. Zero-padded forms like `{:08}` are unchanged.
- In malachite-bigint, `Roots for BigInt` now truncates toward zero on negative inputs instead
  of flooring, and `modinv` returns `None` instead of panicking when the value is a multiple of
  the modulus — both matching num-bigint.

### malachite-base

- New arithmetic traits with primitive implementations, also implemented by the bignum types
  where noted below: `Average` (floor and ceiling midpoints, implemented everywhere),
  `Compound`/`CompoundAssign`, `RisingFactorial`, `MulAddMul`/`MulSubMul` (fused
  `x * y ± z * w`), and the comparison family `PartialOrdDouble`/`PartialOrdAbsDouble`/
  `OrdDouble` (compare a number against twice another without computing the double — the shape
  of a round-to-nearest decision).
- New named constants for primitive floats, with corresponding traits: Catalan's constant and
  Euler's constant.
- GMP-style formatting: `gmp_format!` and friends, with `%Z`, `%Q`, and `%R` conversions
  rendering `Integer`, `Rational`, and `Float` values, plus GMP-compatible string conversions
  to back them.
- Balanced-tree folding for iterator `Sum` and `Product` (`balanced_fold`), improving both
  accuracy and speed of long reductions.

### malachite-nz

- Number theory: the Chinese remainder theorem (`multi_crt` and balanced variants), modular
  division (`ModDiv`, `mod_div_list`), modular square roots (`ModSqrt`), Bell numbers (single
  and vector forms), Landau's function, rising factorials, and completed Fibonacci and Lucas
  sequences with improved subfactorials. A Kronecker symbol edge case was also fixed.
- Fused operations: `mul_shr_round` (a fused `(x * y) >> k` with rounding, via a Mulders short
  product) and `MulAddMul`/`MulSubMul` for `Natural` and `Integer`, and `AddMul`/`SubMul` are
  now faster than their unfused equivalents.
- Performance: division and modular arithmetic with precomputed inverses (modular
  multiplication improved by roughly 20% in the precomputed paths) and fused shift-add limb
  kernels.
- Fixed: the 0/0 division contract and negative-number formatting flags listed above, and an
  unsoundness in the `mpfr_can_round_raw` port with 32-bit limbs (a latent bug in MPFR itself
  on 32-bit-limb builds): a carry absorbed by a truncated limb was misread as a binade change,
  letting `Float::can_round` claim an undecidable rounding was decided.

### malachite-q

- `Rational` GCD and extended GCD (in the lattice sense), rational reconstruction (recovering
  p/q from its residue mod m), Dedekind sums, harmonic numbers, and height functions
  (`to_height`, `into_height`, `height_significant_bits`).
- `simplest_rational_in_interval` now uses FLINT's algorithm, and the related
  denominators-in-interval functions were redesigned around a mediant heap, making some of them
  hundreds of times faster.
- `AddMul` and `SubMul` implementations, sequence utilities, and GMP-style string conversions.

### malachite-float

- Correctly rounded aggregates: `Sum`, `Product`, and dot products (ports of `mpfr_sum` and
  `mpfr_dot`, without the latter's abort on extreme exponents), `add_mul`/`sub_mul` (fused
  multiply-add rounded once), `mul_add_mul`/`mul_sub_mul` (`mpfr_fmma`/`fmms`), and
  `Float`-valued factorials.
- More MPFR coverage: `hypot`, `compound` (with an upstream MPFR rounding bug found and
  corrected in the port), `positive_difference` (`mpfr_dim`), `min`/`max`, remainders (`rem`,
  IEEE remainder, and quotient-bit variants), the round-to-integer family including
  `fractional_part` and integer/fraction decomposition, `can_round`, and `subnormalize`
  (enabling faithful emulation of IEEE formats such as quad precision).
- Mixed `Float`/`Rational` variants throughout (fused operations, remainders, `min`/`max`,
  `positive_difference`), treating the `Rational` operand exactly.
- New constants, correctly rounded to any precision: Euler's constant γ (Brent-McMillan),
  Catalan's constant (Adamchik's formula), and the digit-defined Liouville, Champernowne, and
  Copeland-Erdős constants. This completes the MPFR constants section of the mapping.
- Random generation matching MPFR bit for bit: uniform floats in the unit interval and the
  other MPFR samplers, plus a new suite of random and exhaustive `Float` generators for
  testing.
- `ToStringBase` and additional string-conversion functions.
- More correctly rounded `f32`/`f64` functions in the `primitive_float_*` family, computed
  exactly via `Float` and rounded once, including sums, products, and dot products of slices.
- The `increment`/`decrement` semantics change listed above.

### malachite-bigint

- The behavioral fixes for num-bigint parity listed above (`Roots`, `modinv`).
- New optional features matching num-bigint 0.4.8 exactly: `serde` (identical wire format,
  cross-deserializable with num-bigint), `rand` (`RandBigInt`, `RandomBits`,
  `UniformBigInt`/`UniformBigUint`, producing bit-identical value streams from identically
  seeded RNGs), `arbitrary`, and `quickcheck`.
- Completed API surface: `DoubleEndedIterator` for `U32Digits`, `Mul` for `Sign`, and overrides
  of `num_integer::Integer`'s default methods (`div_mod_floor` in one division, `div_ceil`,
  `gcd_lcm`, `extended_gcd_lcm`, `next`/`prev_multiple_of`, `dec`/`inc`), with
  `Euclid`/`CheckedEuclid` forwarded to Malachite's Euclidean-division operations.
- Removed a hex-parsing workaround for a malachite bug that no longer exists.

### Documentation and website

- Ten transition-mapping pages documenting the correspondence between Malachite and other
  libraries, function by function: GMP integers and rationals; FLINT integers, integers mod n,
  rationals, and arithmetic functions; MPFR floats; and num integers, rationals, and traits.
- A documentation audit across the workspace: complexity annotations verified and standardized
  (conventions recorded in `DOC-CONVENTIONS.md`), plus refreshed front-page examples.

## 0.10.0 — 2026-07-26

Reconstructed retroactively. The two big themes were `Float` elementary functions — the
exponential and power families, correctly rounded at any precision — and a complete rewrite of
`Float`-string interconversion.

### Breaking and behavioral changes

- `Float`'s `Display`, `Debug`, and the other string conversions were rewritten on a port of
  MPFR's `get_str`. Output became correctly rounded scientific decimal at every exponent (the
  old implementation bailed out above `|exponent| > 10000`), with MPFR's precision-dependent
  digit counts, so many outputs differ textually from 0.9.2.
- `RationalSequence` in malachite-base was renamed to `FoerSequence` (a sequence that is Finite
  Or Eventually Repeating), freeing the old name from the misreading that it had something to
  do with `Rational`.
- The internal module layouts of malachite-float and malachite-q were reorganized; code that
  named deep module paths directly may have needed import adjustments.

### Highlights

- The `Float` exponential family: `exp`, `exp_x_minus_1`, `power_of_2`, `power_of_10`, and
  their `_x_minus_1` companions, with `Rational`-argument versions and `Float`-valued outputs
  for primitive inputs.
- The `Float` power family: `Float` raised to `Float`, signed and unsigned integer powers, IEEE
  `powr`, and roots — `sqrt`, `cbrt`, and nth roots (`root_u`, `root_s`) — including
  `Float`-valued square roots, cube roots, and logarithms of unsigned integers.
- String-to-`Float` parsing (a port of `set_str`, bases 2 through 62), `to_sci_string`, and
  serde support for `Float`, alongside the `get_str` rewrite above.
- New constants: e, the cube root of 2, Gelfond's constant, the Gelfond-Schneider constant, and
  Ramanujan's constant, with the corresponding primitive-float constant traits in
  malachite-base and the correctly-rounded-`f32`/`f64` emulation machinery
  (`primitive_float_*`) in malachite-float.
- Performance: string conversion with precomputed inverses and a round of division and
  conversion threshold tuning in malachite-nz.
