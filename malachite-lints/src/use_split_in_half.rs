// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_note;
use rustc_hir::def::Res;
use rustc_hir::intravisit::{Visitor, walk_expr};
use rustc_hir::{BindingMode, Body, Expr, ExprKind, HirId, Mutability, Node, PatKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::Span;
use std::collections::HashMap;

declare_lint! {
    /// ### What it does
    ///
    /// Flags calling both `upper_half()` and `lower_half()` on the same immutable local variable
    /// within one function body.
    ///
    /// ### Why is this bad?
    ///
    /// `split_in_half()` produces both halves in one call, so the pair of calls is better spelled
    /// as a single destructuring.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let hi = t.upper_half();
    /// let lo = t.lower_half();
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let (hi, lo) = t.split_in_half();
    /// ```
    pub USE_SPLIT_IN_HALF,
    Deny,
    "calling both `upper_half()` and `lower_half()` on the same value instead of `split_in_half()`"
}

declare_lint_pass!(UseSplitInHalf => [USE_SPLIT_IN_HALF]);

// Whether the method call `e` resolves to `malachite_base::num::conversion::traits::SplitInHalf`,
// rather than to some unrelated method that happens to share a name.
fn is_split_in_half_method(cx: &LateContext<'_>, e: &Expr<'_>) -> bool {
    let Some(fn_did) = cx.typeck_results().type_dependent_def_id(e.hir_id) else {
        return false;
    };
    let Some(trait_did) = cx.tcx.trait_of_assoc(fn_did) else {
        return false;
    };
    let path = cx.get_def_path(trait_did);
    path.len() == 5
        && path[0].as_str() == "malachite_base"
        && path[1].as_str() == "num"
        && path[2].as_str() == "conversion"
        && path[3].as_str() == "traits"
        && path[4].as_str() == "SplitInHalf"
}

// If `e`'s receiver (behind `&`) is an immutable local of non-mutable-reference type, whose value
// therefore cannot change between two reads, returns the local's `HirId`.
fn stable_local_receiver(cx: &LateContext<'_>, recv: &Expr<'_>) -> Option<HirId> {
    let recv = recv.peel_borrows();
    let ExprKind::Path(qpath) = &recv.kind else {
        return None;
    };
    let Res::Local(hir_id) = cx.qpath_res(qpath, recv.hir_id) else {
        return None;
    };
    let Node::Pat(pat) = cx.tcx.hir_node(hir_id) else {
        return None;
    };
    let PatKind::Binding(BindingMode(_, Mutability::Not), ..) = pat.kind else {
        return None;
    };
    // A `&mut` local is immutable itself, but the value it points at is not.
    let mut ty = cx.typeck_results().expr_ty(recv);
    while let rustc_middle::ty::Ref(_, inner, mutability) = ty.kind() {
        if *mutability == Mutability::Mut {
            return None;
        }
        ty = *inner;
    }
    Some(hir_id)
}

// The first `upper_half()` and `lower_half()` call spans seen for each local.
#[derive(Default)]
struct HalfCalls {
    upper: Option<Span>,
    lower: Option<Span>,
}

struct HalfCallFinder<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    calls: HashMap<HirId, HalfCalls>,
}

impl<'tcx> Visitor<'tcx> for HalfCallFinder<'_, 'tcx> {
    fn visit_expr(&mut self, e: &'tcx Expr<'tcx>) {
        if !e.span.from_expansion()
            && let ExprKind::MethodCall(seg, recv, [], _) = e.kind
        {
            let name = seg.ident.name.as_str();
            if matches!(name, "upper_half" | "lower_half")
                && is_split_in_half_method(self.cx, e)
                && let Some(local) = stable_local_receiver(self.cx, recv)
            {
                let calls = self.calls.entry(local).or_default();
                let slot = if name == "upper_half" {
                    &mut calls.upper
                } else {
                    &mut calls.lower
                };
                slot.get_or_insert(e.span);
            }
        }
        walk_expr(self, e);
    }
}

impl<'tcx> LateLintPass<'tcx> for UseSplitInHalf {
    fn check_body(&mut self, cx: &LateContext<'tcx>, body: &Body<'tcx>) {
        // Nested bodies are not walked; each closure gets its own `check_body` call.
        let mut finder = HalfCallFinder {
            cx,
            calls: HashMap::new(),
        };
        finder.visit_expr(body.value);
        let mut pairs: Vec<(Span, Span)> = finder
            .calls
            .into_values()
            .filter_map(|calls| Some((calls.upper?, calls.lower?)))
            .collect();
        // HashMap iteration order is arbitrary; report in source order.
        pairs.sort_by_key(|(upper, lower)| upper.lo().min(lower.lo()));
        for (upper, lower) in pairs {
            let (first, second) = if upper.lo() <= lower.lo() {
                (upper, lower)
            } else {
                (lower, upper)
            };
            // in_test_code is comparatively expensive, so it runs after the structural checks.
            if crate::in_test_code(cx, first) {
                continue;
            }
            span_lint_and_note(
                cx,
                USE_SPLIT_IN_HALF,
                second,
                "use `split_in_half()` instead of taking `upper_half()` and `lower_half()` \
                separately",
                Some(first),
                "the other half of the same value is taken here",
            );
        }
    }
}
