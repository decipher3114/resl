use winnow::{ModalResult, Parser, combinator::alt};

use crate::{
    StatefulInput,
    expr::Expr,
    macros::label,
    state::{EvalState, FmtState},
    utils::delimited_multispace0,
    value::Value,
};

/// Unary prefix operation.
#[derive(Debug, Clone)]
pub struct PrefixOp {
    pub(crate) op: Op,
    pub(crate) operand: Box<Expr>,
}

/// Unary prefix operators.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Op {
    Negate,
    Not,
}

impl PrefixOp {
    pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
        (
            delimited_multispace0(alt(('-'.value(Op::Negate), '!'.value(Op::Not)))),
            Expr::require_parse.map(Box::new),
        )
            .context(label!("prefix operation"))
            .map(|(op, operand)| PrefixOp { op, operand })
            .map(Expr::PrefixOp)
            .parse_next(input)
    }

    pub(crate) fn compute(self, state: &mut EvalState) -> Value {
        let value = self.operand.evaluate(state);

        match value {
            Value::Integer(int) if self.op == Op::Negate => {
                return Value::Integer(-int);
            }
            Value::Float(float) if self.op == Op::Negate => {
                return Value::Float(-float);
            }
            Value::Boolean(bool) if self.op == Op::Not => {
                return Value::Boolean(!bool);
            }
            _ => {}
        }

        Value::Null
    }

    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        self.op.format(writer)?;
        if state.pretty() {
            write!(writer, " ")?;
        }
        self.operand.format(writer, state)
    }
}

impl From<(char, Expr)> for PrefixOp {
    fn from((operator, operand): (char, Expr)) -> Self {
        PrefixOp {
            op: Op::from(operator),
            operand: Box::new(operand),
        }
    }
}

impl Op {
    pub(crate) fn format<W: std::fmt::Write>(&self, f: &mut W) -> std::fmt::Result {
        match self {
            Op::Negate => write!(f, "-"),
            Op::Not => write!(f, "!"),
        }
    }
}

impl From<char> for Op {
    fn from(op: char) -> Self {
        match op {
            '!' => Op::Not,
            '-' => Op::Negate,
            _ => unreachable!("Parser should ensure valid operator"),
        }
    }
}
