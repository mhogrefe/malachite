// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::source::snippet_opt;
use rustc_errors::Applicability;
use rustc_hir::{BindingMode, ByRef, ExprKind, Mutability, PatKind, Stmt, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a `let` that destructures a tuple only to keep one field and discard the rest with
    /// `_`, like `let (x, _) = f();` or `let (_, o) = f();`, suggesting direct field access
    /// (`let x = f().0;`).
    ///
    /// ### Why is this bad?
    ///
    /// Malachite functions pervasively return `(value, Ordering)` tuples, and callers that only
    /// want one field write the tuple-and-wildcard form out of habit. Direct field access is
    /// shorter and, more importantly, leaves the initializer as a plain expression that can be
    /// chained (`f().0.g().0`) instead of forcing an intermediate binding — the idiomatic
    /// Malachite style over GMP/FLINT's assembly-like bind-mutate-rebind.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let (t, _) = x.exp_prec(p);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let t = x.exp_prec(p).0;
    /// ```
    pub LET_TUPLE_UNDERSCORE_TO_FIELD,
    Deny,
    "destructuring a tuple in a `let` only to discard all fields but one"
}

declare_lint_pass!(LetTupleUnderscoreToField => [LET_TUPLE_UNDERSCORE_TO_FIELD]);

impl<'tcx> LateLintPass<'tcx> for LetTupleUnderscoreToField {
    fn check_stmt(&mut self, cx: &LateContext<'tcx>, stmt: &'tcx Stmt<'tcx>) {
        if stmt.span.from_expansion() || crate::in_test_code(cx, stmt.span) {
            return;
        }
        let StmtKind::Let(local) = stmt.kind else {
            return;
        };
        // `let ... else` reruns the initializer's divergence; leave it alone.
        if local.els.is_some() {
            return;
        }
        let Some(init) = local.init else {
            return;
        };
        let PatKind::Tuple(pats, dot_dot) = local.pat.kind else {
            return;
        };
        // A `..` rest pattern already elides fields; nothing to suggest.
        if dot_dot.as_opt_usize().is_some() {
            return;
        }
        // Exactly one element is a plain by-value binding; every other is a top-level wildcard.
        // Two or more bindings means both fields are genuinely used; a `ref`/`mut ref` binding or
        // a subpattern is not a simple rename, so bail in those cases.
        let mut binding = None;
        for (i, pat) in pats.iter().enumerate() {
            match pat.kind {
                PatKind::Wild => {}
                PatKind::Binding(BindingMode(ByRef::No, mutbl), _, ident, None) => {
                    if binding.is_some() {
                        return;
                    }
                    binding = Some((i, mutbl, ident));
                }
                _ => return,
            }
        }
        let Some((idx, mutbl, ident)) = binding else {
            return;
        };
        let Some(init_snip) = snippet_opt(cx, init.span) else {
            return;
        };
        // Field access binds tighter than most expression forms, so anything that is not already
        // a postfix expression must be parenthesized to keep the suggestion valid.
        let needs_parens = !matches!(
            init.kind,
            ExprKind::MethodCall(..)
                | ExprKind::Call(..)
                | ExprKind::Path(..)
                | ExprKind::Field(..)
                | ExprKind::Index(..)
        );
        let base = if needs_parens {
            format!("({init_snip})")
        } else {
            init_snip
        };
        let mut_kw = if matches!(mutbl, Mutability::Mut) {
            "mut "
        } else {
            ""
        };
        span_lint_and_sugg(
            cx,
            LET_TUPLE_UNDERSCORE_TO_FIELD,
            stmt.span,
            "destructuring a tuple only to discard all fields but one",
            "access the field directly",
            format!("let {mut_kw}{ident} = {base}.{idx};"),
            Applicability::MachineApplicable,
        );
    }
}
