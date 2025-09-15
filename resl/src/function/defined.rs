use winnow::{
    ModalResult, Parser,
    combinator::{alt, cut_err, delimited, fail, preceded, separated},
};

use crate::{
    StatefulInput,
    binding::Binding,
    context::Context,
    expr::Expr,
    ident::Ident,
    macros::{exp_char, exp_desc, label},
    state::{EvalState, FmtState},
    utils::delimited_multispace0,
    value::Value,
};

/// User-declared function.
#[derive(Debug, Clone)]
pub struct Defined {
    ctx_idx: usize,
    body: Box<Expr>,
}

impl Defined {
    pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Self> {
        // Store the current active context index before parsing the block
        let current_ctx_idx = input.state.active_ctx_idx();

        // Get the next available context index
        // This will be the new block's context index
        let ctx_idx = input.state.avail_ctx_idx();

        // Increment the available context index for nested expressions
        input.state.increment_avail_ctx_idx();

        // Set this new block's context as active
        // This ensures that the nested blocks have this block as their parent
        input.state.set_active_ctx(ctx_idx);

        // Parse the block without unwrapping the result
        // This allows restoring the state variables later
        let parse_result = (
            delimited(
                '|',
                alt((
                    delimited_multispace0(separated(
                        1..,
                        Ident::parse_ident,
                        delimited_multispace0(','),
                    )),
                    cut_err(fail).context(exp_desc!("one or more parameters")),
                )),
                alt((
                    preceded(
                        delimited_multispace0(','),
                        cut_err(']')
                            .context(exp_desc!("an expression"))
                            .context(exp_char!(']')),
                    ),
                    cut_err('|').context(exp_char!(',')).context(exp_char!('|')),
                )),
            ),
            Expr::require_parse.map(Box::new),
        )
            .context(label!("function declaration"))
            .parse_next(input);

        // Restore active context to previous one
        input.state.set_active_ctx(current_ctx_idx);

        let (params, body): (Vec<Ident>, Box<Expr>) = parse_result.inspect_err(|_| {
            // Returned backtrack error during parsing
            // The expression might not be a declaration

            // Decrement avail_ctx_idx to avoid skipping indices
            input.state.decrement_avail_ctx_idx();
        })?;

        // Create a context with params as keys and default bindings
        let ctx = Context::from_iter(
            Some(current_ctx_idx),
            params.into_iter().map(|p| (p, Binding::default())),
        );

        // Place this context in the state
        // This context will then be updated during function call evaluation
        input.state.place_ctx(ctx_idx, ctx);

        Ok(Self { ctx_idx, body })
    }

    pub(crate) fn evaluate(self, state: &mut EvalState, args: Vec<Expr>) -> Value {
        // Check if the number of arguments matches the number of parameters
        if args.len() != state[self.ctx_idx].len() {
            return Value::Null;
        }

        // Assign argument expressions to the function's context
        state[self.ctx_idx].assign_from_iter(args);

        // Save the index of the current active ctx
        let current_ctx_idx = state.active_ctx_idx();

        // Set self as the active context
        state.set_active_ctx(self.ctx_idx);

        // Evaluate the expression in the context of this block
        let value = self.body.to_owned().evaluate(state);

        // Reset active context
        state.set_active_ctx(current_ctx_idx);

        // Reset the context expressions to Null for future calls
        state[self.ctx_idx].reassign_default_expr();

        value
    }

    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        write!(writer, "|")?;

        let mut params_iter = state[self.ctx_idx].keys().peekable();

        while let Some(param) = params_iter.next() {
            param.format(writer, state)?;

            if params_iter.peek().is_some() {
                write!(writer, ",")?;
                if state.pretty() {
                    write!(writer, " ")?;
                }
            }
        }
        write!(writer, "|")?;

        if state.pretty() {
            write!(writer, " ")?;
        }

        self.body.format(writer, state)
    }
}
