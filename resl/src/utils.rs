use winnow::{Parser, ascii::multispace0, combinator::delimited};

#[inline(always)]
pub(crate) fn delimited_multispace0<I, O, E>(parser: impl Parser<I, O, E>) -> impl Parser<I, O, E>
where
    I: winnow::stream::StreamIsPartial + winnow::stream::Stream,
    <I as winnow::stream::Stream>::Token: winnow::stream::AsChar + Clone,
    E: winnow::error::ParserError<I>,
{
    delimited(multispace0, parser, multispace0)
}

pub(crate) fn write_indent<W: std::fmt::Write>(
    writer: &mut W,
    indent_level: usize,
) -> std::fmt::Result {
    for _ in 0..indent_level {
        write!(writer, "    ")?;
    }
    Ok(())
}
