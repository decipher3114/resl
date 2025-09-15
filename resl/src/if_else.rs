use winnow::{
    ModalResult, Parser,
    combinator::{cut_err, preceded, separated_pair},
};

use crate::{
    StatefulInput,
    expr::Expr,
    macros::{exp_char, label},
    state::{EvalState, FmtState},
    value::Value,
};

/// Ternary if-else expression.
#[derive(Debug, Clone)]
pub struct IfElse {
    if_expr: Box<Expr>,
    then_expr: Box<Expr>,
    else_expr: Box<Expr>,
}

impl IfElse {
    pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
        separated_pair(
            preceded('?', Expr::require_parse.map(Box::new)),
            cut_err(':').context(exp_char!(':')),
            separated_pair(
                Expr::require_parse.map(Box::new),
                cut_err('|').context(exp_char!('|')),
                Expr::require_parse.map(Box::new),
            ),
        )
        .context(label!("conditional expression"))
        .map(|(if_expr, (then_expr, else_expr))| Self {
            if_expr,
            then_expr,
            else_expr,
        })
        .map(Expr::IfElse)
        .parse_next(input)
    }

    pub(crate) fn evaluate(self, state: &mut EvalState) -> Value {
        match self.if_expr.evaluate(state) {
            Value::Boolean(bool) => match bool {
                true => self.then_expr.evaluate(state),
                false => self.else_expr.evaluate(state),
            },
            _ => Value::Null,
        }
    }

    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        let pretty = state.pretty();

        write!(writer, "?")?;
        if pretty {
            write!(writer, " ")?;
        };

        self.if_expr.format(writer, state)?;

        write!(writer, ":")?;
        if pretty {
            write!(writer, " ")?;
        };

        self.then_expr.format(writer, state)?;

        write!(writer, "|")?;
        if pretty {
            write!(writer, " ")?;
        };

        self.else_expr.format(writer, state)?;
        Ok(())
    }
}
