use winnow::{
    ModalResult, Parser,
    combinator::{alt, cut_err, delimited, fail, preceded, separated},
};

use crate::{
    StatefulInput,
    expr::Expr,
    function::Fn,
    ident::Ident,
    macros::{exp_char, exp_desc, label},
    state::{EvalState, FmtState},
    utils::delimited_multispace0,
    value::Value,
};

/// Function call expression.
#[derive(Debug, Clone)]
pub struct FnCall {
    name: Ident,
    args: Vec<Expr>,
}

impl FnCall {
    pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
        (
            delimited_multispace0(Ident::parse_ident),
            delimited(
                '(',
                // Arguments (expressions) separated by commas
                alt((
                    delimited_multispace0(separated(1.., Expr::parse, delimited_multispace0(','))),
                    cut_err(fail).context(exp_desc!("one or more arguments")),
                )),
                alt((
                    // Trailing comma before closing ')'
                    preceded(
                        delimited_multispace0(','),
                        cut_err(')')
                            .context(exp_desc!("an expression"))
                            .context(exp_char!(')')),
                    ),
                    // No trailing comma
                    cut_err(')').context(exp_char!(',')).context(exp_char!(')')),
                )),
            ),
        )
            .context(label!("function call"))
            .map(|(ident, args)| Self { name: ident, args })
            .map(Expr::FnCall)
            .parse_next(input)
    }

    pub(crate) fn evaluate(self, state: &mut EvalState) -> Value {
        if let Some(Expr::Fn(function)) = state.get_expr(&self.name) {
            match function {
                Fn::Defined(declared) => {
                    return declared.to_owned().evaluate(state, self.args);
                }
                Fn::BuiltIn(func) => return func(state, self.args),
            }
        }
        Value::Null
    }

    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        let pretty = state.pretty();
        self.name.format(writer, state)?;

        write!(writer, "(")?;

        let mut args_iter = self.args.iter().peekable();

        while let Some(arg) = args_iter.next() {
            arg.format(writer, state.indented())?;

            if args_iter.peek().is_some() {
                write!(writer, ",")?;
                if pretty {
                    write!(writer, " ")?;
                }
            }
        }

        write!(writer, ")")
    }
}
