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
is exactly what `Rational::significant_bits` returns, in constant time; write
`x.significant_bits()`.

### `redundant_from_in_literal_comparison`

Flags widening a primitive integer with `from` only to compare the result with an integer literal,
like `i64::from(x) <= 32`. Because `from` is an exact, value-preserving conversion, dropping it
leaves the comparison unchanged as long as the literal is representable in the source type:
`x <= 32` (the literal takes the source type). Fires for every comparison operator and either
operand order, but not when the literal is out of the source type's range (then the conversion is
load-bearing) or when the other operand is not a literal. Distinct from
`redundant_from_in_comparison`, which is about comparing a *bignum* with `Bignum::from(primitive)`.

### `use_square`

Flags multiplying a bignum by itself (`&x * &x`) and raising one to the power of 2 (`x.pow(2)`,
`x.pow_assign(2)`): squaring has a dedicated, faster implementation — `square()`, `(&x).square()`,
or `square_assign()`.

### `use_parity`

Flags parity tests spelled the long way — `x % 2` or `x & 1` compared with 0 or 1 (with `% 2`
written as a literal or the `TWO` constant), or `divisible_by(2)` — for a `Natural`, an `Integer`,
or a primitive integer: use `even()` and `odd()`. The `% 2 == 1` / `!= 1` forms are skipped for
*signed* primitives, whose remainder can be `-1`, so `% 2` does not test oddness there; the
`== 0` / `!= 0` forms and every `& 1` form are safe for all integer types.

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

### `manual_float_from_primitive`

Flags constructing a [`Float`] from a primitive integer at exactly its own significant-bit
precision and discarding the ordering, like `Float::from_unsigned_prec(x, x.significant_bits()).0`
(with or without a `.max(1)` or `max(.., 1)` guard, and including the `_round` variants, whose
rounding mode is equally dead). That is an exact conversion — precisely `Float::from(x)`, which is
shorter and also handles `x == 0`, where the unguarded spelling panics on the zero precision. The
machine-applicable suggestion spells the type `Self` inside an `impl Float` so it does not trip
`use_self`.

### `manual_from_sign_and_abs`

Flags building a signed [`Integer`] as a magnitude and then conditionally negating it in place:

```rust,ignore
let mut a = Integer::from(nat);
if negative {
    a.neg_assign();
}
```

That is `Integer::from_sign_and_abs(!negative, nat)` (or `from_sign_and_abs_ref`) in one step,
without the `mut` binding. Does not fire when the condition reads the freshly built value itself
(then no `from_sign_and_abs` form exists).

### `redundant_prec_round_of_exact_constant`

Flags rounding one of the named [`Float`] constants that are exact at every precision (`ONE`,
`TWO`, `NEGATIVE_ONE`, `ONE_HALF` — the single-significant-bit values) to a precision, like
`Float::from_float_prec_round(Float::ONE, prec, rm)`, in all four `from_float_prec*` spellings
(by value or by reference). The rounding is a no-op: the rounding mode is dead and the ordering is
always `Equal`, so write the dedicated constructor, `(Float::one_prec(prec), Equal)`.

### `compare_with_primitive`

Flags comparing a bignum with a named bignum constant (`ZERO`, `ONE`, `TWO`, `NEGATIVE_ONE`) or
with `Bignum::from(primitive)` in method position (`cmp`, `partial_cmp`, `eq`, `ne`), suggesting a
direct comparison against the primitive: `*x == 1u32`, `x.partial_cmp(&1u32).unwrap()`. The bignum
types implement `PartialEq`/`PartialOrd` against the primitives (a total order, since a bignum can
represent every primitive value), so neither a constant nor a conversion is needed. Unsigned
literals are preferred for nonnegative values. Operator-position `from` comparisons belong to
`redundant_from_in_comparison`; this covers the method forms and the named-constant comparisons.

### `use_round_variant`

Flags `x.foo_prec_round(p, rm)` or `x.foo_prec(p)` when `x` is an immutable local whose `let`
initializer already pins its precision to that same expression `p` — the explicit precision is
redundant. Use the `foo_round(rm)` shorthand, or plain `foo()` when the `_prec` call's `Ordering`
is discarded with `.0` (`t.ln_prec(wp).0` becomes `t.ln()`, not `t.ln_round(Nearest).0`). Binary
operations (`x.mul_prec_round(y, p, rm)`) fire only when *every* `Float` operand is independently
precision-pinned, since the `_round` variants compute at the maximum of the operand precisions. The
lint tracks the binding through `.0`/tuple patterns and bails if any precision-source local is
reassigned or mutably borrowed before the use.

### `use_saturating_from`

Flags `T::exact_from(EXPR.max(0))` (and `0.max(EXPR)`) for an *unsigned* `T`: use
`T::saturating_from(EXPR)`. The `.max(0)` already commits to clamping the low end, so pairing it
with `exact_from`'s panic-on-overflow is inconsistent; `saturating_from` clamps both ends (its low
bound of 0 for an unsigned target is exactly what `.max(0)` does), and is equivalent whenever the
source cannot exceed `T`'s maximum. Only unsigned targets fire — signed `saturating_from` clamps
the low end to `MIN`, not 0, which would differ.

### `use_divisible_by`

Flags `x % b == 0` or `x % b != 0` for a primitive integer, `Natural`, or `Integer`: use
`x.divisible_by(b)` (or `!x.divisible_by(b)`). Divisor 2 is excluded and left to `use_parity`.

### `use_width_mask`

Flags `x % T::WIDTH` for a primitive integer: since a type's bit width is a power of two, this is
`x & T::WIDTH_MASK`, which is cheaper and states the bit-masking intent directly. `WIDTH_MASK` is a
`u64` like `WIDTH`, so there is no type mismatch. The `% WIDTH == 0` / `!= 0` forms are left to
`use_divisible_by`.

### `use_checked_log_base_2`

Flags an `if x.is_power_of_2() { .. }` whose body then takes `x.floor_log_base_2()` (or
`floor_log_base_2_abs()`) of the same receiver: `checked_log_base_2()` returns `Some(exact log)`
exactly when the value is a power of two, so `if let Some(e) = x.checked_log_base_2()` tests the
condition and produces the log in one call. Fires only for receivers that have `CheckedLogBase2` —
a primitive integer, a `Natural`, or a `Rational` (not `Integer` or `Float`).

### `missing_inline_on_delegator`

Flags a public function whose entire body is a single forwarding call — a thin delegator like
`fn foo(&self) -> T { self.inner.foo() }` — that lacks `#[inline]`. Trivial wrappers are not
inlined across crate boundaries without the attribute, defeating the point of the delegation.
Construction (a body that is just a `Ctor` call) is not delegation and is not flagged; functions
already carrying `#[inline]` (or `#[inline(never)]`) are left alone.

### `use_trailing_zeros`

Flags a loop that strips trailing zero bits one at a time — `while x.even() { x >>= 1; }`, with an
optional `counter += 1` — and suggests `x.trailing_zeros()`, which computes the same shift count
directly. The loop body may do the `x >>= 1` and at most one counter update on a different place;
anything else (a `let`, a second shift) bails.

### `use_exact_from`

Flags `T::try_from(x).unwrap()` for an integer target `T`: use `T::exact_from(x)`, Malachite's
idiom for a conversion that panics on an out-of-range value. The two are equivalent for integers,
since `try_from` fails exactly when the value is out of range. The integer-target guard leaves
`char`, `NonZero`, and other non-range `TryFrom` uses alone.

### `shift_of_one`

Flags shifting `1` or `T::ONE` left by an amount, where a named `malachite-base` helper reads at
the level of the operation: `(1 << n) - 1` builds a low-`n`-bit mask, so `T::low_mask(n)`;
`x & (1 << n) != 0` (or `== 0`) tests bit `n`, so `x.get_bit(n)` (or `!x.get_bit(n)`); and any
other `1 << n` is two-to-the-`n`, so `T::power_of_2(n)`. A *constant* shift amount is left alone
(`1 << 70` folds at compile time, but `power_of_2(70)` is a runtime call), as are const contexts (a
`const`/`static` item or a `const fn`), where the helpers — not being const fns — could not be
called anyway.

### `mul_div_by_power_of_2_literal`

Flags multiplying or dividing a primitive integer by a power-of-two literal (`x * 8`, `x / 16`, and
the `*=`/`/=` forms): use a shift. This is the primitive-integer companion of
`mul_div_by_power_of_2`, which covers the bignum `x * T::power_of_2(k)` spelling. Signed division
truncates toward zero while `>>` takes the floor, so a signed `/` converts to `shr_round(k, Down)`
(or plain `>>` when the floor is wanted). Unlike `*`, a shift does not detect value overflow — `<<`
drops the high bits where `*` would panic in a debug build — so `<<` is appropriate only where
overflow is already ruled out.

### `use_const_binding`

Flags an immutable `let` whose initializer is a *derived* compile-time constant — arithmetic, a
unary operation, or a cast that mentions at least one named constant, like
`let rnd_bit = Limb::WIDTH - 5;`. Such a value reads more clearly as a named `const`, which
announces that it is fixed at compile time, is computed once, and can be referred to by an
unambiguous `SCREAMING_SNAKE_CASE` name — and it lets other lints (such as `shift_of_one`, which
treats a constant shift amount specially) recognize the operand as constant without tracing the
`let` back to its initializer. Only *derived* constants are flagged: a bare literal is already
clear, and a bare path to an existing constant would just be a rename. Initializers that do not
evaluate at compile time are left alone, which also excludes anything depending on a generic
parameter, where a `const` item could not name it.

### `use_const_block`

Flags a *derived* compile-time constant that appears as a subexpression of a larger runtime
expression — arithmetic, a unary operation, a comparison, or a cast built from named constants, like
`RND_BIT + 1` in `msl >> (RND_BIT + 1)`, `Limb::ONE << RND_BIT` in `msl & (Limb::ONE << RND_BIT)`,
or `!SOME_FLAG` in a runtime condition. Such a constant island reads as if it were recomputed each
time; folding it to its value or wrapping it in a `const { .. }` block makes the compile-time
evaluation explicit and guaranteed. It is the subexpression companion of `use_const_binding` (which
covers a whole `let`). Only the *maximal* constant subexpression is flagged (the largest one whose
enclosing expression is not itself a constructible constant), and only when it is *derived* — built
from at least one named constant, not a bare literal computation the compiler folds anyway. A
subexpression that does not evaluate at compile time is left alone, as is one that mentions a local
— including a `bool` that short-circuiting makes constant-valued while it still names a runtime
operand (`SOME_CONST && n < THRESHOLD`), which could not be lifted into a `const { .. }` block.
Anything already inside a `const { .. }` block or a const context is skipped.

### `use_const_cast`

Flags a numeric conversion of a `const { .. }` block — either an `as` cast
(`const { (A - B) << 1 } as f64`) or a `from`/`exact_from`/`wrapping_from` call
(`u64::exact_from(const { Self::MAX_EXPONENT - 1 })`). The whole expression is a compile-time
constant, so the conversion should be an `as` cast *inside* the block:
`const { ((A - B) << 1) as f64 }` / `const { (Self::MAX_EXPONENT - 1) as u64 }`. Folding it in
evaluates the whole thing once, at compile time; for a value representable in the target type
— which a working conversion guarantees — the cast gives the same result. It complements
`use_const_block`, which wraps the constant argument in the first place: once the argument is a
`const { .. }` block, the conversion belongs inside it (`use_const_block` leaves it out because
`ConstEvalCtxt` does not fold `as` casts, so the block stops at the integer operand).

### `duplicate_const`

Flags a *derived* compile-time constant that is written out more than once with the same value —
whether as an inline `const { .. }` block (`const { Limb::WIDTH - 1 }`) or as a `const` item
(`const TWICE_WIDTH: u64 = Limb::WIDTH << 1;`) — anywhere in the crate. Repeating the same
computation invites drift and hides that the values are meant to be equal; a single named constant
(a `pub(crate)` associated constant on the relevant type, or a standalone `const`) states it once
and lets every site refer to it.

Only computed constants are considered — a binary/unary op or a cast — so bare literals, renames,
and array/struct/call constants are ignored. Instances are grouped by their source text *and* by
the `DefId`s of the `const`s they reference, so two `const { UPPER_LIMIT - 1 }` blocks whose
`UPPER_LIMIT` is a different (scope-local) constant are *not* merged: only genuinely identical
values are flagged. Because grouping is textual, the same value written two different ways
(`i64::WIDTH - 1` versus `Limb::WIDTH - 1`) is left alone — which is correct, since those two
differ on 32-bit platforms.

## Tests

`cargo test` in this directory runs UI tests: `ui/main.rs` exercises `long_lines` (the basic
case, `dylint.toml` exceptions and their staleness, and `allow`/`expect` attribute exemptions and
their staleness), and the files under `examples/` (one per lint, run by `ui_examples`) exercise the
type-aware lints against real Malachite types.
