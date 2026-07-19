// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::consts::ConstEvalCtxt;
use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::source::snippet;
use clippy_utils::visitors::for_each_expr;
use clippy_utils::{get_parent_expr, path_to_local_with_projections};
use core::ops::ControlFlow;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a *derived* compile-time constant that appears as a subexpression of a larger runtime
    /// expression — arithmetic, a unary operation, a comparison, or a cast built from named
    /// constants, such as `RND_BIT + 1` in `msl >> (RND_BIT + 1)` or `Limb::ONE << RND_BIT` in
    /// `msl & (Limb::ONE << RND_BIT)`. Suggests folding it to its value or wrapping it in a
    /// `const { .. }` block.
    ///
    /// ### Why is this bad?
    ///
    /// A constant island buried in a runtime expression reads as if it were computed each time. A
    /// `const { .. }` block (or the folded literal) makes the compile-time evaluation explicit and
    /// guaranteed, and — like `use_const_binding` for a whole binding — lets the reader see at a
    /// glance which part of the expression does not depend on runtime state.
    ///
    /// Only the *maximal* constant subexpression is flagged (the largest one whose enclosing
    /// expression is not itself a constructible constant), and only when it is *derived* — built
    /// from at least one named constant, not a bare literal computation the compiler folds anyway.
    /// A subexpression is left alone if it does not evaluate at compile time, or if it mentions a
    /// local — including a `bool` expression that short-circuiting makes constant-*valued* while it
    /// still names a runtime operand (`SOME_CONST && n < THRESHOLD`), which could not be lifted into
    /// a `const { .. }` block. Anything already inside a `const { .. }` block or a const context is
    /// likewise skipped, and a fully-constant `let` initializer is `use_const_binding`'s job, not
    /// this lint's.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let mut digit = u8::exact_from(msl >> (RND_BIT + 1));
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let mut digit = u8::exact_from(msl >> const { RND_BIT + 1 });
    /// ```
    pub USE_CONST_BLOCK,
    Deny,
    "a derived compile-time constant subexpression that should be folded or wrapped in `const { .. }`"
}

declare_lint_pass!(UseConstBlock => [USE_CONST_BLOCK]);

// Whether `e` is a *computed* expression: arithmetic, a unary op, or a cast. A bare literal or a
// bare path (including a path to a `const`) is already atomic and needs no wrapping.
fn is_computed(e: &Expr<'_>) -> bool {
    matches!(
        e.kind,
        ExprKind::Binary(..) | ExprKind::Unary(..) | ExprKind::Cast(..)
    )
}

// Whether `e` mentions any local binding. Such an expression cannot be lifted into a `const { .. }`
// block, even if constant folding (e.g. short-circuiting) made it constant-*valued*.
fn references_local<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> bool {
    for_each_expr(cx, e, |sub| {
        if path_to_local_with_projections(sub).is_some() {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    })
    .is_some()
}

impl<'tcx> LateLintPass<'tcx> for UseConstBlock {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        // A computed expression, derived from at least one named constant, outside any const
        // context (a `const`/`static`/`const fn`, or an existing `const { .. }` block).
        if !is_computed(expr)
            || !crate::references_named_const(cx, expr)
            || crate::in_const_context(cx, expr)
        {
            return;
        }
        // The subexpression itself is a compile-time constant that mentions no local (so it can
        // actually be lifted into a `const { .. }`), but its enclosing expression is not such a
        // constant — so this is the maximal constant island. (When the parent is also a
        // constructible constant, the parent is flagged instead; when there is no enclosing
        // expression — a fully-constant `let` initializer, say — that is `use_const_binding`'s.)
        let cx_eval = ConstEvalCtxt::new(cx);
        if cx_eval.eval(expr).is_none() || references_local(cx, expr) {
            return;
        }
        let Some(parent) = get_parent_expr(cx, expr) else {
            return;
        };
        if cx_eval.eval(parent).is_some() && !references_local(cx, parent) {
            return;
        }
        span_lint_and_help(
            cx,
            USE_CONST_BLOCK,
            expr.span,
            "this subexpression is a compile-time constant embedded in a runtime expression",
            None,
            format!(
                "fold it to its value, or wrap it: `const {{ {} }}`",
                snippet(cx, expr.span, "..")
            ),
        );
    }
}
