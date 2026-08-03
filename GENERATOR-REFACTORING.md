# Generator refactoring: consumer-scoped modules

Instructions for migrating the test-generator system away from `var_N` names. Written 2026-08-02
for a future session to execute incrementally. The design was discussed and endorsed in outline;
the checkpoints below mark the decisions that still need Mikhail's sign-off as the work
progresses. Read the whole document before starting.

## Why

The current system names generators by tuple shape plus a global counter:
`natural_natural_unsigned_rounding_mode_quadruple_gen_var_2`. Observed failure modes, each hit
in practice:

- The *constraint* is the semantic payload ("bits is near the top of the product, so the
  short-product path is dense"), and it is exactly what the name omits. Call sites are opaque.
- Each generator spans four files (`generators/mod.rs` plus `exhaustive.rs`, `random.rs`,
  `special_random.rs`), and the `var` numbering across them is only conventionally aligned. It
  has already slipped: there is a `mod.rs` `var_4` delegating to an `exhaustive.rs` `var_5`.
  Never resolve a generator by name; follow the `Generator::new` references.
- Discoverability is grep-and-hope: "does a generator for valid `mul_shr_round` inputs exist?"
  has no better answer than grepping shape prefixes and reading comments.

Scale, measured 2026-08-02 (count of `pub fn *_gen_var_N` in each `generators/mod.rs`):
base 406, nz 299, q 68, float 371 — **1144 numbered generators**, ~89,000 lines across the
generator files. This is a campaign, not a task. Do not attempt it in one session.

## Target design

One module per *consumer operation*, holding that operation's generators with short local names
and all three modes colocated:

```text
malachite_nz::test_util::generators::mul_shr_round::valid()
    // -> Generator<(Natural, Natural, u64, RoundingMode)>
malachite_nz::test_util::generators::mul_shr_round::high_shift()
    // -> the short-product-dense variant
```

Call sites read `mul_shr_round::valid().test_properties(…)`. The scope carries the context, so
names inside it stay short. Everything stays statically typed; nothing is looked up by string.

Per-module file shape (this is the template; `<op>.rs` lives in
`src/test_util/generators/<op>.rs` of the crate that owns the operation's type):

```rust
// Copyright header as usual.

//! Generators for `<op>`'s tests, demos, and benches.

use ...; // whatever the three modes need

// All `(T, U, ...)` that are valid inputs to `<op>`: <the constraint, in one sentence>.
pub fn valid() -> Generator<(T, U, ...)> {
    Generator::new(&exhaustive_valid, &random_valid, &special_random_valid)
}

fn exhaustive_valid() -> It<(T, U, ...)> { ... }                       // no config
fn random_valid(config: &GenConfig) -> It<(T, U, ...)> { ... }        // seeded from EXAMPLE_SEED
fn special_random_valid(config: &GenConfig) -> It<(T, U, ...)> { ... } // striped variants
```

Rules the template encodes:

- The three mode functions are **private** and referenced only from the one public function, so
  numbering skew is structurally impossible.
- The `Generator::new` slots are typed differently: the exhaustive slot is a config-free
  `fn() -> It<T>`; the random and special slots take `&GenConfig`. (A variant with no natural
  exhaustive form still needs a real exhaustive function — map/filter over exhaustive inputs —
  because the slot types differ; do not pass a random function in the exhaustive slot.)
- Every public generator carries a doc comment stating its constraint. The catalog check
  (below) enforces this.
- Shared validity predicates (e.g. `mul_shr_round_valid`) live in the same module, `pub(crate)`
  if other generator modules need them.

What does **not** change:

- The plain, unconstrained, unnumbered generators (`natural_pair_gen`,
  `unsigned_gen`, …) keep their current names and homes. They are fine.
- The combinator layer (`exhaustive_pairs_from_single`, `random_triples`, `lex_pairs`, striped
  sources, `geometric_random_unsigneds`, `It`, `GenConfig`, `EXAMPLE_SEED`) is untouched; the
  per-operation functions are written on top of it, as today.
- Demo/bench runner keys (they reference demo *function* names, not generator names).

## Hard constraints

1. **Bit-identical streams.** Property-test reproducibility depends on the exact sequence each
   generator emits. Migration means *moving code verbatim*, adjusting only names and imports.
   Any change to seeds, combinator structure, filter/map order, or config-key strings
   (`"mean_bits_n"`, `"mean_shift_n"`, …) changes the streams and is out of scope. If a
   generator's body looks improvable, note it in the tracking file and leave it.
2. **Compile-time typing preserved.** No string registries, no runtime lookup.
3. **Generics preserved.** Many base generators are generic over the value and bits types
   (`<T: PrimitiveUnsigned, U: PrimitiveUnsigned>` with `where` clauses like
   `u64: SaturatingFrom<U>`). The scoped functions keep the same signatures.
4. **Each batch leaves the tree green** (see the verification protocol).

## Migration mechanics

Work in per-operation batches, atomically: move the generators, update every call site, delete
the old names, all in one batch. Do **not** build a deprecated-alias layer inside a crate — each
`var_N` typically has only one to three call sites (test file, demo file, sometimes a bench),
so atomic renames are cheaper than alias bookkeeping and avoid deprecation-warning noise.

The exception is malachite-base generators consumed by nz/q/float: update the downstream call
sites in the same batch. If a base generator has many cross-crate consumers, it may deserve a
themed shared module instead of an operation module (checkpoint 2 below).

Per-batch procedure:

1. Pick an operation (start with ones added recently; their generators are few and clean).
2. Find its generators: grep the operation's test/demo files for `_gen` imports, then follow
   each `Generator::new` in `generators/mod.rs` to the three mode functions **by reference,
   not by name**.
3. Create `src/test_util/generators/<op>.rs`; move the mode-function bodies verbatim; wire the
   template above. Register `pub mod <op>;` in `generators/mod.rs` (or the generators
   directory's module file — keep one convention).
4. Update call sites (tests, demos, benches) to `use …::generators::<op>;` +
   `<op>::valid()`.
5. Delete the old `var_N` functions from all four files.
6. Run the verification protocol.
7. Update the tracking table at the bottom of this file.

Choosing local names: `valid` for "all valid inputs to the operation" (the most common
constraint); descriptive short names for variants (`high_shift`, `nonzero_divisor`,
`extreme`). If a name needs a sentence, put the sentence in the doc comment and keep the name
short.

## The catalog check

The "global count is hard to track" problem is solved by a generated inventory, not by keys.
Add a script (`generator-catalog.py`, repo root, modeled on `unincluded-files.py`) that:

- walks every `src/test_util/generators/` tree,
- extracts each `pub fn` and its doc comment,
- fails if any public generator lacks a doc comment,
- writes/updates a checked-in `docs/generator-catalog.md` index (name, crate, module, type,
  constraint sentence).

Wire it into `additional-lints.sh` next to the unincluded-files check. Build this **during the
pilot**, not at the end — it pays for itself immediately as migration progress tracking.

## Verification protocol (per batch)

From the repo root, with `-j 4` always (full parallelism has OOM-crashed this machine):

1. `cargo check --all-targets --features bin_build -j 4` in every touched crate. For nz also
   `--features bin_build,32_bit_limbs`.
2. Run the moved generators' property tests: `cargo test --release --features bin_build -j 4
   --test lib <op>` in the owning crate (release, not debug — some suites contain extreme
   tests).
3. Run one demo per moved generator (`cargo run --release --features bin_build -j 4 -- -l 3 -m
   exhaustive -d demo_<…>`) to prove the wiring; demos validate generators before tests do.
4. `bash additional-lints.sh` from the repo root (it includes the unincluded-files check, which
   catches a forgotten `pub mod` — new files are invisible to the compiler and every lint
   without one).
5. `cargo fmt --all`, then re-run the lint sweep if formatting touched anything.

Known traps, all hit in past sessions:

- A sub-second clippy/lint result is a **cache replay**; `touch src/lib.rs` before trusting a
  clean sweep, and plant a deliberate violation if the result decides anything important.
- Piping build/test output through `head`/`tail` can kill the command mid-run and fake a green
  exit; redirect to a file and grep the file.
- `use` lines over 100 columns must become `::*` globs (house rule), and `superfmt.sh` (run
  from the crate dir) reflows comments; run it if you edit doc comments.
- The nz test suite for any module containing an extreme test must run with `--release`.
- Property-test call sites sometimes pass `GenConfig` overrides keyed by strings
  (`"mean_bits_n"`); those keys must survive the move exactly.

## Order of work

1. **Pilot** (one session): the `mul_shr_round` generators — 2 in base
   (`unsigned_unsigned_unsigned_rounding_mode_quadruple_gen_var_1`,
   `signed_signed_unsigned_rounding_mode_quadruple_gen_var_1`, generic, with base-internal call
   sites only) and 3 in nz (`natural_natural_…_var_1`, `…_var_2`,
   `integer_integer_…_var_1`). They are recent, clean, and their authorship pain motivated this
   design. Build the catalog script in the same session.
2. **Checkpoint with Mikhail** (see below) before proceeding.
3. **Batches**: q (68 generators — smallest full crate, good second step), then nz by
   arithmetic module, then float, then base last (largest, generic, cross-crate consumers).
4. New generators use the new style immediately, regardless of migration progress.

## Checkpoints requiring Mikhail's decision

1. After the pilot: does the colocated-modes layout read well enough to give up the
   "read `random.rs` as one document" property? (Recommendation: yes, but he sees it daily.)
2. Placement rule for generators shared across operations: primary-owner-plus-reexport, or
   promotion to a themed module (`generators::rounding::…`)? Sharing is the signal that a
   constraint deserves a name; the rule needs his taste.
3. Whether plain-type sugar (`Natural::exhaustive()` via a trait) is wanted at all — pure
   ergonomics, orthogonal to the migration, skippable.
4. Whether demo/bench runner keys should become hierarchical to match — separate decision,
   defer until the generator migration is well along.

## Tracking

Maintain this table as batches land (counts from 2026-08-02):

| crate | var_N at start | migrated | remaining |
|---|---|---|---|
| malachite-base | 406 | 0 | 406 |
| malachite-nz | 299 | 0 | 299 |
| malachite-q | 68 | 0 | 68 |
| malachite-float | 371 | 0 | 371 |

Notes discovered during migration (name skews, shared generators, bodies that deserve later
improvement) go here rather than being fixed in-flight.
