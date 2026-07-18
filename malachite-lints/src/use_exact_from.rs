// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::source::snippet;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags `T::try_from(x).unwrap()` for an integer target `T`, such as
    /// `usize::try_from(n).unwrap()`.
    ///
    /// ### Why is this bad?
    ///
    /// `ExactFrom::exact_from` is Malachite's idiom for a conversion that panics when the value is
    /// not exactly representable, and it is shorter and clearer than `try_from(...).unwrap()`. For
    /// an integer target the two are equivalent: `try_from` fails precisely when the value is out
    /// of range, which is exactly when `exact_from` panics.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let n = usize::try_from(x).unwrap();
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let n = usize::exact_from(x);
    /// ```
    pub USE_EXACT_FROM,
    Deny,
    "`try_from(...).unwrap()` into an integer type instead of `exact_from`"
}

declare_lint_pass!(UseExactFrom => [USE_EXACT_FROM]);

impl<'tcx> LateLintPass<'tcx> for UseExactFrom {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        // `<recv>.unwrap()`.
        let ExprKind::MethodCall(seg, recv, [], _) = expr.kind else {
            return;
        };
        if seg.ident.name.as_str() != "unwrap" {
            return;
        }
        // `recv` is `T::try_from(arg)`, where `try_from` is an associated function.
        let ExprKind::Call(callee, [arg]) = recv.kind else {
            return;
        };
        let ExprKind::Path(qpath) = &callee.kind else {
            return;
        };
        let Res::Def(DefKind::AssocFn, did) = cx.qpath_res(qpath, callee.hir_id) else {
            return;
        };
        if cx.tcx.item_name(did).as_str() != "try_from" {
            return;
        }
        // The target (the type of the whole `unwrap()` expression, since `unwrap` yields `T`) must
        // be an integer, so that `exact_from` applies and is equivalent. This excludes, for
        // example, `char::try_from` and fallible conversions whose error is not about range.
        if !cx.typeck_results().expr_ty(expr).is_integral() {
            return;
        }
        let suggestion = format!(
            "{}({})",
            snippet(cx, callee.span, "..").replace("try_from", "exact_from"),
            snippet(cx, arg.span, ".."),
        );
        span_lint_and_help(
            cx,
            USE_EXACT_FROM,
            expr.span,
            "`try_from(...).unwrap()` into an integer type",
            None,
            format!("use `{suggestion}`"),
        );
    }
}
