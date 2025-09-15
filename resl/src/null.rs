use winnow::{
    ModalResult, Parser,
    ascii::Caseless,
    combinator::{alt, cut_err, fail, peek, preceded},
};

use crate::{StatefulInput, expr::Expr, macros::exp_desc};

pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
    alt((
        "null".value(Expr::Null),
        // Fail if "null" is not correctly cased
        preceded(
            peek(Caseless("null")),
            cut_err(fail).context(exp_desc!("null")),
        ),
    ))
    .parse_next(input)
}
