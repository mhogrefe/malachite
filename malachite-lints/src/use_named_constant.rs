// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags constructing one of the named bignum constants (`ZERO`, `ONE`, `TWO`,
    /// `NEGATIVE_ONE`, `ONE_HALF`) the long way: `from` (or `const_from_unsigned`/
    /// `const_from_signed`) of a literal 0, 1, 2, or -1; `Rational::from_unsigneds(1, 2)` or
    /// `from_signeds(1, 2)`; or, for `Float`, the dedicated constructors `one_prec`, `two_prec`,
    /// `negative_one_prec`, and `one_half_prec` with a literal precision of 1.
    ///
    /// ### Why is this bad?
    ///
    /// The named constant says what the value is at a glance and involves no conversion. For
    /// `Float`, only precision-1 constructions are flagged: the named constants have precision 1,
    /// so `Float::one_prec(p)` with any other `p` is not the same value-and-precision.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let x = Float::one_prec(1);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let x = Float::ONE;
    /// ```
    pub USE_NAMED_CONSTANT,
    Deny,
    "constructing a named bignum constant instead of using it"
}

declare_lint_pass!(UseNamedConstant => [USE_NAMED_CONSTANT]);

impl<'tcx> LateLintPass<'tcx> for UseNamedConstant {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        // Tests, demos, and test utilities construct constants the long way on purpose, to
        // exercise the constructors themselves.
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::Call(callee, args) = expr.kind else {
            return;
        };
        let ExprKind::Path(qpath) = &callee.kind else {
            return;
        };
        let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
            return;
        };
        let fn_name = cx.tcx.item_name(fn_did);
        let fn_name = fn_name.as_str();
        let Some(t_name) = crate::bignum_name(cx, cx.typeck_results().expr_ty(expr)) else {
            return;
        };
        let konst = match (t_name, fn_name, args) {
            // A `Float`'s named constants have precision 1, so only the dedicated constructors
            // with a literal precision of 1 construct exactly them.
            ("Float", "one_prec", [p]) if crate::literal_value(p) == Some(1) => "ONE",
            ("Float", "two_prec", [p]) if crate::literal_value(p) == Some(1) => "TWO",
            ("Float", "negative_one_prec", [p]) if crate::literal_value(p) == Some(1) => {
                "NEGATIVE_ONE"
            }
            ("Float", "one_half_prec", [p]) if crate::literal_value(p) == Some(1) => "ONE_HALF",
            ("Float", ..) => return,
            (_, "from" | "const_from" | "const_from_unsigned" | "const_from_signed", [a]) => {
                match crate::literal_value(a) {
                    Some(0) => "ZERO",
                    Some(1) => "ONE",
                    Some(2) => "TWO",
                    Some(-1) => "NEGATIVE_ONE",
                    _ => return,
                }
            }
            ("Rational", "from_unsigneds" | "from_signeds", [n, d])
                if crate::literal_value(n) == Some(1) && crate::literal_value(d) == Some(2) =>
            {
                "ONE_HALF"
            }
            _ => return,
        };
        span_lint(
            cx,
            USE_NAMED_CONSTANT,
            expr.span,
            format!("use `{t_name}::{konst}` instead of `{fn_name}(..)`"),
        );
    }
}
