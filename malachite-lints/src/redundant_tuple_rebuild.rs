// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::res::MaybeResPath;
use clippy_utils::source::snippet_opt;
use clippy_utils::visitors::for_each_expr;
use core::ops::ControlFlow;
use rustc_hir::{Expr, ExprKind, HirId, Node, PatKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags taking a tuple apart in a pattern and then putting the same tuple back together, as
    /// in `|&((n_2, n_1), (d_1, d_0))| (n_2, n_1) < (d_1, d_0)`, where `(n_2, n_1)` and
    /// `(d_1, d_0)` are rebuilt exactly as they were destructured.
    ///
    /// ### Why is this bad?
    ///
    /// The names introduced by the pattern are never used for anything but the reassembly, so they
    /// are pure overhead: the reader has to check, element by element, that the tuple really is
    /// being rebuilt in the original order rather than transposed. Binding the tuple to one name
    /// says the same thing and removes the chance of that mistake.
    ///
    /// The lint only fires when *every* name from the pattern is used exactly once, inside the
    /// rebuilt tuple. If any of them is used on its own as well, the destructuring is earning its
    /// keep and nothing is reported.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// xs.filter(|&((n_2, n_1), (d_1, d_0))| (n_2, n_1) < (d_1, d_0))
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// xs.filter(|&(n, d)| n < d)
    /// ```
    pub REDUNDANT_TUPLE_REBUILD,
    Deny,
    "destructuring a tuple in a pattern and rebuilding the same tuple in the body"
}

declare_lint_pass!(RedundantTupleRebuild => [REDUNDANT_TUPLE_REBUILD]);

// How many times `id` is referred to in the body enclosing `expr`.
fn use_count(cx: &LateContext<'_>, expr: &Expr<'_>, id: HirId) -> usize {
    let owner = cx.tcx.hir_enclosing_body_owner(expr.hir_id);
    let body = cx.tcx.hir_body_owned_by(owner);
    let mut count = 0;
    for_each_expr(cx, body.value, |e| {
        if e.res_local_id() == Some(id) {
            count += 1;
        }
        ControlFlow::<()>::Continue(())
    });
    count
}

impl<'tcx> LateLintPass<'tcx> for RedundantTupleRebuild {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        let ExprKind::Tup(elements) = expr.kind else {
            return;
        };
        if elements.len() < 2 {
            return;
        }
        // Every element must be a bare mention of a binding.
        let mut ids = Vec::with_capacity(elements.len());
        for element in elements {
            let Some(id) = element.res_local_id() else {
                return;
            };
            ids.push(id);
        }
        // Those bindings must be the whole of one tuple pattern, in the order it wrote them.
        let Node::Pat(pattern) = cx.tcx.parent_hir_node(ids[0]) else {
            return;
        };
        let PatKind::Tuple(sub_patterns, dots) = pattern.kind else {
            return;
        };
        // A `..` means the pattern skips elements, so the tuple is not being rebuilt whole.
        if dots.as_opt_usize().is_some() || sub_patterns.len() != elements.len() {
            return;
        }
        if sub_patterns
            .iter()
            .zip(&ids)
            .any(|(sub, &id)| sub.hir_id != id)
        {
            return;
        }
        // If a name is used anywhere else, the pattern is doing real work.
        if ids.iter().any(|&id| use_count(cx, expr, id) != 1) {
            return;
        }
        let pattern = snippet_opt(cx, pattern.span).map_or_else(
            || "the pattern".to_string(),
            |pattern| format!("`{pattern}`"),
        );
        span_lint_and_help(
            cx,
            REDUNDANT_TUPLE_REBUILD,
            expr.span,
            format!("this rebuilds the tuple that {pattern} took apart"),
            None,
            "bind the tuple to one name instead of naming its elements and putting them back \
             together",
        );
    }
}
