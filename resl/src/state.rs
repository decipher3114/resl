use string_interner::{StringInterner, backend::StringBackend, symbol::SymbolU32};

use crate::{binding::Binding, context::Context, expr::Expr, ident::Ident};

pub(crate) type Interner = StringInterner<StringBackend>;

/// Manages all evaluation contexts and string interning.
///
/// CtxState holds the global state for RESL evaluation, including all variable
/// binding contexts organized in a hierarchical structure and a string interner
/// for efficient identifier storage.
#[derive(Debug)]
pub struct CtxState {
    contexts: Vec<Context>,
    interner: Interner,
}

impl CtxState {
    /// Creates a new context state with root context and built-in functions.
    pub(crate) fn new() -> Self {
        let mut interner = StringInterner::new();
        Self {
            contexts: vec![Context::root(&mut interner)],
            interner,
        }
    }

    /// Places a context at the specified index, resizing the context vector if needed.
    pub(crate) fn place_ctx(&mut self, ctx_idx: usize, ctx: Context) {
        if ctx_idx >= self.len() {
            self.contexts.resize(ctx_idx + 1, Context::default());
        }
        self[ctx_idx] = ctx;
    }

    /// Finds the context index that contains the specified identifier.
    pub(crate) fn find_ctx_with_ident(
        &self,
        current_ctx_idx: usize,
        ident: &Ident,
    ) -> Option<usize> {
        let mut ctx_idx = current_ctx_idx;
        loop {
            let ctx = &self[ctx_idx];
            if ctx.contains_key(ident) {
                return Some(ctx_idx);
            }
            ctx_idx = ctx.parent_ctx_idx()?;
        }
    }
}

impl Default for CtxState {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for CtxState {
    type Target = [Context];

    fn deref(&self) -> &Self::Target {
        &self.contexts
    }
}

impl std::ops::DerefMut for CtxState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.contexts
    }
}

/// State manager for parsing operations.
///
/// ParseState tracks the current context during parsing, manages context
/// creation for blocks and function definitions, and provides access to
/// the underlying context state and string interner.
#[derive(Debug)]
pub struct ParseState<'ctx> {
    active_ctx_idx: usize,
    avail_ctx_idx: usize,
    ctx_state: &'ctx mut CtxState,
}

impl<'ctx> ParseState<'ctx> {
    /// Creates a new parse state from the context state.
    pub(crate) fn new(ctx_state: &'ctx mut CtxState) -> Self {
        let ctx_state_len = ctx_state.contexts.len();
        Self {
            active_ctx_idx: ctx_state_len - 1,
            avail_ctx_idx: ctx_state_len,
            ctx_state,
        }
    }

    /// Gets the current active context index.
    pub(crate) fn active_ctx_idx(&self) -> usize {
        self.active_ctx_idx
    }

    /// Sets the active context for parsing operations.
    pub(crate) fn set_active_ctx(&mut self, ctx_idx: usize) {
        self.active_ctx_idx = ctx_idx;
    }

    /// Gets the next available context index for new contexts.
    pub(crate) fn avail_ctx_idx(&mut self) -> usize {
        self.avail_ctx_idx
    }

    /// Increments the available context index counter.
    pub(crate) fn increment_avail_ctx_idx(&mut self) {
        self.avail_ctx_idx += 1;
    }

    /// Decrements the available context index counter.
    pub(crate) fn decrement_avail_ctx_idx(&mut self) {
        self.avail_ctx_idx -= 1;
    }

    /// Interns a string and returns its symbol identifier.
    pub(crate) fn get_interned(&mut self, s: &str) -> SymbolU32 {
        self.ctx_state.interner.get_or_intern(s)
    }
}

impl std::ops::Deref for ParseState<'_> {
    type Target = CtxState;

    fn deref(&self) -> &Self::Target {
        self.ctx_state
    }
}

impl std::ops::DerefMut for ParseState<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ctx_state
    }
}

/// State manager for expression evaluation.
///
/// EvalState manages the active context during expression evaluation and
/// provides access to variable bindings and context switching for function
/// calls and block evaluation.
#[derive(Debug)]
pub struct EvalState<'ctx> {
    active_ctx_idx: usize,
    ctx_state: &'ctx mut CtxState,
}

impl<'ctx> EvalState<'ctx> {
    /// Creates a new evaluation state starting from the root context.
    pub(crate) fn new(ctx_state: &'ctx mut CtxState) -> Self {
        Self {
            active_ctx_idx: 0,
            ctx_state,
        }
    }

    /// Gets the current active context index.
    pub(crate) fn active_ctx_idx(&self) -> usize {
        self.active_ctx_idx
    }

    /// Sets the active context for evaluation operations.
    pub(crate) fn set_active_ctx(&mut self, ctx_idx: usize) {
        self.active_ctx_idx = ctx_idx;
    }

    /// Finds the context that contains the specified identifier.
    pub(crate) fn find_ctx_with_ident(&self, ident: &Ident) -> Option<usize> {
        self.ctx_state
            .find_ctx_with_ident(self.active_ctx_idx, ident)
    }

    /// Gets the expression bound to an identifier, if it exists.
    pub(crate) fn get_expr<'a>(&'a mut self, ident: &'a Ident) -> Option<&'a Expr> {
        let ctx_idx = self.find_ctx_with_ident(ident)?;
        if let Some(Binding::Expr(expr)) = self[ctx_idx].get(ident) {
            return Some(expr);
        }
        None
    }
}

impl std::ops::Deref for EvalState<'_> {
    type Target = CtxState;

    fn deref(&self) -> &Self::Target {
        self.ctx_state
    }
}

impl std::ops::DerefMut for EvalState<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ctx_state
    }
}

/// State manager for formatting operations.
///
/// FmtState controls formatting options like pretty-printing and indentation
/// level, and provides read-only access to context state for variable
/// resolution during expression formatting.
#[derive(Debug, Clone, Copy)]
pub struct FmtState<'ctx> {
    pretty: bool,
    indent_level: usize,
    ctx_state: &'ctx CtxState,
}

impl<'ctx> FmtState<'ctx> {
    /// Creates a new format state with the specified pretty-printing mode.
    pub(crate) fn new(pretty: bool, ctx_state: &'ctx CtxState) -> Self {
        Self {
            pretty,
            indent_level: 0,
            ctx_state,
        }
    }

    /// Creates a new format state with increased indentation level.
    pub(crate) fn indented(&self) -> Self {
        Self {
            pretty: self.pretty,
            indent_level: self.indent_level + 1,
            ctx_state: self.ctx_state,
        }
    }

    /// Returns whether pretty-printing is enabled.
    pub(crate) fn pretty(&self) -> bool {
        self.pretty
    }

    /// Gets the current indentation level.
    pub(crate) fn indent_level(&self) -> usize {
        self.indent_level
    }

    /// Resolves an identifier to its string representation.
    pub(crate) fn resolve_ident(&self, ident: &Ident) -> &str {
        self.ctx_state
            .interner
            .resolve(ident.to_symbol())
            .expect("Identifier not found in interner")
    }
}

impl std::ops::Deref for FmtState<'_> {
    type Target = CtxState;

    fn deref(&self) -> &Self::Target {
        self.ctx_state
    }
}
