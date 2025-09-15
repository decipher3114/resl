use winnow::{
    ModalResult, Parser,
    combinator::{alt, cut_err, delimited, preceded, separated, separated_pair},
};

use crate::{
    StatefulInput,
    expr::Expr,
    macros::{exp_char, exp_desc, label},
    state::{EvalState, FmtState},
    string,
    utils::{delimited_multispace0, write_indent},
    value::Value,
};

/// Map of key-expression pairs.
#[cfg(not(feature = "preserve-order"))]
pub(crate) type Map = std::collections::BTreeMap<String, Expr>;
#[cfg(feature = "preserve-order")]
pub(crate) type Map = indexmap::IndexMap<String, Expr>;

pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
    delimited(
        '[',
        delimited_multispace0(separated(
            1..,
            separated_pair(
                delimited('"', string::parse_plain.map(str::to_string), '"'),
                delimited_multispace0(':'),
                Expr::require_parse,
            ),
            delimited_multispace0(','),
        ))
        .map(|list: Vec<(String, Expr)>| list.into_iter().collect::<Map>()),
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
    .context(label!("map"))
    .map(Expr::Map)
    .parse_next(input)
}

pub(crate) fn evaluate(map: Map, state: &mut EvalState) -> Value {
    Value::Map(
        map.into_iter()
            .map(|(key, expr)| (key, expr.evaluate(state)))
            .collect(),
    )
}

pub(crate) fn format<W: std::fmt::Write>(
    map: &Map,
    writer: &mut W,
    state: FmtState,
) -> std::fmt::Result {
    let pretty = state.pretty();

    write!(writer, "[")?;

    if map.is_empty() {
        write!(writer, "]")?;
        return Ok(());
    }

    if pretty {
        writeln!(writer)?;
        write_indent(writer, state.indented().indent_level())?;
    };

    let mut map_iter = map.iter().peekable();
    while let Some((key, expr)) = map_iter.next() {
        write!(writer, "\"{key}\": ")?;
        expr.format(writer, state.indented())?;
        if map_iter.peek().is_some() {
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
