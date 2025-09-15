use crate::{expr::Expr, state::FmtState, value::Value};

/// Represents a variable binding in the evaluation context.
///
/// Bindings can either be unevaluated expressions that will be computed when accessed,
/// or cached values that have already been computed. This enables lazy evaluation and
/// caching of variable values during expression evaluation.
#[derive(Debug, Clone)]
pub enum Binding {
    /// An unevaluated expression that will be computed when the binding is accessed
    Expr(Expr),
    /// A cached value that has already been computed and stored
    Cached(Value),
}

impl Binding {
    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        match self {
            Binding::Expr(expr) => expr.format(writer, state),
            Binding::Cached(_) => unreachable!("Cached values should not be formatted."),
        }
    }
}

impl From<Expr> for Binding {
    fn from(expr: Expr) -> Self {
        Binding::Expr(expr)
    }
}

impl From<Value> for Binding {
    fn from(value: Value) -> Self {
        Binding::Cached(value)
    }
}

impl Default for Binding {
    fn default() -> Self {
        Binding::Expr(Expr::default())
    }
}
