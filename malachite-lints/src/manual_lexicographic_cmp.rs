// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::eq_expr_value;
use clippy_utils::source::snippet_opt;
use rustc_hir::{BinOpKind, Expr, ExprKind, Node, UnOp};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags comparing two sequences of values lexicographically by hand, like
    /// `a_1 < b_1 || a_1 == b_1 && a_0 < b_0`, when comparing tuples says the same thing:
    /// `(a_1, a_0) < (b_1, b_0)`. Covers `<`, `<=`, `>`, and `>=`, and chains of any length.
    ///
    /// ### Why is this bad?
    ///
    /// The expanded form repeats each operand two or three times, and the repetition is where the
    /// mistakes live: comparing the wrong pair, using the strict operator in the last position, or
    /// getting the nesting wrong when the chain grows past two elements. Tuple comparison is
    /// lexicographic already, names each operand once, and extends to more elements by adding one
    /// name per side.
    ///
    /// This is a rewrite of the *whole* comparison, so it only fires when every operand is a plain
    /// place expression — a path, field, index, or literal. Operands that could have side effects
    /// or be expensive are left alone, since tuple comparison evaluates all of them while the
    /// expanded form short-circuits.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if p_1 < q_1 || p_1 == q_1 && p_0 < q_0 { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if (p_1, p_0) < (q_1, q_0) { .. }
    /// ```
    pub MANUAL_LEXICOGRAPHIC_CMP,
    Deny,
    "comparing sequences lexicographically by hand instead of comparing tuples"
}

declare_lint_pass!(ManualLexicographicCmp => [MANUAL_LEXICOGRAPHIC_CMP]);

// The strict form of an ordering operator: the one the earlier elements are compared with. Only the
// last element keeps the original operator, which is what makes `<=` and `>=` work.
fn strict(op: BinOpKind) -> Option<BinOpKind> {
    match op {
        BinOpKind::Lt | BinOpKind::Le => Some(BinOpKind::Lt),
        BinOpKind::Gt | BinOpKind::Ge => Some(BinOpKind::Gt),
        _ => None,
    }
}

// Whether `e` is a plain place expression, so that naming it inside a tuple neither repeats work
// nor changes what runs. Tuple comparison evaluates every element, unlike the short-circuiting form
// it replaces.
fn is_place(e: &Expr<'_>) -> bool {
    match e.peel_borrows().kind {
        ExprKind::Path(_) | ExprKind::Lit(_) => true,
        ExprKind::Field(base, _) => is_place(base),
        ExprKind::Index(base, index, _) => is_place(base) && is_place(index),
        ExprKind::Unary(UnOp::Deref | UnOp::Neg, inner) | ExprKind::Cast(inner, _) => {
            is_place(inner)
        }
        _ => false,
    }
}

// Splits `e` into the operands of a chain of `op`, which is left-associated, so `a || b || c` is
// `((a || b) || c)`.
fn flatten<'tcx>(e: &'tcx Expr<'tcx>, op: BinOpKind, out: &mut Vec<&'tcx Expr<'tcx>>) {
    if let ExprKind::Binary(inner, lhs, rhs) = e.kind
        && inner.node == op
    {
        flatten(lhs, op, out);
        flatten(rhs, op, out);
    } else {
        out.push(e);
    }
}

type Chain<'tcx> = (BinOpKind, Vec<&'tcx Expr<'tcx>>, Vec<&'tcx Expr<'tcx>>);

// The flat spelling, in which each disjunct restates every earlier equality:
//
//     a_1 > b_1 || (a_1 == b_1 && a_2 > b_2) || (a_1 == b_1 && a_2 == b_2 && a_3 > b_3)
//
// Disjunct `i` must be `i` equalities of the pairs already seen, followed by one comparison of the
// next pair.
fn flat_chain<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<Chain<'tcx>> {
    let mut disjuncts = Vec::new();
    flatten(e, BinOpKind::Or, &mut disjuncts);
    if disjuncts.len() < 2 {
        return None;
    }
    let (mut a, mut b, mut ops) = (Vec::new(), Vec::new(), Vec::new());
    for (i, disjunct) in disjuncts.iter().enumerate() {
        let mut conjuncts = Vec::new();
        flatten(disjunct, BinOpKind::And, &mut conjuncts);
        if conjuncts.len() != i + 1 {
            return None;
        }
        for (j, conjunct) in conjuncts[..i].iter().enumerate() {
            let ExprKind::Binary(eq, x, y) = conjunct.kind else {
                return None;
            };
            if eq.node != BinOpKind::Eq || !same_pair(cx, x, y, a[j], b[j]) {
                return None;
            }
        }
        let ExprKind::Binary(cmp, x, y) = conjuncts[i].kind else {
            return None;
        };
        strict(cmp.node)?;
        if !is_place(x) || !is_place(y) {
            return None;
        }
        a.push(x);
        b.push(y);
        ops.push(cmp.node);
    }
    let last = *ops.last()?;
    // Every element but the last is compared strictly, and all in the same direction.
    if ops[..ops.len() - 1]
        .iter()
        .any(|&op| Some(op) != strict(last))
    {
        return None;
    }
    Some((last, a, b))
}

// The nested spelling, in which the tail is itself a lexicographic comparison:
//
//     a_1 > b_1 || a_1 == b_1 && (a_2 > b_2 || a_2 == b_2 && a_3 > b_3)
fn nested_chain<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<Chain<'tcx>> {
    let ExprKind::Binary(op, lhs, rhs) = e.kind else {
        return None;
    };
    if strict(op.node).is_some() {
        return (is_place(lhs) && is_place(rhs)).then(|| (op.node, vec![lhs], vec![rhs]));
    }
    if op.node != BinOpKind::Or {
        return None;
    }
    let ExprKind::Binary(head, a, b) = lhs.kind else {
        return None;
    };
    let ExprKind::Binary(and, eq, tail) = rhs.kind else {
        return None;
    };
    let ExprKind::Binary(eq_op, x, y) = eq.kind else {
        return None;
    };
    if and.node != BinOpKind::And || eq_op.node != BinOpKind::Eq || !same_pair(cx, x, y, a, b) {
        return None;
    }
    if !is_place(a) || !is_place(b) {
        return None;
    }
    let (op, mut a_rest, mut b_rest) = lex_chain(cx, tail)?;
    if strict(op)? != head.node {
        return None;
    }
    let mut a_all = vec![a];
    a_all.append(&mut a_rest);
    let mut b_all = vec![b];
    b_all.append(&mut b_rest);
    Some((op, a_all, b_all))
}

// Whether `x` and `y` are the pair `a` and `b`, in either order.
fn same_pair<'tcx>(
    cx: &LateContext<'tcx>,
    x: &'tcx Expr<'tcx>,
    y: &'tcx Expr<'tcx>,
    a: &'tcx Expr<'tcx>,
    b: &'tcx Expr<'tcx>,
) -> bool {
    (eq_expr_value(cx, x, a) && eq_expr_value(cx, y, b))
        || (eq_expr_value(cx, x, b) && eq_expr_value(cx, y, a))
}

// A hand-written lexicographic comparison, in whichever spelling. The flat form is tried first: a
// flat three-element chain contains a two-element nested one, and reporting the longer chain is
// what makes the suggested rewrite cover the whole comparison.
fn lex_chain<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<Chain<'tcx>> {
    match (flat_chain(cx, e), nested_chain(cx, e)) {
        (Some(flat), Some(nested)) if nested.1.len() > flat.1.len() => Some(nested),
        (Some(flat), _) => Some(flat),
        (None, nested) => nested,
    }
}

fn tuple(cx: &LateContext<'_>, parts: &[&Expr<'_>]) -> Option<String> {
    let mut out = Vec::with_capacity(parts.len());
    for part in parts {
        out.push(snippet_opt(cx, part.span)?);
    }
    Some(format!("({})", out.join(", ")))
}

impl<'tcx> LateLintPass<'tcx> for ManualLexicographicCmp {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        let Some((op, a, b)) = lex_chain(cx, expr) else {
            return;
        };
        if a.len() < 2 {
            return;
        }
        // A longer chain contains shorter ones; only the outermost should report.
        let mut id = expr.hir_id;
        for _ in 0..a.len() * 2 {
            let Node::Expr(parent) = cx.tcx.parent_hir_node(id) else {
                break;
            };
            if lex_chain(cx, parent).is_some_and(|(_, outer, _)| outer.len() > a.len()) {
                return;
            }
            id = parent.hir_id;
        }
        let (Some(a_tuple), Some(b_tuple)) = (tuple(cx, &a), tuple(cx, &b)) else {
            return;
        };
        let op = op.as_str();
        span_lint_and_help(
            cx,
            MANUAL_LEXICOGRAPHIC_CMP,
            expr.span,
            format!(
                "compare tuples instead of spelling out a lexicographic comparison of {} elements",
                a.len()
            ),
            None,
            format!("`{a_tuple} {op} {b_tuple}` is the same comparison"),
        );
    }
}
