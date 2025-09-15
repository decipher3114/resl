use winnow::{
    ModalResult, Parser,
    combinator::{alt, cut_err, delimited, preceded, separated},
};

use crate::{
    StatefulInput,
    expr::Expr,
    macros::{exp_char, exp_desc, label},
    state::{EvalState, FmtState},
    utils::{delimited_multispace0, write_indent},
    value::Value,
};

/// List of expressions.
pub(crate) type List = Vec<Expr>;

pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
    delimited(
        '[',
        delimited_multispace0(separated(0.., Expr::parse, delimited_multispace0(','))),
        alt((
            // Trailing comma before closing ']'
            preceded(
                delimited_multispace0(','),
                cut_err(']')
                    .context(exp_desc!("an expression"))
                    .context(exp_char!(']')),
            ),
            // No trailing comma
            cut_err(']').context(exp_char!(',')).context(exp_char!(']')),
        )),
    )
    .context(label!("list"))
    .map(Expr::List)
    .parse_next(input)
}

pub(crate) fn evaluate(list: List, state: &mut EvalState) -> Value {
    Value::List(list.into_iter().map(|expr| expr.evaluate(state)).collect())
}

pub(crate) fn format<W: std::fmt::Write>(
    list: &List,
    writer: &mut W,
    state: FmtState,
) -> std::fmt::Result {
    let pretty = state.pretty();

    write!(writer, "[")?;

    if list.is_empty() {
        write!(writer, "]")?;
        return Ok(());
    }

    if pretty {
        writeln!(writer)?;
        write_indent(writer, state.indented().indent_level())?;
    }

    let mut list_iter = list.iter().peekable();

    while let Some(expr) = list_iter.next() {
        expr.format(writer, state.indented())?;

        if list_iter.peek().is_some() {
            write!(writer, ",")?;
            if pretty {
                writeln!(writer)?;
                write_indent(writer, state.indented().indent_level())?;
            } else {
                write!(writer, " ")?;
            }
        }
    }

    if pretty {
        writeln!(writer)?;
        write_indent(writer, state.indent_level())?;
    }

    write!(writer, "]")
}
