use yansi::{Color, Condition, Paint, Style};

#[derive(Debug)]
pub(crate) enum CliError {
    Io(std::io::Error),
    Fmt(std::fmt::Error),
    Resl(resl::ParseError),
    Json(serde_json::Error),
    TomlSer(toml::ser::Error),
    TomlDe(toml::de::Error),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Io(err) => display_io_error(f, err),
            CliError::Fmt(err) => display_fmt_error(f, err),
            CliError::Resl(err) => display_resl_error(f, err),
            CliError::Json(err) => display_json_error(f, err),
            CliError::TomlSer(err) => display_toml_ser_error(f, err),
            CliError::TomlDe(err) => display_toml_de_error(f, err),
        }
    }
}

static IS_TTY: Condition = Condition::STDERR_IS_TTY;
const RED_BOLD_UL: Style = Color::Red.bold().underline().whenever(IS_TTY);
const BRIGHT_RED: Style = Color::BrightRed.whenever(IS_TTY);
const WHITE: Style = Color::White.whenever(IS_TTY);
const YELLOW_BOLD: Style = Color::Yellow.bold().whenever(IS_TTY);
const CYAN_BOLD: Style = Color::Cyan.bold().whenever(IS_TTY);
const BRIGHT_BLACK_BOLD: Style = Color::BrightBlack.bold().whenever(IS_TTY);

fn display_io_error(f: &mut std::fmt::Formatter<'_>, err: &std::io::Error) -> std::fmt::Result {
    write!(
        f,
        "{} {}",
        "IO Error:".paint(RED_BOLD_UL),
        err.paint(BRIGHT_RED)
    )
}

fn display_fmt_error(f: &mut std::fmt::Formatter<'_>, err: &std::fmt::Error) -> std::fmt::Result {
    write!(
        f,
        "{} {}",
        "Format Error:".paint(RED_BOLD_UL),
        err.paint(BRIGHT_RED)
    )
}

fn display_resl_error(f: &mut std::fmt::Formatter<'_>, err: &resl::ParseError) -> std::fmt::Result {
    let label = err
        .label
        .clone()
        .map(|label| format!("Invalid {label}"))
        .unwrap_or(String::from("Invalid Token"));

    writeln!(
        f,
        "{} {}",
        "Parse Error:".paint(RED_BOLD_UL),
        label.paint(BRIGHT_RED),
    )?;

    // Location specifier

    let location = format!("line {}, column {}", err.line_number, err.column);

    let line_index = err.line_number.to_string();

    let gutter = line_index.len() + 1;

    for _ in 0..gutter {
        write!(f, " ")?;
    }

    writeln!(
        f,
        "{}{}{}",
        "┌─[".paint(WHITE),
        location.paint(YELLOW_BOLD),
        "]".paint(WHITE)
    )?;

    // Empty line
    for _ in 0..gutter {
        write!(f, " ")?;
    }

    writeln!(f, "{}", "│".paint(WHITE))?;

    // Line Location and Content

    writeln!(
        f,
        " {}{}{}",
        line_index.paint(BRIGHT_BLACK_BOLD),
        "│".paint(WHITE),
        err.line_content
    )?;

    // Marker for error position
    let column_position = err.column - 1;

    for _ in 0..gutter {
        write!(f, " ")?;
    }

    write!(f, "{}", "│".paint(WHITE))?;

    for _ in 0..column_position {
        write!(f, " ")?;
    }

    writeln!(f, "{}", "^".paint(WHITE))?;

    // Expected tokens

    for _ in 0..gutter {
        write!(f, " ")?;
    }

    write!(f, "{}", "└─[".paint(WHITE))?;

    write!(f, "{}", "Expected ".paint(CYAN_BOLD))?;

    match err.expected.as_slice() {
        [] => {}
        [single] => write!(f, "{}", single.paint(CYAN_BOLD))?,
        [all @ .., last] => {
            write!(
                f,
                "{} {} {}",
                all.join(", ").paint(CYAN_BOLD),
                "or".paint(CYAN_BOLD),
                last.paint(CYAN_BOLD)
            )?;
        }
    }

    writeln!(f, "{}", "]".paint(WHITE))?;

    Ok(())
}

fn display_json_error(
    f: &mut std::fmt::Formatter<'_>,
    err: &serde_json::Error,
) -> std::fmt::Result {
    write!(
        f,
        "{} {}",
        "JSON Error:".paint(RED_BOLD_UL),
        err.paint(BRIGHT_RED)
    )
}

fn display_toml_ser_error(
    f: &mut std::fmt::Formatter<'_>,
    err: &toml::ser::Error,
) -> std::fmt::Result {
    write!(
        f,
        "{} {}",
        "TOML Error:".paint(RED_BOLD_UL),
        err.paint(BRIGHT_RED)
    )
}

fn display_toml_de_error(
    f: &mut std::fmt::Formatter<'_>,
    err: &toml::de::Error,
) -> std::fmt::Result {
    write!(
        f,
        "{} {}",
        "TOML Error:".paint(RED_BOLD_UL),
        err.paint(BRIGHT_RED)
    )
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::Io(err)
    }
}

impl From<std::fmt::Error> for CliError {
    fn from(err: std::fmt::Error) -> Self {
        CliError::Fmt(err)
    }
}

impl From<resl::ParseError> for CliError {
    fn from(err: resl::ParseError) -> Self {
        CliError::Resl(err)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        CliError::Json(err)
    }
}

impl From<toml::ser::Error> for CliError {
    fn from(err: toml::ser::Error) -> Self {
        CliError::TomlSer(err)
    }
}

impl From<toml::de::Error> for CliError {
    fn from(err: toml::de::Error) -> Self {
        CliError::TomlDe(err)
    }
}
