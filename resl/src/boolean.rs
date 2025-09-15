use winnow::{
    ModalResult, Parser,
    ascii::Caseless,
    combinator::{alt, cut_err, fail, peek, preceded},
};

use crate::{
    Expr, StatefulInput,
    macros::{exp_str, label},
};

pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
    alt((
        alt((
            "true",
            // Fail if "true" is not correctly cased
            preceded(
                peek(Caseless("true")),
                cut_err(fail).context(exp_str!("true")),
            ),
        ))
        .value(true),
        alt((
            "false",
            // Fail if "false" is not correctly cased
            preceded(
                peek(Caseless("false")),
                cut_err(fail).context(exp_str!("false")),
            ),
        ))
        .value(false),
    ))
    .context(label!("boolean"))
    .map(Expr::Bool)
    .parse_next(input)
}
