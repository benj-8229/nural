use clap::{Parser, Subcommand, builder::styling};

const ABOUT: &str = "Minimal, fast, context aware note taking tool";
const INIT_LONG_ABOUT: &str = "Initialize a new note context\n\nNotes are scoped to the context they're created in, and commands will access the context based off cwd";
const CREATE_LONG_ABOUT: &str = "Create a new note\nWhen executed with the global flag, this will create a note that can be accessed from any context\nAliases: c, cr, create";
const DELETE_LONG_ABOUT: &str = "Delete a note\nWhen executed with the global flag this can delete notes from any context\nWill ask for confirmation before deleting\nAliases: d, del, delete";
const APPEND_LONG_ABOUT: &str = "Append some text to the end of a note\nWhen executed with the global flag this can append to notes from any context\nAliases: a, ap, append";
const OPEN_LONG_ABOUT: &str = "Open a note\nWill open in the text editor defined in your config \nAliases: o, op, open";
const READ_LONG_ABOUT: &str = "Read a note\nWill read using the command defined in your config \nAliases: r, re, rea, read";

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Yellow.on_default().bold())
    .usage(styling::AnsiColor::Yellow.on_default().bold())
    .literal(styling::AnsiColor::BrightWhite.on_default().bold())
    .placeholder(styling::AnsiColor::White.on_default())
    .error(styling::AnsiColor::BrightRed.on_default().bold())
    .valid(styling::AnsiColor::BrightGreen.on_default().bold())
    .invalid(styling::AnsiColor::BrightRed.on_default().bold());

#[derive(Parser)]
#[command(after_help="Thank you for trying out Nural!", disable_colored_help=false, flatten_help=false)]
#[command(styles=STYLES, version, about=ABOUT)]
pub struct CliEntry {
    #[command(subcommand)]
    pub subcommand: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Initialize a new note context 
    #[command(disable_help_flag=false, long_about=INIT_LONG_ABOUT)]
    Init {
        /// Initialize note context in same directory as repository root. Will backtrack to find repository.
        #[arg(required=false, short, long="use-git")]
        git: bool,

        /// Directory to initialize context in. Defaults to CWD
        #[arg(required=false)]
        directory: Option<String>,
    },

    /// Create a new note 
    #[command(disable_help_flag=false, long_about=CREATE_LONG_ABOUT, aliases=["c", "cr"])]
    Create {
        /// The name of the note to create
        #[arg(required=true)]
        name: String,
    },

    /// Delete a note 
    #[command(disable_help_flag=false, long_about=DELETE_LONG_ABOUT, aliases=["d", "de", "del"])]
    Delete {
        /// Search for note to delete by name
        #[arg(required=false)]
        name: Option<String>,
    },

    /// Append text to the end of a note 
    #[command(disable_help_flag=false, long_about=APPEND_LONG_ABOUT, aliases=["a", "ap"])]
    Append {
        /// The text to be appended to the note
        #[arg(required=true)]
        text: String,

        /// Search for note to append to by name
        #[arg(default_value="", required=false)]
        name: Option<String>,
    },

    /// Open a note in the editor
    #[command(disable_help_flag=false, long_about=OPEN_LONG_ABOUT, aliases=["o", "op"])]
    Open {
        /// Search for note to open by name
        #[arg(required=false)]
        name: Option<String>,
    },

    /// Read a note 
    #[command(disable_help_flag=false, long_about=READ_LONG_ABOUT, aliases=["r", "re", "rea"])]
    Read {
        /// Search for note to open by name
        #[arg(required=false)]
        name: Option<String>,
    },
}

pub fn parse_cli() -> CliEntry {
    CliEntry::parse()
}
