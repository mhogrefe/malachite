// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::source::snippet_opt;
use rustc_ast::{Block, Crate, Item, ItemKind, ModKind, StmtKind, UseTreeKind, VisibilityKind};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags two or more adjacent `use` items whose paths are identical except for the final
    /// component, like `use malachite_float::ComparableFloat;` followed by `use
    /// malachite_float::Float;`. A brace group counts as a final component, so a single import
    /// sitting next to a braced one from the same module is flagged too.
    ///
    /// ### Why is this bad?
    ///
    /// One braced import states the shared path once, so the reader sees at a glance which names
    /// come from the same module, and adding a name later touches one line instead of adding
    /// another. `rustfmt` sorts imports but does not merge them, so nothing else catches this.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use malachite_float::ComparableFloat;
    /// use malachite_float::Float;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// use malachite_float::{ComparableFloat, Float};
    /// ```
    pub COLLAPSE_ADJACENT_IMPORTS,
    Deny,
    "adjacent `use` items differing only in their final component"
}

declare_lint_pass!(CollapseAdjacentImports => [COLLAPSE_ADJACENT_IMPORTS]);

// What must match for two adjacent `use` items to be mergeable: the path up to (but excluding) the
// final component, and the visibility, since `pub use` and `use` cannot share a brace group.
type Key = (Vec<String>, String);

// The merge key of a `use` item, or `None` if the item is not a mergeable `use` — which also makes
// it a run-breaker, since anything between two imports means they are not adjacent.
fn merge_key(cx: &EarlyContext<'_>, item: &Item) -> Option<Key> {
    // An attribute (`#[cfg(...)]`, a doc comment) applies to the whole item, so it cannot be
    // carried into a shared brace group.
    if !item.attrs.is_empty() || item.span.from_expansion() {
        return None;
    }
    let ItemKind::Use(tree) = &item.kind else {
        return None;
    };
    let segments: Vec<String> = tree
        .prefix
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect();
    let prefix = match tree.kind {
        // `use a::b::C;` — everything but `C` is shared. A single-segment `use a;` has no prefix
        // to share with anything.
        UseTreeKind::Simple(_) => {
            if segments.len() < 2 {
                return None;
            }
            segments[..segments.len() - 1].to_vec()
        }
        // `use a::b::{C, D};` — the brace group is itself the final component.
        UseTreeKind::Nested { .. } => segments,
        // A glob subsumes its siblings rather than merging with them; that is a different problem.
        UseTreeKind::Glob(_) => return None,
    };
    let vis = match &item.vis.kind {
        VisibilityKind::Inherited => String::new(),
        VisibilityKind::Public => "pub".to_string(),
        VisibilityKind::Restricted { .. } => snippet_opt(cx, item.vis.span)?,
    };
    Some((prefix, vis))
}

// The text each item contributes inside the merged brace group: `C`, `C as D`, or the contents of
// an existing brace group, verbatim.
fn merged_leaves(cx: &EarlyContext<'_>, run: &[&Item]) -> Option<Vec<String>> {
    let mut leaves = Vec::with_capacity(run.len());
    for item in run {
        let ItemKind::Use(tree) = &item.kind else {
            return None;
        };
        match &tree.kind {
            UseTreeKind::Simple(rename) => {
                let last = tree.prefix.segments.last()?.ident;
                leaves.push(match rename {
                    Some(rename) => format!("{last} as {rename}"),
                    None => last.to_string(),
                });
            }
            // Reuse the source text rather than reprinting the tree: a nested group may itself
            // nest (`{b::{C, D}, E}`), and its contents are already a comma-separated list that
            // drops straight into the merged group.
            UseTreeKind::Nested { span, .. } => {
                let text = snippet_opt(cx, *span)?;
                let inner = text
                    .trim()
                    .strip_prefix('{')?
                    .strip_suffix('}')?
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                let inner = inner.trim().trim_end_matches(',').trim();
                if !inner.is_empty() {
                    leaves.push(inner.to_string());
                }
            }
            UseTreeKind::Glob(_) => return None,
        }
    }
    Some(leaves)
}

fn report(cx: &EarlyContext<'_>, run: &[&Item], key: Option<&Key>) {
    let (Some((prefix, vis)), [first, .., last]) = (key, run) else {
        return;
    };
    let message = format!(
        "these {} `use` items differ only in their final component",
        run.len()
    );
    let vis = if vis.is_empty() {
        String::new()
    } else {
        format!("{vis} ")
    };
    let help = match merged_leaves(cx, run) {
        Some(leaves) => format!(
            "merge them into `{vis}use {}::{{{}}};`",
            prefix.join("::"),
            leaves.join(", ")
        ),
        None => format!("merge them into a single braced `{vis}use`"),
    };
    span_lint_and_help(
        cx,
        COLLAPSE_ADJACENT_IMPORTS,
        first.span.to(last.span),
        message,
        None,
        help,
    );
}

// Scans one sequence of sibling items for runs of mergeable imports. A `None` entry stands for
// something that is not an item at all (a statement in a block), which breaks any run in progress.
fn scan<'a>(cx: &EarlyContext<'_>, items: impl Iterator<Item = Option<&'a Item>>) {
    let mut run: Vec<&'a Item> = Vec::new();
    let mut key: Option<Key> = None;
    for slot in items {
        match slot.and_then(|item| merge_key(cx, item)) {
            Some(k) if key.as_ref() == Some(&k) => run.push(slot.unwrap()),
            next => {
                report(cx, &run, key.as_ref());
                run.clear();
                if next.is_some() {
                    run.push(slot.unwrap());
                }
                key = next;
            }
        }
    }
    report(cx, &run, key.as_ref());
}

impl EarlyLintPass for CollapseAdjacentImports {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, krate: &Crate) {
        scan(cx, krate.items.iter().map(|item| Some(&**item)));
    }

    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &Item) {
        if let ItemKind::Mod(_, _, ModKind::Loaded(items, ..)) = &item.kind {
            scan(cx, items.iter().map(|item| Some(&**item)));
        }
    }

    fn check_block(&mut self, cx: &EarlyContext<'_>, block: &Block) {
        scan(
            cx,
            block.stmts.iter().map(|stmt| match &stmt.kind {
                StmtKind::Item(item) => Some(&**item),
                _ => None,
            }),
        );
    }
}
