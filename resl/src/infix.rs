use winnow::{
    ModalResult, Parser,
    combinator::{alt, cut_err, delimited, opt},
};

use crate::{
    StatefulInput,
    block::Block,
    boolean,
    expr::Expr,
    fn_call::FnCall,
    ident::Ident,
    index::Index,
    macros::{exp_desc, exp_str, label},
    number,
    prefix::PrefixOp,
    state::{EvalState, FmtState},
    string,
    utils::delimited_multispace0,
    value::Value,
};

/// Binary infix operation.
#[derive(Debug, Clone)]
pub struct InfixOp {
    lhs: Box<Expr>,
    op: Op,
    rhs: Box<Expr>,
    parenthesized: bool,
}

/// Infix operator categories.
#[derive(Debug, Clone, Copy)]
enum Op {
    Arithmetic(ArithmeticOp),
    Logic(LogicOp),
    Comparison(ComparisonOp),
}

/// Arithmetic operators.
#[derive(Debug, Clone, Copy)]
enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// Logical operators.
#[derive(Debug, Clone, Copy)]
enum LogicOp {
    And,
    Or,
}

/// Comparison operators.
#[derive(Debug, Clone, Copy)]
enum ComparisonOp {
    Eq,
    NotEq,
    Gt,
    Lt,
    GtOrEq,
    LtOrEq,
}

impl InfixOp {
    // Parse valid operands for infix expressions, avoiding infinite recursion
    fn parse_operand(input: &mut StatefulInput) -> ModalResult<Expr> {
        alt((
            string::parse,
            number::parse,
            // Index and FnCall have to be parsed before Ident
            // because they have ident as their first parser
            Index::parse,
            FnCall::parse,
            Ident::parse,
            // Boolean must be parsed after Ident
            // Ident already discovers true/false as identifiers
            boolean::parse,
            Block::parse,
            PrefixOp::parse,
            InfixOp::parse_parenthesized,
        ))
        .parse_next(input)
    }

    fn parse_operator(input: &mut StatefulInput) -> ModalResult<Op> {
        alt((
            "+".value(Op::Arithmetic(ArithmeticOp::Add)),
            "-".value(Op::Arithmetic(ArithmeticOp::Sub)),
            "*".value(Op::Arithmetic(ArithmeticOp::Mul)),
            "/".value(Op::Arithmetic(ArithmeticOp::Div)),
            "&&".value(Op::Logic(LogicOp::And)),
            "||".value(Op::Logic(LogicOp::Or)),
            "==".value(Op::Comparison(ComparisonOp::Eq)),
            "!=".value(Op::Comparison(ComparisonOp::NotEq)),
            ">=".value(Op::Comparison(ComparisonOp::GtOrEq)),
            "<=".value(Op::Comparison(ComparisonOp::LtOrEq)),
            ">".value(Op::Comparison(ComparisonOp::Gt)),
            "<".value(Op::Comparison(ComparisonOp::Lt)),
        ))
        .context(exp_str!("+"))
        .context(exp_str!("-"))
        .context(exp_str!("*"))
        .context(exp_str!("/"))
        .context(exp_str!("&&"))
        .context(exp_str!("||"))
        .context(exp_str!("=="))
        .context(exp_str!("!="))
        .context(exp_str!(">="))
        .context(exp_str!("<="))
        .context(exp_str!(">"))
        .context(exp_str!("<"))
        .parse_next(input)
    }

    pub(crate) fn parse_parenthesized(input: &mut StatefulInput) -> ModalResult<Expr> {
        delimited(
            '(',
            (
                // Right operand can be another infix expression
                cut_err(Self::parse_operand.map(Box::new)).context(exp_desc!("operand")),
                delimited_multispace0(cut_err(Self::parse_operator)),
                // Right operand can be another infix expression
                cut_err(Self::parse.map(Box::new)).context(exp_desc!("operand")),
            ),
            ')',
        )
        .context(label!("infix expression"))
        .map(|(lhs, op, rhs)| InfixOp {
            lhs,
            op,
            rhs,
            parenthesized: true,
        })
        .map(Expr::InfixOp)
        .parse_next(input)
    }

    pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Expr> {
        alt((
            // Non-parenthesized operation or single operand
            (
                Self::parse_operand,
                // Optional operator and right operand
                opt((
                    delimited_multispace0(Self::parse_operator),
                    // Right operand can be another infix expression
                    cut_err(Self::parse).context(exp_desc!("operand")),
                ))
                .context(label!("infix expression")),
            )
                .map(|(lhs, rest)| match rest {
                    Some((op, rhs)) => Expr::InfixOp(InfixOp {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                        parenthesized: false,
                    }),
                    None => lhs,
                }),
            // Parenthesized infix operation
            Self::parse_parenthesized,
        ))
        .parse_next(input)
    }

    pub(crate) fn evaluate(self, state: &mut EvalState) -> Value {
        let lhs_value = self.lhs.evaluate(state);
        let rhs_value = self.rhs.evaluate(state);

        match lhs_value {
            Value::Integer(int_lhs) => match self.op {
                Op::Arithmetic(math_op) => {
                    let int_rhs = match rhs_value {
                        Value::Integer(int) => int,
                        Value::Float(dec) => dec.round() as i64,
                        _ => return Value::Null,
                    };
                    return Value::Integer(handle_math_ops(math_op, int_lhs, int_rhs));
                }
                Op::Comparison(comp_op) => {
                    let int_rhs = match rhs_value {
                        Value::Integer(int) => int,
                        Value::Float(dec) => dec.round() as i64,
                        _ => return Value::Null,
                    };
                    return match comp_op {
                        ComparisonOp::Eq => Value::Boolean(int_lhs == int_rhs),
                        ComparisonOp::NotEq => Value::Boolean(int_lhs != int_rhs),
                        ComparisonOp::Gt => Value::Boolean(int_lhs > int_rhs),
                        ComparisonOp::Lt => Value::Boolean(int_lhs < int_rhs),
                        ComparisonOp::GtOrEq => Value::Boolean(int_lhs >= int_rhs),
                        ComparisonOp::LtOrEq => Value::Boolean(int_lhs <= int_rhs),
                    };
                }
                _ => (),
            },
            Value::Float(dec_lhs) => match self.op {
                Op::Arithmetic(math_op) => {
                    let dec_rhs = match rhs_value {
                        Value::Integer(int) => int as f64,
                        Value::Float(dec) => dec,
                        _ => return Value::Null,
                    };
                    return Value::Float(handle_math_ops(math_op, dec_lhs, dec_rhs));
                }
                Op::Comparison(comp_op) => {
                    let r_float = match rhs_value {
                        Value::Integer(r_int) => r_int as f64,
                        Value::Float(r_float) => r_float,
                        _ => return Value::Null,
                    };
                    return match comp_op {
                        ComparisonOp::Eq => Value::Boolean(dec_lhs == r_float),
                        ComparisonOp::NotEq => Value::Boolean(dec_lhs != r_float),
                        ComparisonOp::Gt => Value::Boolean(dec_lhs > r_float),
                        ComparisonOp::Lt => Value::Boolean(dec_lhs < r_float),
                        ComparisonOp::GtOrEq => Value::Boolean(dec_lhs >= r_float),
                        ComparisonOp::LtOrEq => Value::Boolean(dec_lhs <= r_float),
                    };
                }
                _ => {}
            },
            Value::Boolean(l_bool) => {
                if let Op::Logic(logical_op) = self.op {
                    let r_bool = match rhs_value {
                        Value::Boolean(r_bool) => r_bool,
                        _ => return Value::Null,
                    };

                    return Value::Boolean(handle_logical_ops(logical_op, l_bool, r_bool));
                }
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
        if self.parenthesized {
            write!(writer, "(")?;
        }

        self.lhs.format(writer, state)?;
        if state.pretty() {
            write!(writer, " ")?;
        }
        self.op.format(writer)?;
        if state.pretty() {
            write!(writer, " ")?;
        }
        self.rhs.format(writer, state)?;

        if self.parenthesized {
            write!(writer, ")")?;
        }
        Ok(())
    }
}

impl Op {
    pub(crate) fn format<W: std::fmt::Write>(&self, f: &mut W) -> std::fmt::Result {
        let s = match self {
            Op::Arithmetic(math_operator) => match math_operator {
                ArithmeticOp::Add => "+",
                ArithmeticOp::Sub => "-",
                ArithmeticOp::Mul => "*",
                ArithmeticOp::Div => "/",
            },
            Op::Logic(logical_operator) => match logical_operator {
                LogicOp::And => "&&",
                LogicOp::Or => "||",
            },
            Op::Comparison(comparison_operator) => match comparison_operator {
                ComparisonOp::Eq => "==",
                ComparisonOp::NotEq => "!=",
                ComparisonOp::Gt => ">",
                ComparisonOp::Lt => "<",
                ComparisonOp::GtOrEq => ">=",
                ComparisonOp::LtOrEq => "<=",
            },
        };
        write!(f, "{s}")
    }
}

fn handle_math_ops<Num>(op: ArithmeticOp, lhs: Num, rhs: Num) -> Num
where
    Num: std::ops::Add<Output = Num>
        + std::ops::Sub<Output = Num>
        + std::ops::Mul<Output = Num>
        + std::ops::Div<Output = Num>
        + Default
        + PartialEq,
{
    match op {
        ArithmeticOp::Add => lhs + rhs,
        ArithmeticOp::Sub => lhs - rhs,
        ArithmeticOp::Mul => lhs * rhs,
        ArithmeticOp::Div => {
            if rhs == Num::default() {
                Num::default()
            } else {
                lhs / rhs
            }
        }
    }
}

fn handle_logical_ops(op: LogicOp, lhs: bool, rhs: bool) -> bool {
    match op {
        LogicOp::And => lhs && rhs,
        LogicOp::Or => lhs || rhs,
    }
}
