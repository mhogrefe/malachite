# malachite-lints

Custom [Dylint](https://github.com/trailofbits/dylint) lints enforcing Malachite house
conventions. These replace the old `additional-lints.py`. All lints are deny-by-default, so any
hit fails the run.

This crate is not a member of the main workspace: it builds against a pinned nightly toolchain
(see `rust-toolchain`) because lints link against rustc internals, exactly like Clippy. The main
Malachite crates remain stable-Rust; nightly is needed only to build and run these lints.

## Running

One-time setup:

```sh
cargo install cargo-dylint dylint-link
```

Then, from any crate in the workspace (the library is registered in the root `Cargo.toml` under
`[workspace.metadata.dylint]`):

```sh
cargo dylint --all -- --all-targets --features bin_build
```

or run every crate at once with `bash additional-lints.sh` from the repo root, which is what
`build.sh` does.

Lint configuration lives in `dylint.toml` at the repo root.

## Using these lints in your own project

Most of these lints check usage of Malachite's public API, so they are also useful in downstream
code. The crate is not published to crates.io (a compiler-plugin dylib pinned to a specific
nightly has no use for dependency resolution); instead, point Dylint at this repository from your
own workspace's `Cargo.toml`:

```toml
[workspace.metadata.dylint]
libraries = [{ git = "https://github.com/mhogrefe/malachite", pattern = "malachite-lints" }]
```

Then, after the one-time `cargo install cargo-dylint dylint-link`, run `cargo dylint --all` in
your project. Notes:

- The lints are deny-by-default; suppress individual ones with
  `#[cfg_attr(dylint_lib = "malachite_lints", allow(lint_name))]` (see your crate's
  `[lints.rust] unexpected_cfgs` table for accepting the `dylint_lib` cfg).
- `long_lines` enforces Malachite's 100-column house style and is configured through a
  `dylint.toml` at your workspace root; if you don't want it, allow it at crate level with
  `#![cfg_attr(dylint_lib = "malachite_lints", allow(long_lines))]`.
- Code under `tests/`, `bin_util/`, or `test_util/` directories, and test-harness builds, are
  exempt from the style lints.

## Lints

### `redundant_from_in_comparison`

Flags comparisons between a bignum type (`Natural`, `Integer`, `Rational`, or `Float`) and a value
converted from a primitive with `from`, such as `x >= Integer::from(prec)`. The bignum types
implement `PartialEq` and `PartialOrd` directly against the primitives, so the conversion (which
may allocate) is unnecessary: `x >= prec` means the same thing. The lint only fires when the
direct impl for the operands in their present order actually exists.

### `redundant_nearest`

Flags calls like `x.foo_prec_round(.., Nearest)` (in both method-call and associated-function
form) when a `foo_prec` shorthand exists on the same type. Exempt: the shorthand's own defining
delegation, everything inside trait impls (operators, `*Assign`, `LogBase`, ..., which delegate
via the explicit form by convention), and code under `tests/`, `bin_util/`, or `test_util/`,
which exercises both spellings on purpose.

### `compare_with_power_of_2`

Flags comparing a bignum with `power_of_2(..)` — via the comparison operators, `cmp`,
`partial_cmp`, or the `*_abs` comparison methods. Materializing a power of 2 just to compare
against it is wasteful (for large powers it allocates a huge number); comparing the value's
exponent with the power is direct and cheap. The advice is type-specific: `Natural` has
`floor_log_base_2`, `ceiling_log_base_2`, and `checked_log_base_2`; an `Integer` can use them
through `unsigned_abs_ref()`; `Rational` additionally has the `_abs` variants; and for a `Float`,
`get_exponent()` gives 1 more than the floor of the log. Tests, demos, and test utilities are
exempt: they compare against `power_of_2` on purpose, to cross-check the log functions
themselves.

### `mul_div_by_power_of_2`

Flags multiplying or dividing a bignum by `power_of_2(..)`, including the `*=` and `/=` forms:
shifting is more direct and cheaper, so `x << k` rather than `x * T::power_of_2(k)` and `x >> k`
rather than `x / T::power_of_2(k)`. Signed shifts accept negative counts (reversing direction),
so a signed `power_of_2` argument needs no special treatment. One case needs care, and gets its
own message: `Integer` division truncates while `>>` takes the floor, so dividing an `Integer`
converts to `shr_round` with `Down` (or `>>` if the floor is really what's wanted). Tests, demos,
and test utilities are exempt: they multiply by `power_of_2` on purpose, to cross-check the shift
operators themselves.

### `long_lines`

Flags source lines longer than `max_line_length` characters (default 100), ignoring trailing
whitespace. `rustfmt` keeps code within the limit but cannot split long string literals or
Markdown constructs in doc comments; this catches those.

Each long line is attributed to the innermost item containing it — doc comments belong to the
item they document — so a line that genuinely cannot be shortened (a long Markdown table row or
link) is exempted by annotating that item:

```rust
#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
pub mod mod_op;
```

`expect` rather than `allow` keeps exemptions from going stale: if the item no longer contains a
long line, the unfulfilled expectation is reported (and `additional-lints.sh` denies it). The
`cfg_attr` guard keeps normal builds from complaining about an unknown lint; the `dylint_lib` cfg
is declared in each crate's `[lints.rust] unexpected_cfgs` table.

Crate-level `//!` doc lines have no containing item (a crate-level `#![expect]` would exempt the
entire crate), so those few are listed in `dylint.toml` under `long_lines_exceptions`, with the
same staleness guarantee: an entry whose line is no longer too long is itself flagged.

### `use_named_constant`

Flags constructing one of the named bignum constants (`ZERO`, `ONE`, `TWO`, `NEGATIVE_ONE`,
`ONE_HALF`) the long way: `from` (or `const_from_unsigned`/`const_from_signed`) of a literal 0,
1, 2, or -1; `Rational::from_unsigneds(1, 2)` or `from_signeds(1, 2)`; or, for `Float`, the
dedicated constructors `one_prec`, `two_prec`, `negative_one_prec`, and `one_half_prec` with a
literal precision of 1. The named constant says what the value is at a glance and involves no
conversion. For `Float`, only precision-1 constructions are flagged: the named constants have
precision 1, so `Float::one_prec(p)` with any other `p` is not the same value-and-precision.
Tests, demos, and test utilities are exempt.

### `runtime_literal_conversion`

Flags bignum conversions of integer literals that happen at runtime — `Natural::from(100u32)` and
the like, and `Rational::from_unsigneds`/`from_signeds` of two literals — as well as any
`const_from*` call outside a const context. A literal conversion can happen at compile time: use
`const_from*` (`Natural::const_from`; `const_from_unsigned`/`const_from_signed` for the other
types; `Rational::const_from_unsigneds`/`const_from_signeds` for fractions) inside a `const`
block or a named `const`. A bare `const_from*` call at runtime is itself flagged: it wastes the
intent of compile-time evaluation, and for the fraction constructors it is genuinely slower
(`const_from_unsigneds`' naive Euclidean gcd measures ~4x worse than `from_unsigneds` on
gcd-heavy inputs). `const fn` bodies count as const contexts. Literals 0, 1, 2, and -1 (and the
fraction (1, 2)) are `use_named_constant`'s territory, and literals that don't fit in a 32-bit
limb are skipped (`const_from*` takes a `Limb`, so they would not compile under `32_bit_limbs`).

### `use_assign_variant`

Flags reassigning a bignum the result of a method on itself — `x = x.add_prec(y, p).0`,
`x = (&x).abs()`, `x = x.clone().neg()` — when an in-place `*_assign*` companion exists (inherent
`{name}_assign`/`_assign_ref`, or a `{Name}Assign` trait in `malachite-base`). The in-place form
avoids a needless move or clone of a potentially huge value. Operator forms (`x = &x * &y`) are
clippy's `assign_op_pattern` territory. The assign variants' own defining delegations are exempt.

### `clone_with_ref_variant`

Flags cloning a bignum where a by-reference alternative exists: `x.clone().op(..)` when the
family has a receiver-by-reference sibling (`op_ref`, `op_ref_val`, `op_ref_ref`),
`y.op(x.clone(), ..)` when it has an argument-by-reference sibling (`op_val_ref`, `op_ref_ref`,
`op_assign_ref`), and `x.clone() * y` / `x += y.clone()` when the operator is implemented for
references. Cloning a bignum can copy an arbitrarily large value; the `_val`/`_ref` families and
the reference operator impls exist precisely to avoid that.

### `manual_rational_significant_bits`

Flags `x.numerator_ref().significant_bits() + x.denominator_ref().significant_bits()` (in either
order, and with the `to_numerator`/`to_denominator` accessors too) for a [`Rational`] `x`. That sum
is exactly what `Rational::significant_bits` returns, in constant time; write `x.significant_bits()`.

### `redundant_from_in_literal_comparison`

Flags widening a primitive integer with `from` only to compare the result with an integer literal,
like `i64::from(x) <= 32`. Because `from` is an exact, value-preserving conversion, dropping it
leaves the comparison unchanged as long as the literal is representable in the source type: `x <= 32`
(the literal takes the source type). Fires for every comparison operator and either operand order,
but not when the literal is out of the source type's range (then the conversion is load-bearing) or
when the other operand is not a literal. Distinct from `redundant_from_in_comparison`, which is about
comparing a *bignum* with `Bignum::from(primitive)`.

### `use_square`

Flags multiplying a bignum by itself (`&x * &x`) and raising one to the power of 2 (`x.pow(2)`,
`x.pow_assign(2)`): squaring has a dedicated, faster implementation — `square()`, `(&x).square()`,
or `square_assign()`.

### `use_parity`

Flags parity tests of a `Natural` or `Integer` spelled as `x % 2 == 0` (or compared with 1, with
`% 2` written with a literal or the `TWO` constant) or as `divisible_by(2)`: use `even()` and
`odd()`.

### `use_reciprocal`

Flags dividing `ONE` by a `Rational` or a `Float` (`Float::ONE / x`): use `reciprocal()`,
`reciprocal_assign()`, or (for `Float`) the `reciprocal_prec*` family for a specific output
precision.

### `let_tuple_underscore_to_field`

Flags a `let` that destructures a tuple only to keep one field and discard the rest with `_` —
`let (x, _) = f();`, `let (_, o) = f();` — suggesting direct field access (`let x = f().0;`,
machine-applicable). Malachite functions pervasively return `(value, Ordering)` tuples; field
access is shorter and leaves the initializer as an expression that can be chained (`f().0.g().0`)
instead of forcing an intermediate binding. Tuples with two or more real bindings, `..` rest
patterns, and `ref` bindings are exempt.

### `assert_ordering_equal_prefer_exact`

Flags binding a `(value, Ordering)` result and then asserting the ordering is `Equal`
(`let (x, o) = f(..); assert_eq!(o, Equal);`) when the function has a `_round` sibling: call
`f_round(.., Exact)` and take `.0` instead. The `Exact` rounding mode *is* the assertion — it
panics if the result is not exactly representable — and it is also usually faster, since the
default `Nearest` does more work than the other modes.

### `assign_then_consumed_once`

Flags a freshly bound mutable bignum that is mutated in place by a single `*_assign*` call and
then moved exactly once (`let mut t = f(..).0; t.op_assign(y); g(t)`): thread the value through a
by-value variant in a chain instead (`g(f(..).0.op(y).0)`). This is the near-inverse of
`use_assign_variant`, which prefers the in-place form when a *persisted* variable is reassigned
its own result; the two do not overlap, because here the binding is fresh and consumed once. GMP,
FLINT, and MPFR's assembly-like bind-mutate-move shape reads naturally in C, but in Malachite
chaining is the idiom.

## Tests

`cargo test` in this directory runs UI tests: `ui/main.rs` exercises `long_lines` (the basic
case, `dylint.toml` exceptions and their staleness, and `allow`/`expect` attribute exemptions and
their staleness), and the files under `examples/` exercise the two type-aware lints against real
Malachite types.
