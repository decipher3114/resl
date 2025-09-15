use winnow::{
    ModalResult, Parser,
    combinator::{alt, cut_err, delimited, fail, opt, preceded, repeat},
};

use crate::{
    StatefulInput,
    expr::Expr,
    ident::Ident,
    macros::{exp_char, exp_desc, label},
    state::{EvalState, FmtState},
    utils::delimited_multispace0,
    value::Value,
};

/// Index operation for element access.
#[derive(Debug, Clone)]
pub struct Index {
    base: Ident,
    indices: Vec<IndexType>,
}

/// Type of indexing operation.
#[derive(Debug, Clone)]
pub(crate) enum IndexType {
    /// Single element access.
    Single(Expr),
    /// Range access with bounds.
    Range(RangeBounds),
}

/// Range bounds for range access operations.
#[derive(Debug, Clone)]
pub(crate) enum RangeBounds {
    /// Range starting from an index to the end.
    StartingFrom(Expr),
    /// Range from the beginning to an index.
    EndingAt(Expr),
    /// Range from one index to another.
    FromTo(Expr, Expr),
}

impl Index {
    pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
        (
            Ident::parse_ident,
            repeat(
                1..,
                preceded(
                    '[',
                    alt((
                        // For cases:
                        // - `x[0]`
                        // - `x[0:]`
                        // - `x[0:1]`
                        (
                            Expr::parse,
                            alt((
                                delimited(
                                    ':',
                                    delimited_multispace0(
                                        // Optional end expression for case:
                                        // - `x[0:1]`
                                        // - `x[0:]`
                                        opt(Expr::parse),
                                    ),
                                    // Require closing ']'
                                    cut_err(']')
                                        .context(exp_desc!("an expression"))
                                        .context(exp_char!(']')),
                                )
                                .map(Some),
                                // Accept immediate closing ']' for case:
                                // - `x[0]
                                ']'.value(None),
                                // Fail otherwise
                                cut_err(fail)
                                    .context(exp_char!(':'))
                                    .context(exp_char!(']')),
                            )),
                        )
                            .map(
                                |(start, end): (Expr, Option<Option<Expr>>)| match end {
                                    Some(end) => match end {
                                        Some(end) => {
                                            IndexType::Range(RangeBounds::FromTo(start, end))
                                        }
                                        None => IndexType::Range(RangeBounds::StartingFrom(start)),
                                    },
                                    None => IndexType::Single(start),
                                },
                            ),
                        // For case:
                        // - `x[:5]`
                        delimited(
                            ':',
                            // Required expression for range end
                            Expr::require_parse,
                            // Peek for closing ']', otherwise fail
                            cut_err(']').context(exp_char!(']')),
                        )
                        .map(|end| IndexType::Range(RangeBounds::EndingAt(end))),
                        // For case:
                        // - `x[]`
                        cut_err(fail)
                            .context(exp_desc!("An expression"))
                            .context(exp_char!(':')),
                    )),
                ),
            ),
        )
            .context(label!("index expression"))
            .map(|(ident, indices): (Ident, Vec<IndexType>)| Self {
                base: ident,
                indices,
            })
            .map(Expr::Index)
            .parse_next(input)
    }

    pub(crate) fn evaluate(self, state: &mut EvalState) -> Value {
        let Some(base_value) = self.base.evaluate(state) else {
            return Value::Null;
        };

        let mut base_value = if matches!(base_value, Value::Map(_) | Value::List(_)) {
            base_value.clone()
        } else {
            return Value::Null;
        };

        for index in self.indices {
            match index {
                IndexType::Single(index_expr) => {
                    let index_value = index_expr.evaluate(state);
                    match index_value {
                        // If the index is a string, try to get from map
                        Value::String(string) => {
                            if let Value::Map(map) = base_value {
                                base_value = map.get(&string).cloned().unwrap_or_default();
                            } else {
                                return Value::Null;
                            }
                        }
                        // If the index is a non-negative integer, try to get from list
                        Value::Integer(int) if int >= 0 => {
                            if let Value::List(list) = base_value {
                                base_value = list.get(int as usize).cloned().unwrap_or_default();
                            } else {
                                return Value::Null;
                            }
                        }
                        _ => {
                            return Value::Null;
                        }
                    }
                }
                IndexType::Range(range_bounds) => {
                    if let Value::List(list) = base_value {
                        let range = match range_bounds {
                            RangeBounds::StartingFrom(index) => {
                                let Some(start) = expr_to_usize(index, state) else {
                                    return Value::Null;
                                };
                                if start >= list.len() {
                                    return Value::Null;
                                }
                                start..list.len()
                            }
                            RangeBounds::EndingAt(end) => {
                                let Some(end) = expr_to_usize(end, state) else {
                                    return Value::Null;
                                };
                                if end > list.len() {
                                    return Value::Null;
                                }
                                0..end
                            }
                            RangeBounds::FromTo(start, end) => {
                                let Some(start) = expr_to_usize(start, state) else {
                                    return Value::Null;
                                };
                                let Some(end) = expr_to_usize(end, state) else {
                                    return Value::Null;
                                };

                                if start > end || start >= list.len() || end > list.len() {
                                    return Value::Null;
                                }

                                start..end
                            }
                        };

                        base_value = list
                            .get(range)
                            .map(|slice| slice.to_vec())
                            .map(Value::List)
                            .unwrap_or_default();
                    }
                }
            }
        }

        base_value
    }

    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        self.base.format(writer, state)?;

        for index in self.indices.iter() {
            match index {
                IndexType::Single(index) => {
                    write!(writer, "[")?;
                    index.format(writer, state)?;
                    write!(writer, "]")?;
                }
                IndexType::Range(range_bounds) => {
                    write!(writer, "[")?;
                    match range_bounds {
                        RangeBounds::StartingFrom(start) => {
                            start.format(writer, state)?;
                            write!(writer, ":")?;
                        }
                        RangeBounds::EndingAt(end) => {
                            write!(writer, ":")?;
                            end.format(writer, state)?;
                        }
                        RangeBounds::FromTo(start, end) => {
                            start.format(writer, state)?;
                            write!(writer, ":")?;
                            end.format(writer, state)?;
                        }
                    }
                    write!(writer, "]")?;
                }
            }
        }

        Ok(())
    }
}

fn expr_to_usize(expr: Expr, state: &mut EvalState) -> Option<usize> {
    match expr.evaluate(state) {
        Value::Integer(int) if int >= 0 => Some(int as usize),
        _ => None,
    }
}
