use winnow::{
    ModalResult, Parser,
    combinator::{alt, cut_err, delimited, fail, repeat, separated_pair, terminated},
};

use crate::{
    StatefulInput,
    context::Context,
    expr::Expr,
    function::Fn,
    ident::Ident,
    macros::{exp_char, exp_desc, label},
    state::{EvalState, FmtState},
    utils::{delimited_multispace0, write_indent},
    value::Value,
};

/// Block expression with scoped variables.
#[derive(Debug, Clone)]
pub struct Block {
    ctx_idx: usize,
    return_expr: Box<Expr>,
}

impl Block {
    pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
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
        let parse_result = delimited(
            '{',
            (
                // Assignments
                delimited_multispace0(alt((
                    repeat(
                        // One or more assignments
                        1..,
                        delimited_multispace0(terminated(
                            separated_pair(
                                Ident::parse_ident,
                                delimited_multispace0('='),
                                alt((
                                    Expr::parse,
                                    // Allow function definitions as well
                                    Fn::parse,
                                    cut_err(fail).context(exp_desc!("an expression")),
                                )),
                            ),
                            cut_err(';').context(exp_char!(';')),
                        )),
                    ),
                    cut_err(fail).context(exp_desc!("at least one assignment")),
                ))),
                // Required final expression in the block
                Expr::require_parse.map(Box::new),
            ),
            cut_err('}').context(exp_char!('}')),
        )
        .context(label!("block expression"))
        .parse_next(input);

        // Restore active context to previous one
        input.state.set_active_ctx(current_ctx_idx);

        let (assignments, return_expr): (Vec<(Ident, Expr)>, Box<Expr>) = parse_result
            .inspect_err(|_| {
                // Returned backtrack error during parsing

                // Decrement avail_ctx_idx to avoid skipping indices
                input.state.decrement_avail_ctx_idx();
            })?;

        // Create a new context for this block with the parsed assignments
        let ctx = Context::from_iter(Some(current_ctx_idx), assignments);

        // Place the new context at its index in the state's contexts
        input.state.place_ctx(ctx_idx, ctx);

        Ok(Expr::Block(Self {
            ctx_idx,
            return_expr,
        }))
    }

    pub(crate) fn evaluate(self, state: &mut EvalState) -> Value {
        // Save the index of the current ctx
        let current_ctx_idx = state.active_ctx_idx();

        // Set self as the active context
        state.set_active_ctx(self.ctx_idx);

        // Evaluate the expression in the context of this block
        let value = self.return_expr.evaluate(state);

        // Reset active context to parent context
        state.set_active_ctx(current_ctx_idx);

        value
    }

    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        let pretty = state.pretty();

        write!(writer, "{{")?;

        if pretty {
            writeln!(writer)?;
        }

        let ctx = &state[self.ctx_idx];
        ctx.format(writer, state.indented())?;

        if pretty {
            write_indent(writer, state.indented().indent_level())?;
        }

        self.return_expr.format(writer, state.indented())?;

        if pretty {
            writeln!(writer)?;
            write_indent(writer, state.indent_level())?;
        }

        write!(writer, "}}")
    }
}
