// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::path_to_local_with_projections;
use clippy_utils::usage::mutated_variables;
use rustc_hir::{BinOpKind, Expr, ExprKind, HirId, Node};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a `Natural` or `Integer` division -- `div_mod`, `div_rem`, `div_assign_mod`,
    /// `div_assign_rem`, `/`, or `%` -- inside a loop whose divisor is a local defined outside the
    /// loop and not mutated within it, where the fused
    /// [`DivModPrecomputed`](malachite_base::num::arithmetic::traits::DivModPrecomputed) exists.
    ///
    /// ### Why is this bad?
    ///
    /// Every division by a multi-limb divisor normalizes it and computes inverses -- for large
    /// divisors, the full Barrett inverse, which costs about as much as a multiplication of the
    /// divisor's size. `precompute_div_mod_data` computes all of that once, outside the loop, and
    /// `div_mod_precomputed` reuses it on every iteration.
    ///
    /// ### Known problems
    ///
    /// Primitive-integer divisions are deliberately not flagged: on processors with fast hardware
    /// dividers, the preinverted form can lose to plain division, so that rewrite is a judgment
    /// call. Loops that run very few times gain little; such sites can carry an `expect` with a
    /// justifying comment. Only divisors that are plain locals are recognized, and only `hir`
    /// loops -- divisions inside iterator closures are not seen.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// for x in xs {
    ///     let (q, r) = x.div_mod(&d);
    ///     // ...
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let data = Natural::precompute_div_mod_data(&d);
    /// for x in xs {
    ///     let (q, r) = x.div_mod_precomputed(&d, &data);
    ///     // ...
    /// }
    /// ```
    pub USE_DIV_MOD_PRECOMPUTED,
    Deny,
    "dividing by a loop-invariant bignum divisor without precomputed division data"
}

declare_lint_pass!(UseDivModPrecomputed => [USE_DIV_MOD_PRECOMPUTED]);

// The method names with a `div_mod_precomputed` counterpart. `/` and `%` are handled separately.
const DIVISION_METHODS: [&str; 4] = ["div_mod", "div_rem", "div_assign_mod", "div_assign_rem"];

// Whether `expr` sits inside the implementation of the operation itself.
fn inside_own_definition<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'tcx>) -> bool {
    let did = cx.tcx.hir_get_parent_item(expr.hir_id).def_id;
    cx.tcx
        .item_name(did.to_def_id())
        .as_str()
        .contains("div_mod_precomputed")
}

// The innermost loop containing `expr`, not crossing a closure or item boundary.
fn enclosing_loop<'tcx>(cx: &LateContext<'tcx>, id: HirId) -> Option<&'tcx Expr<'tcx>> {
    for (_, node) in cx.tcx.hir_parent_iter(id) {
        match node {
            Node::Expr(e) => match e.kind {
                ExprKind::Loop(..) => return Some(e),
                ExprKind::Closure(_) => return None,
                _ => {}
            },
            Node::Item(_) | Node::ImplItem(_) | Node::TraitItem(_) => return None,
            _ => {}
        }
    }
    None
}

impl<'tcx> LateLintPass<'tcx> for UseDivModPrecomputed {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion()
            || crate::in_test_code(cx, expr.span)
            || inside_own_definition(cx, expr)
        {
            return;
        }
        let divisor = match expr.kind {
            ExprKind::MethodCall(seg, _, [d], _)
                if DIVISION_METHODS.contains(&seg.ident.name.as_str()) =>
            {
                d
            }
            ExprKind::Binary(op, _, d) if matches!(op.node, BinOpKind::Div | BinOpKind::Rem) => d,
            _ => return,
        };
        let divisor = crate::peel_clone_and_borrows(divisor);
        // Only `Natural` and `Integer` divisors have `DivModPrecomputed`; for primitives the
        // preinverted form is not a clear win, and `Rational` division is multiplication by the
        // reciprocal.
        if !matches!(
            crate::bignum_name(cx, cx.typeck_results().expr_ty(divisor).peel_refs()),
            Some("Natural" | "Integer")
        ) {
            return;
        }
        let Some(local) = path_to_local_with_projections(divisor) else {
            return;
        };
        let Some(enclosing) = enclosing_loop(cx, expr.hir_id) else {
            return;
        };
        // The divisor must be defined outside the loop...
        if enclosing.span.contains(cx.tcx.hir_span(local)) {
            return;
        }
        // ...and not mutated inside it. `mutated_variables` returns `None` when it cannot tell;
        // treat that conservatively as mutated.
        let Some(mutated) = mutated_variables(enclosing, cx) else {
            return;
        };
        if mutated.contains(&local) {
            return;
        }
        clippy_utils::diagnostics::span_lint(
            cx,
            USE_DIV_MOD_PRECOMPUTED,
            expr.span,
            "this division's divisor is loop-invariant; call `precompute_div_mod_data()` outside \
            the loop and use `div_mod_precomputed()` here",
        );
    }
}
