//! # RESL CLI
//!
//! Command-line interface for RESL - Runtime Evaluated Serialization Language.
//!
//! Formats, evaluates, and converts RESL configuration files to JSON, TOML, and vice versa.

mod error;

mod json_utils;
mod toml_utils;

use std::{
    fs,
    io::{self, Read as _, Write},
    path::PathBuf,
    process::exit,
};

use clap::{Parser, Subcommand, ValueEnum};
use resl::evaluate_and_format;

use crate::{
    error::CliError,
    json_utils::{json_to_resl, resl_to_json},
    toml_utils::{resl_to_toml, toml_to_resl},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,

    /// The input file to read from (Note: leave empty for stdin)
    #[arg(short, long, global = true)]
    input: Option<PathBuf>,

    /// The output file to write to (Note: leave empty for stdout)
    #[arg(short, long, global = true)]
    output: Option<PathBuf>,

    /// The format style for output
    #[arg(short, long, global = true)]
    pretty: bool,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    /// Format RESL expression
    Format,

    /// Parse and evaluate RESL expression
    Evaluate,

    /// Export from RESL to JSON/TOML
    Export {
        /// Format to export to (json, toml)
        #[arg(value_enum, long)]
        to: DataFormat,
    },

    /// Import from JSON/TOML to RESL
    Import {
        /// Format to import from (json, toml)
        #[arg(value_enum, long)]
        from: DataFormat,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum DataFormat {
    #[value(name = "JSON", alias = "json")]
    Json,
    #[value(name = "TOML", alias = "toml")]
    Toml,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        exit(1);
    }
}

fn run() -> anyhow::Result<(), CliError> {
    let cli = Cli::parse();

    let input = match cli.input {
        Some(input_path) => fs::read_to_string(input_path)?,
        None => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            input
        }
    };

    let pretty = cli.pretty;

    match cli.command {
        Command::Format => match cli.output {
            Some(output_path) => {
                let mut file = fs::File::create(output_path)?;

                resl::format(&input, &mut IoFmtAdapter(&mut file), pretty)?;
            }
            None => {
                let mut stdout = io::stdout();
                resl::format(&input, &mut IoFmtAdapter(&mut stdout), pretty)?;
            }
        },
        Command::Evaluate => match cli.output {
            Some(output_path) => {
                let mut file = fs::File::create(output_path)?;
                evaluate_and_format(&input, &mut IoFmtAdapter(&mut file), pretty)?;
            }
            None => {
                let mut stdout = io::stdout();
                evaluate_and_format(&input, &mut IoFmtAdapter(&mut stdout), pretty)?;
            }
        },
        Command::Export { to } => {
            let resl_value = resl::evaluate(&input)?;
            match to {
                DataFormat::Json => {
                    let json_value = resl_to_json(resl_value);

                    match cli.output {
                        Some(output_path) => {
                            let mut file = fs::File::create(output_path)?;
                            if cli.pretty {
                                serde_json::to_writer_pretty(&mut file, &json_value)?;
                            } else {
                                serde_json::to_writer(&mut file, &json_value)?;
                            }
                        }
                        None => {
                            if cli.pretty {
                                serde_json::to_writer_pretty(io::stdout(), &json_value)?;
                            } else {
                                serde_json::to_writer(io::stdout(), &json_value)?;
                            }
                        }
                    }
                }
                DataFormat::Toml => {
                    let toml_value = resl_to_toml(resl_value);

                    match cli.output {
                        Some(output_path) => {
                            let mut file = fs::File::create(output_path)?;
                            let s = if cli.pretty {
                                toml::to_string_pretty(&toml_value)?
                            } else {
                                toml::to_string(&toml_value)?
                            };

                            file.write_all(s.as_bytes())?;
                        }
                        None => {
                            let s = if cli.pretty {
                                toml::to_string_pretty(&toml_value)?
                            } else {
                                toml::to_string(&toml_value)?
                            };
                            io::stdout().write_all(s.as_bytes())?;
                        }
                    }
                }
            };
        }
        Command::Import { from } => {
            let resl_value = match from {
                DataFormat::Json => {
                    let json_value = serde_json::from_str(&input)?;
                    json_to_resl(json_value)
                }
                DataFormat::Toml => {
                    let toml_value = toml::from_str(&input)?;
                    toml_to_resl(toml_value)
                }
            };

            match cli.output {
                Some(output_path) => {
                    let mut file = fs::File::create(output_path)?;
                    resl_value.write_formatted(&mut IoFmtAdapter(&mut file), pretty)?;
                }
                None => {
                    let mut stdout = io::stdout();
                    resl_value.write_formatted(&mut IoFmtAdapter(&mut stdout), pretty)?;
                }
            }
        }
    }

    exit(0)
}

/// A wrapper adapter that implements [`std::fmt::Write`] for types that implement [`std::io::Write`].
struct IoFmtAdapter<'a, W: std::io::Write>(&'a mut W);

impl<'a, W: std::io::Write> std::fmt::Write for IoFmtAdapter<'a, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
    }
}
