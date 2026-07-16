// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::eq_expr_value;
use clippy_utils::source::snippet;
use rustc_ast::Mutability;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::intravisit::{Visitor, walk_expr};
use rustc_hir::{
    BindingMode, Block, ByRef, Expr, ExprKind, HirId, Node, PatKind, QPath, Stmt, StmtKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags calling a `*_prec_round` or `*_prec` method on a [`Float`] local whose binding
    /// visibly pins its precision to the very same precision expression, like
    ///
    /// ```rust,ignore
    /// let arg = x.mul_prec_round(y, working_prec, Floor).0;
    /// let lo = arg.exp_prec_round(working_prec, Floor).0;
    /// ```
    ///
    /// ### Why is this bad?
    ///
    /// The receiver already has the requested precision, so re-specifying it is redundant; the
    /// `*_round` variant computes at the receiver's own precision and says so:
    ///
    /// ```rust,ignore
    /// let lo = arg.exp_round(Floor).0;
    /// ```
    ///
    /// The lint only fires when this is provable locally: the receiver -- and every other `Float`
    /// operand, since the `*_round` variants compute at the maximum of their `Float` operands'
    /// precisions -- is an immutable local bound by a `let` whose initializer is itself a
    /// `*_prec_round`/`*_prec` call (possibly through the precision-preserving
    /// `floor_and_ceiling`), all precision expressions are syntactically identical and refer to
    /// the same bindings, and no local mentioned in the precision expression is reassigned between
    /// the bindings and the use.
    pub USE_ROUND_VARIANT,
    Deny,
    "re-specifying a precision the receiver is already known to have; use the `*_round` variant"
}

declare_lint_pass!(UseRoundVariant => [USE_ROUND_VARIANT]);

// If `base` (a method name with `_val`/`_ref` suffixes already stripped) is a
// precision-specifying family member, returns whether it takes a rounding mode and the family
// stem, e.g. `mul_prec_round` -> (true, "mul") and `exp_prec` -> (false, "exp").
fn prec_family(base: &str) -> Option<(bool, &str)> {
    if let Some(stem) = base.strip_suffix("_prec_round") {
        Some((true, stem))
    } else if let Some(stem) = base.strip_suffix("_prec") {
        Some((false, stem))
    } else {
        None
    }
}

// The precision argument of a call in a precision-specifying family: the second-to-last argument
// when a rounding mode follows, and the last otherwise.
fn prec_arg<'tcx>(has_rm: bool, args: &'tcx [Expr<'tcx>]) -> Option<&'tcx Expr<'tcx>> {
    if has_rm {
        args.len().checked_sub(2).map(|i| &args[i])
    } else {
        args.last()
    }
}

// If `e` is a `*_prec_round`/`*_prec` call whose result carries the requested precision -- either
// a method call or a `Float::...` associated-function call -- returns its precision argument.
fn prec_producing_call<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
) -> Option<&'tcx Expr<'tcx>> {
    match e.kind {
        ExprKind::MethodCall(seg, _, args, _) => {
            let base = crate::strip_variant_suffixes(seg.ident.name.as_str());
            let (has_rm, _) = prec_family(base)?;
            prec_arg(has_rm, args)
        }
        ExprKind::Call(callee, args) => {
            let ExprKind::Path(ref qpath) = callee.kind else {
                return None;
            };
            let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
                return None;
            };
            let name = cx.tcx.item_name(fn_did);
            let base = crate::strip_variant_suffixes(name.as_str());
            let (has_rm, _) = prec_family(base)?;
            prec_arg(has_rm, args)
        }
        _ => None,
    }
}

// If `e` is a call to the house `floor_and_ceiling` helper (which returns a pair of `Float`s
// bracketing an exact result, both with the precision of the rounded input), returns its argument.
fn floor_and_ceiling_inner<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::Call(callee, [arg]) = e.kind else {
        return None;
    };
    let ExprKind::Path(ref qpath) = callee.kind else {
        return None;
    };
    let Res::Def(DefKind::Fn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
        return None;
    };
    if cx.tcx.item_name(fn_did).as_str() != "floor_and_ceiling" {
        return None;
    }
    Some(arg)
}

// The precision expression that `init` visibly pins its result to, if any. `elem` is the tuple
// element being bound (`None` for a direct binding, where a `.0`/`.1` projection may appear in
// the initializer instead). Ordinary `*_prec*` calls pin only their first element (the value);
// `floor_and_ceiling` pins both elements of its output to its input's precision.
fn pinning_prec<'tcx>(
    cx: &LateContext<'tcx>,
    init: &'tcx Expr<'tcx>,
    elem: Option<usize>,
) -> Option<&'tcx Expr<'tcx>> {
    let (core, pos) = match (elem, init.kind) {
        (None, ExprKind::Field(base, ident)) if ident.as_str() == "0" => (base, 0),
        (None, ExprKind::Field(base, ident)) if ident.as_str() == "1" => (base, 1),
        (None, _) => (init, 0),
        (Some(i), _) => (init, i),
    };
    if let Some(inner) = floor_and_ceiling_inner(cx, core) {
        return prec_producing_call(cx, inner);
    }
    if pos != 0 {
        return None;
    }
    prec_producing_call(cx, core)
}

// Whether a plain (round-to-`Nearest`, value-only) variant of the family `stem` exists: either an
// inherent method or a `malachite_base` trait like `Ln` or `Exp`.
fn plain_variant_exists(
    cx: &LateContext<'_>,
    adt_did: rustc_hir::def_id::DefId,
    stem: &str,
) -> bool {
    if crate::has_inherent_fn(cx, adt_did, stem) {
        return true;
    }
    let trait_name = crate::camel_case(stem);
    for module in ["arithmetic", "logic"] {
        let path = format!("malachite_base::num::{module}::traits::{trait_name}");
        if !clippy_utils::paths::lookup_path_str(cx.tcx, clippy_utils::paths::PathNS::Type, &path)
            .is_empty()
        {
            return true;
        }
    }
    false
}

// Collects the `HirId`s of all locals referenced within an expression.
struct LocalCollector(Vec<HirId>);

impl<'tcx> Visitor<'tcx> for LocalCollector {
    fn visit_expr(&mut self, e: &'tcx Expr<'tcx>) {
        if let ExprKind::Path(QPath::Resolved(None, path)) = e.kind
            && let Res::Local(id) = path.res
        {
            self.0.push(id);
        }
        walk_expr(self, e);
    }
}

fn collect_locals(e: &Expr<'_>) -> Vec<HirId> {
    let mut collector = LocalCollector(Vec::new());
    collector.visit_expr(e);
    let mut ids = collector.0;
    ids.sort_by_key(|id| {
        (
            id.owner.def_id.local_def_index.as_u32(),
            id.local_id.as_u32(),
        )
    });
    ids.dedup();
    ids
}

// Detects reassignment of any watched local: direct or destructuring assignment, compound
// assignment, or a mutable borrow (which could feed a later write).
struct MutationFinder<'a> {
    watched: &'a [HirId],
    found: bool,
}

impl<'tcx> Visitor<'tcx> for MutationFinder<'_> {
    fn visit_expr(&mut self, e: &'tcx Expr<'tcx>) {
        if self.found {
            return;
        }
        match e.kind {
            ExprKind::Assign(lhs, ..) | ExprKind::AssignOp(_, lhs, _) => {
                if collect_locals(lhs)
                    .iter()
                    .any(|id| self.watched.contains(id))
                {
                    self.found = true;
                    return;
                }
            }
            ExprKind::AddrOf(_, Mutability::Mut, inner) => {
                if collect_locals(inner)
                    .iter()
                    .any(|id| self.watched.contains(id))
                {
                    self.found = true;
                    return;
                }
            }
            _ => {}
        }
        walk_expr(self, e);
    }
}

fn mutates_watched(stmt: &Stmt<'_>, watched: &[HirId]) -> bool {
    let mut finder = MutationFinder {
        watched,
        found: false,
    };
    match stmt.kind {
        StmtKind::Let(l) => {
            if let Some(init) = l.init {
                finder.visit_expr(init);
            }
            if let Some(els) = l.els {
                finder.visit_block(els);
            }
        }
        StmtKind::Expr(e) | StmtKind::Semi(e) => finder.visit_expr(e),
        StmtKind::Item(_) => {}
    }
    finder.found
}

// Whether `local_expr` is a path to an immutable local whose `let` binding visibly pins its
// precision to an expression identical to `prec`, with no local mentioned in `prec` reassigned
// between the binding and `use_expr`.
fn local_pinned_at<'tcx>(
    cx: &LateContext<'tcx>,
    local_expr: &'tcx Expr<'tcx>,
    use_expr: &'tcx Expr<'tcx>,
    prec: &'tcx Expr<'tcx>,
) -> bool {
    let ExprKind::Path(QPath::Resolved(None, path)) = local_expr.kind else {
        return false;
    };
    let Res::Local(local_id) = path.res else {
        return false;
    };
    // The binding must be immutable, so that the value (and hence its precision) cannot have
    // changed since initialization.
    let Node::Pat(pat) = cx.tcx.hir_node(local_id) else {
        return false;
    };
    let PatKind::Binding(BindingMode(ByRef::No, Mutability::Not), _, _, None) = pat.kind else {
        return false;
    };
    // Find the enclosing `let` statement, allowing the binding to sit inside a tuple pattern (the
    // house `let (x, o) = ...` and `let (lo, hi) = floor_and_ceiling(...)` shapes).
    let mut let_stmt = None;
    let mut let_stmt_id = None;
    for (pid, node) in cx.tcx.hir_parent_iter(local_id) {
        match node {
            Node::Pat(_) => {}
            Node::LetStmt(l) => {
                let_stmt = Some(l);
                let_stmt_id = Some(pid);
                break;
            }
            _ => return false,
        }
    }
    let (Some(l), Some(l_id)) = (let_stmt, let_stmt_id) else {
        return false;
    };
    let Some(init) = l.init else {
        return false;
    };
    // Which value is being bound: the whole initializer, or a direct tuple element of it.
    let elem = if l.pat.hir_id == pat.hir_id {
        None
    } else if let PatKind::Tuple(elems, _) = l.pat.kind
        && let Some(i) = elems.iter().position(|p| p.hir_id == pat.hir_id)
    {
        Some(i)
    } else {
        return false;
    };
    let Some(init_prec) = pinning_prec(cx, init, elem) else {
        return false;
    };
    // Both precision expressions must be syntactically identical and refer to the same bindings.
    if !eq_expr_value(cx, prec, init_prec) {
        return false;
    }
    let watched = collect_locals(prec);
    if watched != collect_locals(init_prec) {
        return false;
    }
    // No local mentioned in the precision expression may be reassigned between the binding and
    // the use. Scan the statements of the shared block from just after the `let` through the
    // statement containing the use (inclusive: if the use sits inside a loop, a mutation
    // anywhere in that loop could precede it on a later iteration).
    if !watched.is_empty() {
        let mut block: Option<&Block<'tcx>> = None;
        for (_, node) in cx.tcx.hir_parent_iter(l_id) {
            if let Node::Block(b) = node {
                block = Some(b);
                break;
            }
        }
        let Some(block) = block else {
            return false;
        };
        let Some(let_index) = block.stmts.iter().position(|s| {
            if let StmtKind::Let(sl) = s.kind {
                sl.hir_id == l.hir_id
            } else {
                false
            }
        }) else {
            return false;
        };
        // Locate the statement of this block containing the use, if any; otherwise the use is in
        // the block's tail expression.
        let mut use_index = None;
        for (pid, node) in cx.tcx.hir_parent_iter(use_expr.hir_id) {
            if let Node::Stmt(_) = node
                && let Some(i) = block.stmts.iter().position(|s| s.hir_id == pid)
            {
                use_index = Some(i);
                break;
            }
            if let Node::Block(b) = node
                && b.hir_id == block.hir_id
            {
                break;
            }
        }
        let scan_end = use_index.unwrap_or(block.stmts.len().saturating_sub(1));
        if scan_end < let_index {
            // The use is not downstream of the binding in this block (e.g. a different
            // control-flow shape); be conservative.
            return false;
        }
        for stmt in &block.stmts[let_index + 1..=scan_end] {
            if mutates_watched(stmt, &watched) {
                return false;
            }
        }
        if use_index.is_none() {
            let mut finder = MutationFinder {
                watched: &watched,
                found: false,
            };
            if let Some(tail) = block.expr {
                finder.visit_expr(tail);
            }
            if finder.found {
                return false;
            }
        }
    }
    true
}

impl<'tcx> LateLintPass<'tcx> for UseRoundVariant {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::MethodCall(seg, recv, args, _) = expr.kind else {
            return;
        };
        let name = seg.ident.name.as_str();
        let base = crate::strip_variant_suffixes(name);
        let Some((has_rm, stem)) = prec_family(base) else {
            return;
        };
        let Some(outer_prec) = prec_arg(has_rm, args) else {
            return;
        };
        // The receiver must be a `Float` local pinned at the requested precision, and so must
        // every other `Float` operand: the `*_round` variants compute at the maximum of their
        // `Float` operands' precisions, so knowing the receiver's precision alone is not enough
        // to drop the explicit one.
        let prec_index = if has_rm {
            args.len() - 2
        } else {
            args.len() - 1
        };
        let recv_peeled = crate::peel_clone_and_borrows(recv);
        let recv_ty = cx.typeck_results().expr_ty(recv_peeled).peel_refs();
        if crate::bignum_name(cx, recv_ty) != Some("Float") {
            return;
        }
        if !local_pinned_at(cx, recv_peeled, expr, outer_prec) {
            return;
        }
        for arg in &args[..prec_index] {
            let arg_peeled = crate::peel_clone_and_borrows(arg);
            if crate::bignum_name(cx, cx.typeck_results().expr_ty(arg_peeled).peel_refs())
                == Some("Float")
                && !local_pinned_at(cx, arg_peeled, expr, outer_prec)
            {
                return;
            }
        }
        // The suggested variant must exist.
        let Some(adt_did) = crate::bignum_adt_did(cx, recv_ty) else {
            return;
        };
        let round_variant = format!("{stem}_round");
        if !crate::has_inherent_fn(cx, adt_did, &round_variant) {
            return;
        }
        // For the `Nearest` shorthand with the `Ordering` immediately discarded (`.0`), the plain
        // method is cleaner still, when one exists (inherent, or as a `malachite_base` trait).
        let ordering_discarded = matches!(
            cx.tcx.parent_hir_node(expr.hir_id),
            Node::Expr(Expr {
                kind: ExprKind::Field(_, ident),
                ..
            }) if ident.as_str() == "0"
        );
        let leading = args[..prec_index]
            .iter()
            .map(|a| snippet(cx, a.span, "..").to_string())
            .collect::<Vec<_>>();
        let advice = if has_rm {
            let mut parts = leading;
            parts.push(snippet(cx, args[args.len() - 1].span, "..").to_string());
            // Replace within the full original name so `_val`/`_ref` suffixes survive:
            // `div_prec_round_ref_val` suggests `div_round_ref_val`.
            format!(
                "use `{}({})` instead of `{name}`",
                name.replace("_prec_round", "_round"),
                parts.join(", ")
            )
        } else if ordering_discarded && plain_variant_exists(cx, adt_did, stem) {
            format!(
                "use plain `{stem}({})` (dropping the `.0`) instead of `{name}`",
                leading.join(", ")
            )
        } else {
            let mut parts = leading;
            parts.push("Nearest".to_string());
            format!(
                "use `{round_variant}({})` instead of `{name}`",
                parts.join(", ")
            )
        };
        span_lint(
            cx,
            USE_ROUND_VARIANT,
            expr.span,
            format!(
                "{advice}: `{}` already has precision `{}`",
                snippet(cx, recv_peeled.span, ".."),
                snippet(cx, outer_prec.span, ".."),
            ),
        );
    }
}
