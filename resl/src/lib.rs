//! # RESL
//!
//! A modern configuration and serialization language with variables, expressions, and dynamic runtime evaluation.
//!
//! **RESL** = **R**untime **E**valuated **S**erialization **L**anguage
//!
//! RESL enables dynamic configuration files with variables, expressions,
//! conditionals, and computed values for flexible configuration management.
//!
//! ## Quick Start
//!
//! ```rust
//! use resl::evaluate;
//!
//! // Simple evaluation
//! let result = evaluate("5 + 3").unwrap();
//! println!("{}", result); // Outputs: 8
//!
//! // Configuration with variables and logic
//! let config = r#"
//!     {
//!         port = 8080;
//!         host = "localhost";
//!         debug = true;
//!         url = concat("http://", host, ":", port);
//!         env = ? debug : "development" | "production";
//!         ["url": url, "environment": env]
//!     }
//! "#;
//! let result = evaluate(config).unwrap();
//! ```
//!
//! ## Key Features
//!
//! - **Variables & References**: Define variables and reference them directly by name
//! - **Function Declaration & Calls**: Define and call functions with parameter passing
//! - **Binary Operations**: Perform arithmetic, logical, and comparison operations
//! - **Conditional Logic**: Use ternary operators `? condition : then | else`
//! - **Rich Data Types**: Support for strings, numbers, booleans, lists, and maps
//! - **Block Expressions**: Group statements and computations in `{}` blocks
//! - **Array/Object Access**: Index into collections with `[key]` syntax and range slicing
//! - **Flexible Structure**: Top-level can be any expression, not just objects
//!
//! ## Installation
//!
//! Add RESL to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! resl = "0.1"
//! ```
//!
//! ## Documentation
//!
//! For comprehensive documentation, examples, and guides, visit:
//! **[https://decipher3114.github.io/resl](https://decipher3114.github.io/resl)**
//!
//! The documentation includes:
//! - Complete syntax guide with examples
//! - Language bindings for C/C++ and other languages
//! - CLI usage and tools
//! - Best practices and patterns
//! - Comparison with other configuration formats

mod block;
mod boolean;
mod expr;
mod fn_call;
mod for_each;
mod function;
mod ident;
mod if_else;
mod index;
mod infix;
mod list;
mod map;
mod null;
mod number;
mod prefix;
mod string;

mod binding;
mod context;
mod error;
mod macros;
mod state;
mod utils;
mod value;

pub use error::ParseError;
pub use expr::Expr;
pub use value::Value;

pub use crate::state::{CtxState, EvalState, FmtState, ParseState};

// use crate::{
//     expression::Expression,
//     state::{CtxState, EvalState, FmtState},
// };

type StatefulInput<'input, 'state> =
    winnow::Stateful<winnow::LocatingSlice<&'input str>, state::ParseState<'state>>;

/// Parses a RESL expression from a string and formats it to a writer.
///
/// This utility function combines parsing and formatting in one operation. It's useful
/// for reformatting, pretty-printing, or validating RESL expressions without needing
/// to handle the intermediate parsed form.
///
/// # Arguments
///
/// * `input` - The RESL expression string to parse and format
/// * `writer` - A mutable reference to any type implementing `Write`
/// * `pretty` - Whether to format the output for readability (with indentation and spacing)
///
/// # Returns
///
/// * `Ok(())` - If parsing and formatting succeed
/// * `Err(ParseError)` - If parsing fails or write operation fails
///
/// # Examples
///
/// ```
/// use resl::format;
///
/// let mut output = String::new();
/// format("{x=5;x*2}", &mut output, true).unwrap();
/// ```
pub fn format<W: std::fmt::Write>(
    input: &str,
    writer: &mut W,
    pretty: bool,
) -> Result<(), ParseError> {
    let mut ctx_state = CtxState::new();

    let expression = Expr::parse_all(input, &mut ctx_state)?;

    // For now, ignore IO errors since they're less common than parse errors
    let _ = expression.format(writer, FmtState::new(pretty, &ctx_state));

    Ok(())
}

/// Evaluates a RESL expression from a string and returns the computed value.
///
/// This is the main entry point for evaluating RESL expressions from string input.
/// It handles parsing the input, setting up the evaluation context, and returning
/// the computed value.
///
/// # Arguments
///
/// * `input` - A string slice containing the RESL expression to evaluate
///
/// # Returns
///
/// * `Ok(Value)` - The evaluated result as a RESL value
/// * `Err(ParseError)` - If parsing fails
///
/// # Examples
///
/// ```
/// use resl::evaluate;
///
/// // Simple arithmetic
/// let result = evaluate("5 + 3").unwrap();
/// assert_eq!(result.to_string(), "8");
///
/// // Variable assignment and reference
/// let result = evaluate("{x = 10; x * 2}").unwrap();
/// assert_eq!(result.to_string(), "20");
///
/// // Function declaration and call
/// let result = evaluate("{add = |a, b| a + b; add(15, 27)}").unwrap();
/// assert_eq!(result.to_string(), "42");
///
/// // Conditional logic
/// let result = evaluate("? true : \"success\" | \"failure\"").unwrap();
/// assert_eq!(result.to_string(), "\"success\"");
///
/// // Collections
/// let result = evaluate("[1, 2, 3]").unwrap();
/// assert_eq!(result.to_string(), "[1, 2, 3]");
/// // result is a List containing integers 1, 2, 3
/// ```
pub fn evaluate(input: &str) -> Result<Value, ParseError> {
    let mut ctx_state = CtxState::new();

    let expression = Expr::parse_all(input, &mut ctx_state)?;

    let value = expression.evaluate(&mut EvalState::new(&mut ctx_state));

    Ok(value)
}

/// Evaluates a RESL expression from a string and writes the formatted result to a writer.
///
/// This utility function combines evaluation and formatting in one operation. It's useful
/// for evaluating RESL expressions and obtaining their string representation without
/// needing to handle the intermediate parsed or evaluated form.
///
/// # Arguments
///
/// * `input` - The RESL expression string to evaluate and format
/// * `writer` - A mutable reference to any type implementing `Write`
/// * `pretty` - Whether to format the output for readability (with indentation and spacing)
///
/// # Returns
///
/// * `Ok(())` - If evaluation and formatting succeed
/// * `Err(ParseError)` - If parsing or evaluation fails, or write operation fails
///
/// # Examples
///
/// ```
/// use resl::evaluate_and_format;
///
/// let mut output = String::new();
/// evaluate_and_format("{x=5;x*2}", &mut output, true).unwrap();
/// assert_eq!(output, "10");
/// ```
pub fn evaluate_and_format<W: std::fmt::Write>(
    input: &str,
    writer: &mut W,
    pretty: bool,
) -> Result<(), ParseError> {
    let value = evaluate(input)?;

    // For now, ignore IO errors since they're less common than parse errors
    let _ = value.write_formatted(writer, pretty);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{evaluate, value::Value};

    #[test]
    fn test_document() {
        let input = r#"
{
    null_val = null;
    app_name = "TradeSync";
    version = 1.0;
    users_count = 42;
    debug = true;

    config = {
        retries = 3;
        timeout = 30;
        timeout
    };

    servers = [
        ["host": "localhost", "port": 8080],
        ["host": "backup.server", "port": 9090]
    ];

    first_server_host = servers[0]["host"];

    math_test = (users_count + 8) * 2;
    flag_test = !false;

    active_host = ? debug : "debug.local" | "prod.local";

    all_ports = servers > (k, v): v["port"];

    add = |a, b| (a + b);
    sum_test = add(5, 10);

    [
        null_val,
        app_name,
        version,
        users_count,
        debug,
        config,
        servers,
        first_server_host,
        math_test,
        flag_test,
        active_host,
        all_ports,
        sum_test
    ]
}
    "#;

        let output = evaluate(input);
        assert!(output.is_ok());
        let output = output.unwrap();

        match output {
            Value::List(list) => {
                assert!(matches!(list[0], Value::Null));
                assert_eq!(list[1], Value::String("TradeSync".into()));
                assert_eq!(list[2], Value::Float(1.0));
                assert_eq!(list[3], Value::Integer(42));
                assert_eq!(list[4], Value::Boolean(true));
                assert_eq!(list[5], Value::Integer(30));
                match &list[6] {
                    Value::List(servers) => {
                        assert_eq!(servers.len(), 2);
                        match &servers[0] {
                            Value::Map(map) => {
                                assert_eq!(
                                    map.get("host"),
                                    Some(&Value::String("localhost".into()))
                                );
                                assert_eq!(map.get("port"), Some(&Value::Integer(8080)));
                            }
                            _ => panic!("Expected server[0] to be map"),
                        }
                        match &servers[1] {
                            Value::Map(map) => {
                                assert_eq!(
                                    map.get("host"),
                                    Some(&Value::String("backup.server".into()))
                                );
                                assert_eq!(map.get("port"), Some(&Value::Integer(9090)));
                            }
                            _ => panic!("Expected server[1] to be map"),
                        }
                    }
                    _ => panic!("Expected servers to be list"),
                }
                assert_eq!(list[7], Value::String("localhost".into()));
                assert_eq!(list[8], Value::Integer((42 + 8) * 2));
                assert_eq!(list[9], Value::Boolean(true));
                assert_eq!(list[10], Value::String("debug.local".into()));
                match &list[11] {
                    Value::List(ports) => {
                        assert_eq!(ports, &vec![Value::Integer(8080), Value::Integer(9090)]);
                    }
                    _ => panic!("Expected all_ports to be list"),
                }
                assert_eq!(list[12], Value::Integer(15));
            }
            _ => panic!("Expected final output to be list"),
        }
    }
}
