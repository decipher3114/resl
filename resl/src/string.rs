use winnow::{
    ModalResult, Parser,
    ascii::escaped,
    combinator::{alt, cut_err, delimited, opt},
    token::take_while,
};

use crate::{
    StatefulInput,
    expr::Expr,
    macros::{exp_char, label},
};

pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
    delimited(
        '"',
        // Parse the content of the string, allowing for escaped characters
        opt(escaped(
            take_while(1.., |c: char| !['\\', '\"', '\n'].contains(&c)),
            '\\',
            alt(('\\'.value("\\"), '"'.value("\""), '\n'.value("\n"))),
        ))
        // If there are no characters between the quotes, return an empty string
        .map(Option::unwrap_or_default),
        cut_err('"').context(exp_char!('"')),
    )
    .context(label!("string"))
    .map(Expr::Str)
    .parse_next(input)
}

pub(crate) fn parse_plain<'input>(
    input: &mut StatefulInput<'input, '_>,
) -> ModalResult<&'input str> {
    take_while(1.., |c: char| {
        c.is_alphanumeric() || ['_', '-', '$'].contains(&c)
    })
    .parse_next(input)
}
