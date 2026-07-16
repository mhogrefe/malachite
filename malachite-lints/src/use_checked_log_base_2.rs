// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::eq_expr_value;
use clippy_utils::source::snippet;
use rustc_hir::intravisit::{Visitor, walk_expr};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::Span;

declare_lint! {
    /// ### What it does
    ///
    /// Flags an `if x.is_power_of_2()` whose body computes `x.floor_log_base_2()` (or
    /// `x.floor_log_base_2_abs()`) on the same value.
    ///
    /// ### Why is this bad?
    ///
    /// `checked_log_base_2` does both at once: it returns `Some(log)` exactly when `x` is a power
    /// of two, so `if let Some(log) = x.checked_log_base_2()` replaces the guard and the
    /// (guaranteed-exact) floor-log with a single call and no separate power-of-two test.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if x.is_power_of_2() {
    ///     let e = x.floor_log_base_2_abs();
    ///     // ...
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if let Some(e) = x.checked_log_base_2() {
    ///     // ...
    /// }
    /// ```
    pub USE_CHECKED_LOG_BASE_2,
    Deny,
    "guarding a `floor_log_base_2` with `is_power_of_2` instead of using `checked_log_base_2`"
}

declare_lint_pass!(UseCheckedLogBase2 => [USE_CHECKED_LOG_BASE_2]);

// Whether `ty` (references peeled) has `checked_log_base_2`: a primitive integer, a `Natural`, or a
// `Rational` (`Integer` and `Float` do not implement it).
fn has_checked_log_base_2<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> bool {
    ty.is_integral() || matches!(crate::bignum_name(cx, ty), Some("Natural" | "Rational"))
}

// Collects the receivers of every `is_power_of_2()` call in an expression (e.g. the condition of an
// `if`), keeping only those whose type has `checked_log_base_2`.
struct IsPow2Receivers<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    receivers: Vec<&'tcx Expr<'tcx>>,
}

impl<'tcx> Visitor<'tcx> for IsPow2Receivers<'_, 'tcx> {
    fn visit_expr(&mut self, e: &'tcx Expr<'tcx>) {
        if let ExprKind::MethodCall(seg, recv, [], _) = e.kind
            && seg.ident.name.as_str() == "is_power_of_2"
            && has_checked_log_base_2(self.cx, self.cx.typeck_results().expr_ty(recv).peel_refs())
        {
            self.receivers.push(recv);
        }
        walk_expr(self, e);
    }
}

// Finds a `floor_log_base_2`/`floor_log_base_2_abs` call whose receiver equals one of `targets`.
struct FloorLogFinder<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    targets: &'a [&'tcx Expr<'tcx>],
    found: Option<Span>,
}

impl<'tcx> Visitor<'tcx> for FloorLogFinder<'_, 'tcx> {
    fn visit_expr(&mut self, e: &'tcx Expr<'tcx>) {
        if self.found.is_some() {
            return;
        }
        if let ExprKind::MethodCall(seg, recv, [], _) = e.kind
            && matches!(
                seg.ident.name.as_str(),
                "floor_log_base_2" | "floor_log_base_2_abs"
            )
            && self.targets.iter().any(|t| eq_expr_value(self.cx, t, recv))
        {
            self.found = Some(e.span);
            return;
        }
        walk_expr(self, e);
    }
}

impl<'tcx> LateLintPass<'tcx> for UseCheckedLogBase2 {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::If(cond, then, _) = expr.kind else {
            return;
        };
        let mut receivers = IsPow2Receivers {
            cx,
            receivers: Vec::new(),
        };
        receivers.visit_expr(cond);
        if receivers.receivers.is_empty() {
            return;
        }
        let mut finder = FloorLogFinder {
            cx,
            targets: &receivers.receivers,
            found: None,
        };
        finder.visit_expr(then);
        if let Some(span) = finder.found {
            span_lint_and_help(
                cx,
                USE_CHECKED_LOG_BASE_2,
                span,
                "computing `floor_log_base_2` under an `is_power_of_2` guard",
                None,
                format!(
                    "use `if let Some(e) = {}.checked_log_base_2()`, which tests and returns the \
                     exact log at once",
                    snippet(cx, receivers.receivers[0].span, ".."),
                ),
            );
        }
    }
}
