// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags parity tests of an integer (a primitive, `Natural`, or `Integer`) spelled as
    /// `x % 2 == 0` (or `!= 0`, or compared with 1), as `x & 1 == 0` (or the other comparisons), or
    /// as `divisible_by(2)`.
    ///
    /// ### Why is this bad?
    ///
    /// `even()` and `odd()` say what is being asked and only inspect the lowest bit.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if &x % 2u32 == 0 { .. }
    /// if n & 1 == 1 { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if x.even() { .. }
    /// if n.odd() { .. }
    /// ```
    pub USE_PARITY,
    Deny,
    "testing the parity of an integer with `% 2`, `& 1`, or `divisible_by(2)` instead of \
    `even()`/`odd()`"
}

declare_lint_pass!(UseParity => [USE_PARITY]);

// Whether `ty` (a type behind any references already peeled) has `even()`/`odd()`: a primitive
// integer, or a `Natural` or `Integer`.
fn has_parity<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> bool {
    ty.is_integral() || matches!(crate::bignum_name(cx, ty), Some("Natural" | "Integer"))
}

// The low-bit extraction underlying a parity test: `x % 2` or `x & 1`, over an integer `x`.
enum ParityForm {
    Rem2,
    And1,
}

// If `e` is `x % 2` (a literal 2 or the `TWO` constant) or `x & 1` where `x` has `even()`/`odd()`,
// returns `x` and which form it is.
fn parity_base<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
) -> Option<(&'tcx Expr<'tcx>, ParityForm)> {
    let ExprKind::Binary(op, l, r) = e.kind else {
        return None;
    };
    let form = match op.node {
        BinOpKind::Rem if crate::is_int_const(cx, r, 2, "TWO") => ParityForm::Rem2,
        BinOpKind::BitAnd if crate::is_int_const(cx, r, 1, "ONE") => ParityForm::And1,
        _ => return None,
    };
    let x = crate::peel_clone_and_borrows(l);
    has_parity(cx, cx.typeck_results().expr_ty(x).peel_refs()).then_some((x, form))
}

impl<'tcx> LateLintPass<'tcx> for UseParity {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        match expr.kind {
            ExprKind::Binary(op, lhs, rhs) if matches!(op.node, BinOpKind::Eq | BinOpKind::Ne) => {
                for (a, b) in [(lhs, rhs), (rhs, lhs)] {
                    let Some((x, form)) = parity_base(cx, a) else {
                        continue;
                    };
                    let Some(k) = crate::literal_value(b) else {
                        continue;
                    };
                    // For `x % 2`, a signed primitive can yield -1, so comparing with 1 does not
                    // test oddness; only the comparisons with 0 are safe there. `x & 1` extracts
                    // the low bit and is safe for every integer type.
                    let x_ty = cx.typeck_results().expr_ty(x).peel_refs();
                    let signed_rem = matches!(form, ParityForm::Rem2) && x_ty.is_signed();
                    let advice = match (op.node, k) {
                        (BinOpKind::Eq, 0) | (BinOpKind::Ne, 1) if !(signed_rem && k == 1) => {
                            "even()"
                        }
                        (BinOpKind::Ne, 0) | (BinOpKind::Eq, 1) if !(signed_rem && k == 1) => {
                            "odd()"
                        }
                        _ => continue,
                    };
                    let form_str = match form {
                        ParityForm::Rem2 => "% 2",
                        ParityForm::And1 => "& 1",
                    };
                    span_lint(
                        cx,
                        USE_PARITY,
                        expr.span,
                        format!("use `{advice}` instead of comparing `{form_str}` with 0 or 1"),
                    );
                    return;
                }
            }
            ExprKind::MethodCall(seg, recv, [arg], _) => {
                if seg.ident.name.as_str() != "divisible_by" {
                    return;
                }
                if has_parity(cx, cx.typeck_results().expr_ty(recv).peel_refs())
                    && crate::is_int_const(cx, arg, 2, "TWO")
                {
                    span_lint(
                        cx,
                        USE_PARITY,
                        expr.span,
                        "use `even()` instead of `divisible_by(2)`",
                    );
                }
            }
            _ => {}
        }
    }
}
