use crate::{
    binding::Binding,
    expr::Expr,
    function::builtin::BUILTIN_FUNCTIONS,
    ident::Ident,
    state::{FmtState, Interner},
    utils::write_indent,
    value::Value,
};

type Bindings = indexmap::IndexMap<Ident, Binding>;
type LookupStack = std::collections::HashSet<Ident>;

/// Represents a variable binding context with optional parent scope.
///
/// Contexts manage variable bindings within a specific scope and provide
/// hierarchical variable resolution through parent contexts. Each context
/// maintains its own set of variable bindings and tracks circular reference
/// detection during variable lookups.
#[derive(Debug, Default, Clone)]
pub struct Context {
    parent_ctx_idx: Option<usize>,
    bindings: Bindings,
    lookup_stack: LookupStack,
}

impl Context {
    /// Creates the root context with built-in functions.
    pub(crate) fn root(interner: &mut Interner) -> Self {
        let bindings = Bindings::from_iter(BUILTIN_FUNCTIONS.into_iter().map(|(name, func)| {
            (
                Ident::using_interner(name, interner),
                Binding::Expr(Expr::Fn(func)),
            )
        }));

        Self {
            parent_ctx_idx: None,
            bindings,
            lookup_stack: LookupStack::new(),
        }
    }

    /// Gets the parent context index, if any.
    pub(crate) fn parent_ctx_idx(&self) -> Option<usize> {
        self.parent_ctx_idx
    }

    /// Creates a context from an iterator of identifier-expression pairs.
    pub(crate) fn from_iter<T, B>(parent_ctx_idx: Option<usize>, iter: T) -> Self
    where
        T: IntoIterator<Item = (Ident, B)>,
        B: Into<Binding>,
    {
        Self {
            parent_ctx_idx,
            bindings: Bindings::from_iter(iter.into_iter().map(|(k, b)| (k, b.into()))),
            lookup_stack: LookupStack::new(),
        }
    }

    /// Assigns expressions to existing bindings from an iterator.
    pub(crate) fn assign_from_iter<T, B>(&mut self, iter: T)
    where
        T: IntoIterator<Item = B>,
        B: Into<Binding>,
    {
        let mut iter = iter.into_iter();
        for (_, binding) in &mut self.bindings {
            // SAFETY: We ensure that the number of expressions matches the number of bindings
            let b = unsafe { iter.next().unwrap_unchecked() };

            *binding = b.into();
        }
    }

    /// Resets all bindings to default expressions.
    pub(crate) fn reassign_default_expr(&mut self) {
        self.iter_mut()
            .for_each(|(_, binding)| *binding = Binding::default());
    }

    /// Initiates a variable lookup, returns true if this creates a circular reference.
    pub(crate) fn initiate_lookup(&mut self, ident: &Ident) -> bool {
        self.lookup_stack.insert(ident.to_owned())
    }

    /// Concludes a variable lookup, removing it from the lookup stack.
    pub(crate) fn conclude_lookup(&mut self, ident: &Ident) {
        self.lookup_stack.remove(ident);
    }

    /// Caches a computed value for an identifier.
    pub(crate) fn cache(&mut self, ident: &Ident, value: Value) {
        self[ident] = Binding::Cached(value)
    }

    /// Formats the context's bindings to a writer with proper indentation.
    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        for (name, expr) in &self.bindings {
            if state.pretty() {
                write_indent(writer, state.indent_level())?;
            }
            name.format(writer, state)?;
            if state.pretty() {
                write!(writer, " ")?;
            }
            write!(writer, "=")?;
            if state.pretty() {
                write!(writer, " ")?;
            }
            expr.format(writer, state)?;
            write!(writer, ";")?;
            if state.pretty() {
                writeln!(writer)?;
            }
        }
        Ok(())
    }
}

impl std::ops::Deref for Context {
    type Target = Bindings;

    fn deref(&self) -> &Self::Target {
        &self.bindings
    }
}

impl std::ops::DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bindings
    }
}
