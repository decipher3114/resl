use winnow::{ModalResult, Parser};

use crate::{
    StatefulInput,
    expr::Expr,
    function::defined::Defined,
    state::{EvalState, FmtState},
    value::Value,
};

pub(crate) mod builtin;
pub(crate) mod defined;

/// Function expression (declared or built-in).
#[derive(Debug, Clone)]
pub enum Fn {
    Defined(Defined),
    BuiltIn(fn(&mut EvalState, Vec<Expr>) -> Value),
}

impl Fn {
    pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
        Defined::parse
            .map(Self::Defined)
            .map(Expr::Fn)
            .parse_next(input)
    }

    pub(crate) fn evaluate(self, _state: &mut EvalState) -> Value {
        Value::Null
    }

    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        match self {
            Fn::Defined(declaration) => declaration.format(writer, state),
            Fn::BuiltIn(_) => write!(writer, "<built-in function>"),
        }
    }
}
