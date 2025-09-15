use winnow::{
    error::{ContextError, ParseError as WinnowParseError, StrContext, StrContextValue},
    stream::AsBStr,
};

use crate::StatefulInput;

/// Represents parsing errors in the RESL language.
///
/// `ParseError` provides detailed error information including the location of the error,
/// the problematic line content, and context about what was expected during parsing.
/// It formats errors in a user-friendly way similar to modern compiler error messages.
///
/// Note: This error type only covers parsing failures. RESL evaluation is infallible
/// and does not produce runtime errors.
///
/// # Examples
///
/// ```
/// use resl::evaluate;
///
/// // This will produce a ParseError due to invalid syntax
/// let result = evaluate("(5 +)");
/// assert!(result.is_err());
///
/// let error = result.unwrap_err();
/// println!("{}", error); // Displays formatted error message
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    /// The line number where the error occurred (1-indexed)
    pub line_number: usize,
    /// The column number where the error occurred (1-indexed)
    pub column: usize,
    /// The content of the line where the error occurred
    pub line_content: String,
    /// An optional label describing the type of error (e.g., "expression", "literal")
    pub label: Option<String>,
    /// A list of expected tokens or constructs that would be valid at this location
    pub expected: Vec<String>,
}

/// Converts winnow parser errors into user-friendly RESL parsing errors.
///
/// This implementation extracts location information, line content, and context
/// from the winnow parser error to create a detailed error message. It processes
/// context information to provide helpful suggestions about what was expected.
impl From<WinnowParseError<StatefulInput<'_, '_>, ContextError>> for ParseError {
    fn from(value: WinnowParseError<StatefulInput, ContextError>) -> Self {
        let offset = value.offset();
        let input_str = value.input().as_bstr();

        let mut line_start_byte = 0;
        let mut line_end_byte = input_str.len();
        let mut line_number = 1;
        let mut column = 1;

        // Enumerate over the input string from starting to the offset
        // This is to find the `line_number`, `line_start_byte`, and `column`
        for (index, byte) in input_str[0..offset].iter().enumerate() {
            // Check if byte represents a new line
            if *byte == b'\n' {
                // Set `line_start_byte` at the next index of the `\n` char
                line_start_byte = index + 1;
                // Increment `line_number` by 1
                line_number += 1;
                // Reset `column` to 1
                column = 1;
            } else {
                // Increment `column` by 1
                column += 1;
            }
        }

        // Enumerate over the input string from the offset to the end
        // This is to find the `line_end_byte`
        for (index, byte) in input_str[offset..].iter().enumerate() {
            // Check if byte represents a new line
            if *byte == b'\n' {
                line_end_byte = index + offset;
                break;
            }
        }

        // The content is always valid UTF-8 since the input is guaranteed to be valid UTF-8
        let line_content =
            unsafe { str::from_utf8_unchecked(&input_str[line_start_byte..line_end_byte]) }
                .to_string();

        let mut label = None;
        let mut expected = Vec::new();

        for ctx in value.inner().context() {
            match ctx {
                StrContext::Label(str) => {
                    // This sets label to the first label encountered
                    let _ = label.get_or_insert(str.to_string());
                }
                StrContext::Expected(val) => match val {
                    StrContextValue::CharLiteral(c) => expected.push(format!("`{c}`")),
                    StrContextValue::StringLiteral(s) => expected.push(format!("\"{s}\"")),
                    StrContextValue::Description(d) => expected.push(d.to_string()),
                    _ => {}
                },
                _ => {}
            }
        }

        Self {
            line_number,
            column,
            line_content,
            label,
            expected,
        }
    }
}

/// Formats the error for display with detailed location and context information.
///
/// The error message includes:
/// - A headline with the error type and location
/// - The source code line where the error occurred
/// - A caret (^) pointing to the exact error location
/// - A list of expected tokens or constructs
///
/// The format is inspired by modern compiler error messages and provides
/// clear, actionable information to help users fix syntax errors.
///
/// # Example Output
///
/// ```text
/// Error: Invalid binary operation
///  --> line 2, column 8
///   |
/// 2 | (5 + )
///   |       ^
///   = Expected expression
/// ```
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // headline
        let label = self
            .label
            .as_ref()
            .map(|l| format!("Invalid {l}"))
            .unwrap_or_else(|| "Invalid token".to_string());

        writeln!(f, "Error: {label}")?;
        writeln!(f, " --> line {}, column {}", self.line_number, self.column)?;

        let gutter_width = self.line_number.to_string().len();

        // gutter + code line
        writeln!(f, "{:>gwidth$} |", "", gwidth = gutter_width)?;
        writeln!(
            f,
            "{:>gwidth$} | {}",
            self.line_number,
            self.line_content,
            gwidth = gutter_width
        )?;

        // marker
        writeln!(
            f,
            "{:>gwidth$} | {:>cwidth$}^",
            "",
            "",
            gwidth = gutter_width,
            cwidth = self.column.saturating_sub(1)
        )?;

        // expected
        write!(f, "{:>gwidth$} = Expected ", "", gwidth = gutter_width)?;
        match self.expected.as_slice() {
            [] => {}
            [single] => write!(f, "{single}")?,
            [all @ .., last] => {
                write!(f, "{} or {last}", all.join(", "))?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for ParseError {}
