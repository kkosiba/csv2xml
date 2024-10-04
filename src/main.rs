use clap::{Parser, ValueEnum};
use std::fmt;

const DEFAULT_ENCODING: &str = "utf-8";

/// Enumeration for delimiters
#[derive(Clone, ValueEnum)]
#[clap(rename_all = "lower")]
enum Delimiter {
    Comma,
    Pipe,
    Tab,
}

impl Delimiter {
    fn as_char(&self) -> char {
        match self {
            Delimiter::Comma => ',',
            Delimiter::Pipe => '|',
            Delimiter::Tab => '\t',
        }
    }
}

/// Enumeration for source name types
#[derive(Clone, ValueEnum)]
#[clap(rename_all = "lower")]
enum NameType {
    Absolute,
    Literal,
    Leaf,
    LeafNoExt,
    NoneType, // Renamed to avoid conflict with Rust's `None`
}

impl fmt::Display for NameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            NameType::Absolute => "absolute",
            NameType::Literal => "literal",
            NameType::Leaf => "leaf",
            NameType::LeafNoExt => "leafnoext",
            NameType::NoneType => "none",
        };
        write!(f, "{}", value)
    }
}

/// Tabular data to XML converter
#[derive(Parser)]
#[command(about = "Converts CSV/TSV files to XML")]
struct Cli {
    /// Path to input file with tabular data
    input_file: String,

    /// Path to XML output file
    output_file: String,

    /// Encoding of the input file
    #[arg(long = "input-encoding", default_value = DEFAULT_ENCODING)]
    encoding: String,

    /// Source name type
    #[arg(long = "source-nametype", default_value_t = NameType::LeafNoExt, value_enum)]
    source_nametype: NameType,

    /// Field limit
    #[arg(short = 'f', long = "field-limit", default_value_t = 131072)]
    field_limit: usize,

    /// Delimiter for the input file
    #[arg(short = 'd', long = "delimiter", default_value_t = Delimiter::Tab, value_enum)]
    delimiter: Delimiter,

    /// Trim whitespace around delimiter
    #[arg(short = 'a', action = clap::ArgAction::SetTrue)]
    trim: bool,

    /// Allow loose quotes
    #[arg(short = 'l', action = clap::ArgAction::SetTrue)]
    allow: bool,
}

fn main() {
    let args = Cli::parse();

    if let Err(error) = encoding::check_encoding(&args.input_file) {
        eprintln!("{}", error);
        std::process::exit(2);
    }

    match converter::convert(
        &args.input_file,
        &args.output_file,
        args.delimiter.as_char(),
        // todo: implement these properly
        //    args.encoding,
        //    args.source_nametype.to_string(),
        //    args.trim,
        //    args.allow,
        //    args.field_limit,
    ) {
        Ok(_) => std::process::exit(0),
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(2);
        }
    }
}
