use serde::{Deserialize, Serialize};

use crate::utils::write_indent;

pub(crate) type ValueList = Vec<Value>;

#[cfg(not(feature = "preserve-order"))]
pub(crate) type ValueMap = std::collections::BTreeMap<String, Value>;
#[cfg(feature = "preserve-order")]
pub(crate) type ValueMap = indexmap::IndexMap<String, Value>;

/// Represents the final output values produced by the RESL language interpreter.
///
/// Values are the end result of evaluating RESL expressions - they are what your
/// RESL programs ultimately produce. Every expression in RESL evaluates to one
/// of these value types. Values can be serialized/deserialized for data interchange
/// and support formatted output with pretty-printing.
///
/// # Examples
///
/// ```
/// use resl::{Value, evaluate};
///
/// // Final outputs from RESL expressions
/// let result1 = evaluate("null").unwrap();
/// assert_eq!(result1, Value::Null);
///
/// let result2 = evaluate("\"Hello, World!\"").unwrap();
/// assert_eq!(result2, Value::String("Hello, World!".to_string()));
///
/// let result3 = evaluate("(5 + 3)").unwrap();
/// assert_eq!(result3, Value::Integer(8));
///
/// let result4 = evaluate("(3.14 * 2.0)").unwrap();
/// assert_eq!(result4, Value::Float(6.28));
///
/// // Operations can also be enclosed in parentheses
/// let result_paren = evaluate("(10 - 2)").unwrap();
/// assert_eq!(result_paren, Value::Integer(8));
///
/// let result5 = evaluate("[1, 2, 3]").unwrap();
/// assert_eq!(result5, Value::List(vec![
///     Value::Integer(1),
///     Value::Integer(2),
///     Value::Integer(3),
/// ]));
///
/// let result6 = evaluate("[\"name\": \"Alice\", \"age\": 30]").unwrap();
/// // This produces a Value::Map containing the key-value pairs
/// ```
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    /// The null output value, representing the absence of meaningful data.
    ///
    /// This is produced by expressions that evaluate to nothing, null literals,
    /// or operations that don't return a meaningful result.
    ///
    /// ### RESL Expressions that produce Null
    ///
    /// ```resl
    /// null
    /// ```
    #[default]
    Null,
    /// A Unicode string output value.
    ///
    /// This is the final result when RESL expressions produce text data.
    /// String literals use double quotes and support standard escape sequences.
    ///
    /// ### RESL Expressions that produce String
    ///
    /// ```resl
    /// "Hello, World!"
    /// "Line 1\nLine 2"
    /// ```
    String(String),
    /// A 64-bit signed integer output value.
    ///
    /// This is produced by integer literals, arithmetic operations that yield
    /// whole numbers, or other expressions that evaluate to integers.
    ///
    /// ### RESL Expressions that produce Integer
    ///
    /// ```resl
    /// 42
    /// -17                   // Negative integer
    /// 0
    /// 9223372036854775807
    /// (5 + 3)               // Binary operations with parentheses
    /// (- 42)                // Unary operations
    /// ```
    Integer(i64),
    /// A 64-bit floating-point output value.
    ///
    /// This is produced by float literals, floating-point arithmetic, or
    /// operations that yield fractional numbers.
    ///
    /// ##### RESL Expressions that produce Decimal
    ///
    /// ```resl
    /// 3.14159
    /// -2.5
    /// 0.0
    /// (3.14 * 2.0)          // Binary operations
    /// (- 3.14)              // Unary operations
    /// ```
    Float(f64),
    /// A boolean output value representing true or false.
    ///
    /// This is the result of logical operations, comparisons, or boolean literals.
    ///
    /// ### RESL Expressions that produce Boolean
    ///
    /// ```resl
    /// true
    /// false
    /// (5 > 3)               // Comparison operations with parentheses
    /// 10 == 10              // Comparison operations without parentheses
    /// (! false)             // Unary logical operations with parentheses
    /// ```
    Boolean(bool),
    /// An ordered list output value containing multiple values.
    ///
    /// This is produced by list literals or operations that generate collections.
    /// Lists can contain any mix of value types, including nested structures.
    ///
    /// ### RESL Expressions that produce List
    ///
    /// ```resl
    /// [1, 2, 3]
    /// ["apple", "banana", "cherry"]
    /// [true, false, null]
    /// [1, "mixed", [2, 3]]
    /// []
    /// ```
    List(ValueList),
    /// An associative map output value with string keys and arbitrary values.
    ///
    /// This is produced by map literals or operations that generate key-value
    /// associations. Maps use string keys and can contain any mix of value types.
    /// The ordering depends on the `preserve-order` feature flag.
    ///
    /// ### RESL Expressions that produce Map
    ///
    /// ```resl
    /// ["name": "Alice", "age": 30]
    /// ["x": 10, "y": 20, "visible": true]
    /// ["data": ["nested": "value"]]
    /// ```
    Map(ValueMap),
}

impl Value {
    /// Formats this value to a writer with optional pretty-printing.
    ///
    /// This method provides fine-grained control over formatting with indentation
    /// levels for nested structures.
    ///
    /// # Parameters
    ///
    /// - `writer`: The writer to output formatted content to
    /// - `pretty`: Whether to use pretty-printing with newlines and indentation
    /// - `indent_level`: The current indentation level for nested formatting
    pub(crate) fn format<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        pretty: bool,
        indent_level: usize,
    ) -> std::fmt::Result {
        match self {
            Value::Null => write!(writer, "null"),
            Value::String(s) => write!(writer, "\"{}\"", s),
            Value::Integer(i) => write!(writer, "{}", i),
            Value::Float(f) => {
                if f.fract() == 0.0 {
                    write!(writer, "{:.1}", f)
                } else {
                    write!(writer, "{f}")
                }
            }
            Value::Boolean(b) => write!(writer, "{}", b),
            Value::List(l) => {
                write!(writer, "[")?;

                if l.is_empty() {
                    write!(writer, "]")?;
                    return Ok(());
                }

                if pretty {
                    writeln!(writer)?;
                    write_indent(writer, indent_level + 1)?;
                }

                let mut list_iter = l.iter().peekable();

                while let Some(value) = list_iter.next() {
                    value.format(writer, pretty, indent_level + 1)?;
                    if list_iter.peek().is_some() {
                        write!(writer, ",")?;
                        if pretty {
                            writeln!(writer)?;
                            write_indent(writer, indent_level + 1)?
                        } else {
                            write!(writer, " ")?;
                        }
                    }
                }

                if pretty {
                    writeln!(writer)?;
                    write_indent(writer, indent_level)?;
                }

                write!(writer, "]")
            }
            Value::Map(m) => {
                write!(writer, "[")?;

                if m.is_empty() {
                    write!(writer, "]")?;
                    return Ok(());
                }

                if pretty {
                    writeln!(writer)?;
                    write_indent(writer, indent_level + 1)?;
                }

                let mut map_iter = m.iter().peekable();

                while let Some((key, value)) = map_iter.next() {
                    write!(writer, "\"{key}\": ")?;
                    value.format(writer, pretty, indent_level + 1)?;
                    if map_iter.peek().is_some() {
                        write!(writer, ",")?;
                        if pretty {
                            writeln!(writer)?;
                            write_indent(writer, indent_level + 1)?
                        } else {
                            write!(writer, " ")?;
                        }
                    }
                }

                if pretty {
                    writeln!(writer)?;
                    write_indent(writer, indent_level)?;
                }

                write!(writer, "]")
            }
        }
    }

    /// Writes a formatted representation of this value to a writer.
    ///
    /// This is a convenience method that calls `format` with an indent level of 0.
    ///
    /// # Parameters
    ///
    /// - `writer`: The writer to output formatted content to
    /// - `pretty`: Whether to use pretty-printing with newlines and indentation
    ///
    /// # Examples
    ///
    /// ```
    /// use resl::Value;
    ///
    /// let value = Value::Integer(42);
    /// let mut output = String::new();
    /// value.write_formatted(&mut output, false).unwrap();
    /// assert_eq!(output, "42");
    /// ```
    pub fn write_formatted<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        pretty: bool,
    ) -> std::fmt::Result {
        self.format(writer, pretty, 0)
    }

    /// Returns `true` if this value is a string.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Returns `true` if this value is an integer.
    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    /// Returns `true` if this value is a float.
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Returns `true` if this value is a boolean.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Returns `true` if this value is a list.
    pub fn is_list(&self) -> bool {
        matches!(self, Value::List(_))
    }

    /// Returns `true` if this value is a map.
    pub fn is_map(&self) -> bool {
        matches!(self, Value::Map(_))
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f, false, 0).map_err(|_| std::fmt::Error)
    }
}
