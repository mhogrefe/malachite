// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::source::snippet;
use rustc_ast::{IntTy, LitIntType, LitKind, UintTy};
use rustc_hir::{BinOpKind, Expr, ExprKind, UnOp};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags an integer literal that is compared with a bignum (`Natural`, `Integer`, `Rational`,
    /// `Float`, `GaussianInteger`, or `GaussianRational`), or that is the shift count of one, when
    /// the literal is not a `u32` (or an `i32`, when it is negative). This covers the comparison
    /// operators, the `eq`/`ne`/`partial_cmp`/`cmp`/`lt`/`le`/`gt`/`ge` methods and their `*_abs`
    /// counterparts, `<<`/`>>` and their assignment forms, and the `shl_*`/`shr_*` method families.
    ///
    /// ### Why is this bad?
    ///
    /// The bignum types implement comparisons and shifts against every primitive integer type, so
    /// any suffix compiles, and an unsuffixed literal silently selects the `i32` implementation.
    /// The `u32`/`i32` convention is the fastest choice: on 64-bit limbs every width takes the
    /// same single-limb path, and on 32-bit limbs a `u64` comparand is wider than a limb, so
    /// comparing with it walks the limbs in a loop (and, for `Float`, builds a `Natural` from it).
    /// Shift counts are converted to `u64` internally whatever their type, so the shorter suffix
    /// costs nothing there, and one convention for both is easier to remember.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if x == 1u64 || x < -1i64 || y > 3 {}
    /// let z = y << 1u64;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if x == 1u32 || x < -1i32 || y > 3u32 {}
    /// let z = y << 1u32;
    /// ```
    pub BIGNUM_LITERAL_SUFFIX,
    Deny,
    "comparing a bignum with, or shifting one by, an integer literal that is not `u32` (or `i32` when negative)"
}

declare_lint_pass!(BignumLiteralSuffix => [BIGNUM_LITERAL_SUFFIX]);

// The integer literal suffixes, longest first so that `u128` is not mistaken for `u1` + `28`.
const SUFFIXES: [&str; 12] = [
    "u128", "i128", "usize", "isize", "u16", "u32", "u64", "i16", "i32", "i64", "u8", "i8",
];

// Whether the type of `e` (through references) is a bignum.
fn is_bignum<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> bool {
    crate::bignum_name(cx, cx.typeck_results().expr_ty(e).peel_refs()).is_some()
}

// If `e`, after peeling `&`s, is an integer literal or a negated one, returns the expression to
// replace (the literal, or the negation), the literal itself, whether it is negated, and its
// suffix.
fn int_literal<'tcx>(
    e: &'tcx Expr<'tcx>,
) -> Option<(&'tcx Expr<'tcx>, &'tcx Expr<'tcx>, bool, LitIntType)> {
    let mut e = e;
    while let ExprKind::AddrOf(_, _, inner) = e.kind {
        e = inner;
    }
    let (lit_expr, neg) = if let ExprKind::Unary(UnOp::Neg, inner) = e.kind {
        (inner, true)
    } else {
        (e, false)
    };
    let ExprKind::Lit(lit) = lit_expr.kind else {
        return None;
    };
    let LitKind::Int(_, ty) = lit.node else {
        return None;
    };
    Some((e, lit_expr, neg, ty))
}

// Lints `e` if it is an integer literal that departs from the convention. `role` describes the
// literal's use in the message.
fn check_literal<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>, role: &str) {
    let Some((e, lit_expr, neg, ty)) = int_literal(e) else {
        return;
    };
    let conforms = match ty {
        LitIntType::Unsigned(UintTy::U32) => !neg,
        LitIntType::Signed(IntTy::I32) => neg,
        _ => false,
    };
    if conforms {
        return;
    }
    let (sign, suffix) = if neg { ("-", "i32") } else { ("", "u32") };
    // The digits as written (hex, underscores, and all), without any existing suffix.
    let text = snippet(cx, lit_expr.span, "..");
    let digits = SUFFIXES
        .iter()
        .find_map(|suffix| text.strip_suffix(suffix))
        .unwrap_or(&text);
    span_lint(
        cx,
        BIGNUM_LITERAL_SUFFIX,
        e.span,
        format!(
            "use `{sign}{digits}{suffix}`: a literal {role} is a `u32`, or an `i32` when negative"
        ),
    );
}

// The files that exercise the multi-type comparison and shift APIs on purpose: the comparison
// modules and the `shl`/`shr` family, in source, test, and extracted-doctest form (the doctest file
// names encode the module path with underscores), plus the operator examples in the arithmetic
// module docs, extracted as `<type>_arithmetic_mod_<n>.rs`.
fn in_showcase_file(cx: &LateContext<'_>, span: rustc_span::Span) -> bool {
    let rustc_span::FileName::Real(real) = cx.sess().source_map().span_to_filename(span) else {
        return false;
    };
    let Some(path) = real.local_path() else {
        return false;
    };
    let path = path.to_string_lossy().replace('\\', "/");
    let name = path.rsplit('/').next().unwrap_or("");
    path.contains("/comparison/")
        || path.contains("_comparison_")
        || name.starts_with("shl")
        || name.starts_with("shr")
        || path.contains("_arithmetic_shl")
        || path.contains("_arithmetic_shr")
        || path
            .rsplit_once("_arithmetic_mod_")
            .is_some_and(|(_, rest)| {
                rest.strip_suffix(".rs")
                    .is_some_and(|n| n.bytes().all(|b| b.is_ascii_digit()))
            })
}

impl<'tcx> LateLintPass<'tcx> for BignumLiteralSuffix {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || in_showcase_file(cx, expr.span) {
            return;
        }
        match expr.kind {
            ExprKind::Binary(op, lhs, rhs) => match op.node {
                BinOpKind::Eq
                | BinOpKind::Ne
                | BinOpKind::Lt
                | BinOpKind::Le
                | BinOpKind::Gt
                | BinOpKind::Ge => {
                    for (bignum, other) in [(lhs, rhs), (rhs, lhs)] {
                        if is_bignum(cx, bignum) {
                            check_literal(cx, other, "compared with a bignum");
                        }
                    }
                }
                BinOpKind::Shl | BinOpKind::Shr => {
                    if is_bignum(cx, lhs) {
                        check_literal(cx, rhs, "shifting a bignum");
                    }
                }
                _ => {}
            },
            ExprKind::AssignOp(op, lhs, rhs) => {
                if matches!(op.node.into(), BinOpKind::Shl | BinOpKind::Shr) && is_bignum(cx, lhs) {
                    check_literal(cx, rhs, "shifting a bignum");
                }
            }
            ExprKind::MethodCall(seg, recv, args, _) => {
                if !is_bignum(cx, recv) {
                    return;
                }
                let name = seg.ident.name.as_str();
                if let [arg] = args
                    && matches!(
                        name,
                        "eq" | "ne"
                            | "partial_cmp"
                            | "cmp"
                            | "lt"
                            | "le"
                            | "gt"
                            | "ge"
                            | "eq_abs"
                            | "ne_abs"
                            | "partial_cmp_abs"
                            | "cmp_abs"
                            | "lt_abs"
                            | "le_abs"
                            | "gt_abs"
                            | "ge_abs"
                    )
                {
                    check_literal(cx, arg, "compared with a bignum");
                } else if let [count, ..] = args
                    && (name.starts_with("shl") || name.starts_with("shr"))
                {
                    check_literal(cx, count, "shifting a bignum");
                }
            }
            _ => {}
        }
    }
}
