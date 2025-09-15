use winnow::{
    ModalResult, Parser,
    ascii::digit1,
    combinator::{cut_err, opt},
};

use crate::{
    StatefulInput,
    expr::Expr,
    macros::{exp_desc, label},
};

pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
    (
        // An optional leading minus sign
        opt('-'),
        (
            // Integral part
            digit1,
            // Fractional part
            opt((
                '.',
                // Require at least one digit after the decimal point
                cut_err(digit1).context(exp_desc!("fractional part")),
            )),
        ),
    )
        .take()
        .context(label!("decimal"))
        .map(|string: &str| {
            if string.contains('.') {
                Expr::Float(string.parse::<f64>().unwrap())
            } else {
                Expr::Int(string.parse::<i64>().unwrap())
            }
        })
        .parse_next(input)
}
