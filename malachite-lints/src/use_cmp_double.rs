// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags comparing a bignum (`Natural`, `Integer`, `Rational`, `Float`, `GaussianInteger`, or `GaussianRational`) against another
    /// value doubled by `<< 1`, like `x.cmp(&(y << 1))` or `x < y << 1`. Covers the comparison
    /// operators, `cmp`, `partial_cmp`, `lt`/`le`/`gt`/`ge`, `eq`/`ne`, and each of their `_abs`
    /// counterparts; the shift amount may be written `1` or `T::ONE`. The doubled value may be on
    /// either side.
    ///
    /// Also flags the mirror image, comparing against a value *halved* by `>> 1`, but only for
    /// `<=` and `>` (and the mirrored `>=` and `<`). `a <= b >> 1` is exactly `2a <= b`, since
    /// `a <= floor(x)` iff `a <= x` for integral `a` — but `a < b >> 1` is *not* `2a < b`, and the
    /// `_abs` comparisons do not survive the flooring either, so those are left alone.
    ///
    /// ### Why is this bad?
    ///
    /// `<< 1` on a bignum allocates a whole new number just to throw it away after the comparison.
    /// The `*_double` comparison methods answer the same question directly, by comparing against
    /// twice the other value without ever forming it.
    ///
    /// Primitives are deliberately not flagged: there `<< 1` is a single instruction, while
    /// `cmp_double` is a call that must also rule out overflow. It is the better choice when the
    /// shift really could overflow, but that is a judgement about the surrounding code, not
    /// something the spelling alone settles.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// x.cmp(&(y << 1))
    /// if x.lt_abs(&(y << 1)) { .. }
    /// if r <= m >> 1 { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// x.cmp_double(&y)
    /// if x.cmp_abs_double(&y) == Less { .. }
    /// if m.cmp_double(&r) != Less { .. }
    /// ```
    pub USE_CMP_DOUBLE,
    Deny,
    "comparing against a value doubled by `<< 1` instead of using the `*_double` comparisons"
}

declare_lint_pass!(UseCmpDouble => [USE_CMP_DOUBLE]);

const TRAITS: &str = "malachite_base::num::comparison::traits";

// The `*_double` methods that could replace a comparison, most specific first. A comparison spelled
// `cmp` can only become `cmp_double`, but a predicate like `<` or `lt_abs` is happy with either the
// total or the partial variant, so those list both and the first one the type implements wins.
fn candidates(name: &str) -> &'static [(&'static str, &'static str)] {
    match name {
        "cmp" => &[("OrdDouble", "cmp_double")],
        "partial_cmp" => &[("PartialOrdDouble", "partial_cmp_double")],
        "cmp_abs" => &[("OrdAbsDouble", "cmp_abs_double")],
        "partial_cmp_abs" => &[("PartialOrdAbsDouble", "partial_cmp_abs_double")],
        "eq" | "ne" | "lt" | "le" | "gt" | "ge" => &[
            ("OrdDouble", "cmp_double"),
            ("PartialOrdDouble", "partial_cmp_double"),
        ],
        "eq_abs" | "ne_abs" | "lt_abs" | "le_abs" | "gt_abs" | "ge_abs" => &[
            ("OrdAbsDouble", "cmp_abs_double"),
            ("PartialOrdAbsDouble", "partial_cmp_abs_double"),
        ],
        _ => &[],
    }
}

// The operand of a shift by one, if `e` is `y << 1` or `y >> 1` (the amount may also be spelled
// `T::ONE`) behind any borrows.
fn shifted<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
    dir: BinOpKind,
) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::Binary(op, lhs, rhs) = crate::peel_clone_and_borrows(e).kind else {
        return None;
    };
    (op.node == dir && crate::is_int_const(cx, rhs, 1, "ONE")).then_some(lhs)
}

// How to phrase a comparison against a *halved* value as one against a doubled one. `a <= b >> 1`
// is exactly `2a <= b`, because `a <= floor(x)` iff `a <= x` when `a` is an integer.
//
// Only `<=` and `>` survive the flooring. `a < b >> 1` is *not* `2a < b`: with `b = 5` and `a = 2`,
// `a < 2` is false while `2a < 5` is true. Nor do the `_abs` comparisons survive it, since
// `|floor(b/2)|` is not `floor(|b|/2)` for negative odd `b`. So this table stays deliberately
// narrow, and the lint says nothing about the spellings that do not transform.
fn halving_test(name: &str, halved_on_right: bool) -> Option<&'static str> {
    Some(match (name, halved_on_right) {
        ("le", true) | ("ge", false) => "!= Less",
        ("gt", true) | ("lt", false) => "== Less",
        _ => return None,
    })
}

impl<'tcx> LateLintPass<'tcx> for UseCmpDouble {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        // A comparison of `a` and `b`: an operator, or one of the comparison methods.
        let (name, a, b) = match expr.kind {
            ExprKind::Binary(op, lhs, rhs) => (
                match op.node {
                    BinOpKind::Eq => "eq",
                    BinOpKind::Ne => "ne",
                    BinOpKind::Lt => "lt",
                    BinOpKind::Le => "le",
                    BinOpKind::Gt => "gt",
                    BinOpKind::Ge => "ge",
                    _ => return,
                },
                lhs,
                rhs,
            ),
            ExprKind::MethodCall(seg, receiver, [arg], _) => {
                (seg.ident.name.as_str(), receiver, arg)
            }
            _ => return,
        };
        let candidates = candidates(name);
        if candidates.is_empty() {
            return;
        }
        // Whichever side is doubled, the other side is the receiver of the replacement, so it is
        // the one whose type must implement the trait.
        for (double, other, reversed) in [(b, a, false), (a, b, true)] {
            if shifted(cx, double, BinOpKind::Shl).is_none() {
                continue;
            }
            let ty = cx.typeck_results().expr_ty(other).peel_refs();
            // Only bignums: see the lint docs on why primitives are left alone.
            if crate::bignum_name(cx, ty).is_none() {
                continue;
            }
            let Some((_, method)) = candidates
                .iter()
                .find(|(t, _)| crate::implements_trait_by_path(cx, ty, &format!("{TRAITS}::{t}")))
            else {
                continue;
            };
            let (message, help) = if reversed {
                (
                    format!("use `{method}` instead of comparing a value doubled by `<< 1`"),
                    format!(
                        "the doubled value is on the left, so the operands swap: `b.{method}(&a)` \
                         answers the same question, reversed"
                    ),
                )
            } else {
                (
                    format!("use `{method}` instead of comparing with a value doubled by `<< 1`"),
                    format!(
                        "`a.{method}(&b)` compares `a` with twice `b` without ever forming twice \
                         `b`, which for a bignum is a whole wasted allocation"
                    ),
                )
            };
            span_lint_and_help(cx, USE_CMP_DOUBLE, expr.span, message, None, help);
            return;
        }
        // A comparison against a halved value says the same thing about the doubled one. Here the
        // halved side becomes the receiver, so it is the side whose type must implement the trait.
        for (half, on_right) in [(b, true), (a, false)] {
            if shifted(cx, half, BinOpKind::Shr).is_none() {
                continue;
            }
            let Some(test) = halving_test(name, on_right) else {
                continue;
            };
            let ty = cx.typeck_results().expr_ty(half).peel_refs();
            if crate::bignum_name(cx, ty).is_none() {
                continue;
            }
            let Some((_, method)) = candidates
                .iter()
                .find(|(t, _)| crate::implements_trait_by_path(cx, ty, &format!("{TRAITS}::{t}")))
            else {
                continue;
            };
            span_lint_and_help(
                cx,
                USE_CMP_DOUBLE,
                expr.span,
                format!("use `{method}` instead of comparing against a value halved by `>> 1`"),
                None,
                format!(
                    "`a <= b >> 1` is exactly `2a <= b`, so `b.{method}(&a) {test}` answers this                      without building `b >> 1`"
                ),
            );
            return;
        }
    }
}
