use winnow::{
    ModalResult, Parser,
    combinator::{cut_err, delimited, separated_pair},
};

use crate::{
    EvalState, Expr, FmtState, StatefulInput, Value,
    binding::Binding,
    context::Context,
    ident::Ident,
    macros::{exp_char, exp_desc},
    utils::delimited_multispace0,
    value::{ValueList, ValueMap},
};

// For-each expression for iterating over lists or maps.
#[derive(Debug, Clone)]
pub struct ForEach {
    base: Ident,
    ctx_idx: usize,
    body: Box<Expr>,
}

impl ForEach {
    pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
        // Store the current active context index before parsing
        let current_ctx_idx = input.state.active_ctx_idx();

        // Get the next available context index
        let ctx_idx = input.state.avail_ctx_idx();

        // Increment the available context index for nested expressions
        input.state.increment_avail_ctx_idx();

        // Set this new context as active
        // This ensures that contexts of nested exprs have this context as parent
        input.state.set_active_ctx(ctx_idx);

        // Parse the expression without unwrapping the result
        // This allows restoring the state later
        let parse_result = separated_pair(
            Ident::parse_ident,
            delimited_multispace0('>'),
            separated_pair(
                delimited(
                    cut_err('(').context(exp_char!('(')),
                    delimited_multispace0(separated_pair(
                        // Identifier for key/index
                        cut_err(Ident::parse_ident).context(exp_desc!("index identifier")),
                        delimited_multispace0(cut_err(',').context(exp_char!(','))),
                        // Identifier for value/element
                        cut_err(Ident::parse_ident).context(exp_desc!("item identifier")),
                    )),
                    cut_err(')').context(exp_char!(')')),
                ),
                delimited_multispace0(cut_err(":").context(exp_char!(':'))),
                Expr::require_parse.map(Box::new),
            ),
        )
        .parse_next(input);

        // Restore active context to previous one
        input.state.set_active_ctx(current_ctx_idx);

        let (base, ((key_ident, value_ident), body)) = parse_result.inspect_err(|_| {
            // Returned backtrack error during parsing

            // Decrement avail_ctx_idx to avoid skipping indices
            input.state.decrement_avail_ctx_idx();
        })?;

        // Create an empty context for key and value indents
        let ctx = Context::from_iter(
            Some(current_ctx_idx),
            [
                (key_ident, Binding::default()),
                (value_ident, Binding::default()),
            ],
        );

        // Place the context at the specified index
        input.state.place_ctx(ctx_idx, ctx);

        Ok(Expr::ForEach(Self {
            base,
            ctx_idx,
            body,
        }))
    }

    pub(crate) fn evaluate(self, state: &mut EvalState) -> Value {
        let base_value = match self.base.evaluate(state) {
            Some(value) => match value {
                Value::List(_) | Value::Map(_) => value.clone(),
                _ => return Value::Null,
            },
            _ => return Value::Null,
        };

        // Save the index of the current active ctx
        let current_ctx_idx = state.active_ctx_idx();

        // Set self as the active context
        state.set_active_ctx(self.ctx_idx);

        let value = match base_value {
            Value::List(list) => {
                let mut value_list = ValueList::new();
                for (index, element) in list.iter().enumerate() {
                    // Assign the index and element values to the context
                    state[self.ctx_idx]
                        .assign_from_iter([Value::Integer(index as i64), element.to_owned()]);

                    // Evaluate the expression in the context of this block
                    let value = self.body.to_owned().evaluate(state);

                    // Push the evaluated value to the result list
                    value_list.push(value);
                }
                Value::List(value_list)
            }
            Value::Map(map) => {
                let mut value_map = ValueMap::new();
                for (key, val) in map.iter() {
                    // Assign the key and value to the context
                    state[self.ctx_idx]
                        .assign_from_iter([Value::String(key.to_owned()), val.to_owned()]);

                    // Evaluate the expression in the context of this block
                    let value = self.body.to_owned().evaluate(state);

                    // Push the evaluated value to the result list
                    value_map.insert(key.to_owned(), value);
                }
                Value::Map(value_map)
            }
            _ => unreachable!("This is ensured by the match at the beginning"),
        };

        // Reset active context
        state.set_active_ctx(current_ctx_idx);

        // Reset the context expressions to Null
        state[self.ctx_idx].reassign_default_expr();

        value
    }

    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        self.base.format(writer, state)?;

        if state.pretty() {
            write!(writer, " ")?;
        }

        write!(writer, ">")?;

        if state.pretty() {
            write!(writer, " ")?;
        }

        write!(writer, "(")?;

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

        write!(writer, ")")?;

        if state.pretty() {
            write!(writer, " ")?;
        }

        write!(writer, ":")?;

        if state.pretty() {
            write!(writer, " ")?;
        }

        self.body.format(writer, state)
    }
}
