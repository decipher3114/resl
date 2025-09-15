use winnow::{
    LocatingSlice, ModalResult, Parser,
    combinator::{alt, cut_err, eof, fail, terminated},
};

use crate::{
    StatefulInput,
    block::Block,
    error::ParseError,
    fn_call::FnCall,
    for_each::ForEach,
    function::Fn,
    ident::Ident,
    if_else::IfElse,
    index::Index,
    infix::InfixOp,
    list::{self, List},
    macros::{exp_desc, label},
    map::{self, Map},
    null,
    prefix::PrefixOp,
    state::{CtxState, EvalState, FmtState, ParseState},
    utils::delimited_multispace0,
    value::Value,
};

/// Represents all possible expressions in the RESL language.
///
/// Expressions are the building blocks of RESL programs. Every construct in RESL
/// is an expression that evaluates to a `Value`. This enum encompasses all syntax
/// elements from literals to complex operations and control structures.
#[derive(Debug, Default, Clone)]
pub enum Expr {
    /// The null value literal.
    ///
    /// Examples: `null`
    #[default]
    Null,
    /// String literals with double quotes.
    ///
    /// Examples: `"hello"`, `"Line 1\nLine 2"`
    Str(String),
    /// 64-bit signed integer literals.
    ///
    /// Examples: `42`, `-17`
    Int(i64),
    /// 64-bit floating-point literals.
    ///
    /// Examples: `3.14`, `-2.5`
    Float(f64),
    /// Boolean literals.
    ///
    /// Examples: `true`, `false`
    Bool(bool),
    /// Ordered collections of expressions.
    ///
    /// Examples: `[1, 2, 3]`, `["a", "b"]`
    List(List),
    /// Key-value associations with string keys.
    ///
    /// Examples: `["name": "Alice", "age": 30]`
    Map(Map),
    /// Variable and function name references.
    ///
    /// Examples: `variable_name`, `my_function`
    Ident(Ident),
    /// Element access operations supporting single elements and ranges.
    /// Supports single element access and range slicing with various bounds.
    ///
    /// Examples: `list[0]`, `map["key"]`, `list[1:3]`, `list[2:]`, `list[:5]`
    Index(Index),
    /// Binary operations between two expressions.
    /// Supports arithmetic (`+`, `-`, `*`, `/`), logical (`&&`, `||`),
    /// and comparison (`==`, `!=`, `>`, `<`, `>=`, `<=`) operators.
    ///
    /// Examples: `a + b`, `x && y`
    InfixOp(InfixOp),
    /// Unary operations on single expressions.
    /// Supports numerical negation (`-`) and logical NOT (`!`).
    ///
    /// Examples: `-x`, `!flag`
    PrefixOp(PrefixOp),
    /// Scoped expressions with local variables.
    /// The last expression becomes the block's return value.
    ///
    /// Examples: `{x = 5; y = x * 2; y}`
    Block(Block),
    /// Ternary if-else expressions.
    /// Evaluates if_expr and returns then_expr or else_expr.
    ///
    /// Examples: `? condition : "yes" | "no"`
    IfElse(IfElse),
    /// For-each loops over lists or maps.
    ///
    /// Examples: `x > (k, v) : concat(k, v)` or `i > (index, item) : item * 2`
    ForEach(ForEach),
    /// Function definitions with parameters.
    ///
    /// Examples: `|a, b| (a + b)`
    Fn(Fn),
    /// Function invocations with arguments.
    ///
    /// Examples: `function_name(arg1, arg2)`
    FnCall(FnCall),
}

impl Expr {
    /// Parses an expression from the input stream.
    pub(crate) fn parse(input: &mut StatefulInput) -> ModalResult<Self> {
        delimited_multispace0(alt((
            // This must be before InfixOp parser because it starts with an Ident
            ForEach::parse,
            // This InfixOp parser includes all the remaining exprs.
            // It will short-circuit in-case no op is found
            // Parses:
            // - Str
            // - Int
            // - Float
            // - Bool
            // - Ident
            // - Index
            // - FnCall
            // - PrefixOp
            // - Block
            // - InfixOp (if operators found)
            InfixOp::parse,
            null::parse,
            map::parse,
            list::parse,
            IfElse::parse,
            fail.context(label!("expression"))
                .context(exp_desc!("a valid expression")),
        )))
        .parse_next(input)
    }

    /// Parses an expression with a requirement that one must be present.
    pub(crate) fn require_parse(input: &mut StatefulInput) -> ModalResult<Self> {
        alt((
            Self::parse,
            cut_err(fail).context(exp_desc!("an expression")),
        ))
        .parse_next(input)
    }

    /// Parses a RESL expression from a string input with a given context state.
    /// This consumes the entire input and returns an error if any unparsed input remains.
    pub(crate) fn parse_all(input: &str, ctx_state: &mut CtxState) -> Result<Self, ParseError> {
        let input = StatefulInput {
            input: LocatingSlice::new(input),
            state: ParseState::new(ctx_state),
        };

        let expr = terminated(
            Self::parse,
            // Ensure the entire input is consumed.
            eof.context(label!("expression"))
                .context(exp_desc!("end of input")),
        )
        .parse(input)?;

        Ok(expr)
    }

    /// Evaluates the expression and returns the computed value.
    pub(crate) fn evaluate(self, state: &mut EvalState) -> Value {
        match self {
            Self::Null => Value::Null,
            Self::Str(s) => Value::String(s),
            Self::Int(i) => Value::Integer(i),
            Self::Float(f) => Value::Float(f),
            Self::Bool(b) => Value::Boolean(b),
            Self::List(list) => list::evaluate(list, state),
            Self::Map(map) => map::evaluate(map, state),
            Self::Ident(ident) => ident.evaluate(state).cloned().unwrap_or_default(),
            Self::Index(index) => index.evaluate(state),
            Self::InfixOp(infix_op) => infix_op.evaluate(state),
            Self::PrefixOp(prefix_op) => prefix_op.compute(state),
            Self::Block(block) => block.evaluate(state),
            Self::IfElse(if_else) => if_else.evaluate(state),
            Self::ForEach(for_each) => for_each.evaluate(state),
            Self::Fn(function) => function.evaluate(state),
            Self::FnCall(fn_call) => fn_call.evaluate(state),
        }
    }

    /// Determines if the expression should be cached after evaluation.
    pub(crate) fn should_be_cached(&self) -> bool {
        !matches!(self, Self::Fn(_))
    }

    /// Formats the expression to a writer with specified formatting state.
    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        state: FmtState,
    ) -> std::fmt::Result {
        match self {
            Self::Null => write!(writer, "null"),
            Self::Str(s) => write!(writer, "\"{}\"", s),
            Self::Int(i) => write!(writer, "{}", i),
            Self::Float(f) => {
                if f.fract() == 0.0 {
                    write!(writer, "{:.1}", f)
                } else {
                    write!(writer, "{}", f)
                }
            }
            Self::Bool(b) => write!(writer, "{}", b),
            Self::List(list) => list::format(list, writer, state),
            Self::Map(map) => map::format(map, writer, state),
            Self::Ident(ident) => ident.format(writer, state),
            Self::Index(index) => index.format(writer, state),
            Self::InfixOp(infix_op) => infix_op.format(writer, state),
            Self::PrefixOp(prefix_op) => prefix_op.format(writer, state),
            Self::Block(block) => block.format(writer, state),
            Self::IfElse(if_else) => if_else.format(writer, state),
            Self::ForEach(for_each) => for_each.format(writer, state),
            Self::Fn(func) => func.format(writer, state),
            Self::FnCall(fn_call) => fn_call.format(writer, state),
        }
    }
}
