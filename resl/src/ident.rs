use string_interner::symbol::SymbolU32;
use winnow::{ModalResult, Parser};

use crate::{
    StatefulInput,
    binding::Binding,
    expr::Expr,
    state::{EvalState, FmtState, Interner},
    string,
    value::Value,
};

/// Variable or function identifier.
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Ident(SymbolU32);

impl Ident {
    pub(crate) fn parse_ident(input: &mut StatefulInput) -> ModalResult<Self> {
        let ident = string::parse_plain
            .verify(|s| !["true", "false", "null"].contains(&s))
            .parse_next(input)?;

        Ok(Ident(input.state.get_interned(ident)))
    }

    pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
        Self::parse_ident.map(Expr::Ident).parse_next(input)
    }

    pub(crate) fn evaluate<'a>(self, state: &'a mut EvalState) -> Option<&'a Value> {
        // Save current context index to restore later
        let current_ctx_idx = state.active_ctx_idx();

        // Find the context index containing the identifier
        // This will start from current context upto parent contexts
        let ctx_idx = state.find_ctx_with_ident(&self)?;

        // Set the context index containing the identifier as active
        // This ensures that any nested lookups have this context as their parent
        state.set_active_ctx(ctx_idx);

        // Initiate the lookup for the identifier
        // This prevents infinite recursion for cyclic dependencies (Context Sensitive)
        if !state[ctx_idx].initiate_lookup(&self) {
            return None;
        }

        // Get the expression or cached value for the identifier
        if let Some(Binding::Expr(expr)) = state[ctx_idx].get(&self) {
            let cacheable = expr.should_be_cached();

            let value = expr.to_owned().evaluate(state);

            if cacheable {
                state[ctx_idx].cache(&self, value);
            };
        };

        // Conclude the lookup for the identifier
        state[ctx_idx].conclude_lookup(&self);

        // Restore the previous active context index
        state.set_active_ctx(current_ctx_idx);

        match state[ctx_idx].get(&self) {
            Some(Binding::Cached(value)) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        let name = state.resolve_ident(self);
        write!(writer, "{name}")
    }

    pub(crate) fn using_interner(name: &str, interner: &mut Interner) -> Self {
        Self(interner.get_or_intern(name))
    }

    pub(crate) fn to_symbol(&self) -> SymbolU32 {
        self.0
    }
}
