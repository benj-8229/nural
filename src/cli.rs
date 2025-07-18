use clap::{Parser, Subcommand, builder::styling};

const ABOUT: &str = "Minimal, fast, context aware note taking tool";
const INIT_LONG_ABOUT: &str = "Initialize a new note context\n\nBy default, commands are scoped only to the current context, with the \"global context\" being accessible using the global flag";
const CREATE_LONG_ABOUT: &str = "Create a new note\nWhen executed with the global flag, this will create a note that can be accessed from any context\nAliases: c, cr, create";
const DELETE_LONG_ABOUT: &str = "Delete a note\nWhen executed with the global flag this can delete notes from any context\nWill ask for confirmation before deleting\nAliases: d, del, delete";
const APPEND_LONG_ABOUT: &str = "Append some text to the end of a note\nWhen executed with the global flag this can append to notes from any context\nWill prompt to create a new note if no note matches filters\nAliases: a, ap, append";
const LIST_LONG_ABOUT: &str = "List notes in the available context\nWhen executed with the global flag this will list notes from all contexts\nCan use filtering syntax\nAliases: l, ls, list";
const EDIT_LONG_ABOUT: &str = "Edit a note\nWhen executed with the global flag this can edit notes from any context\nWill open in the text editor defined in your config \nAliases: e, ed, edit";
const FILTER_ABOUT: &str = "\
Use filter syntax to select notes based on metadata or content

Filters are key-value pairs, separated by ':' or ',' (both are supported):

    tag:bug,title:crash       # Matches notes with tag 'bug' and title containing 'crash'
    tag:urgent                # Matches any note with the 'urgent' tag
    title:README              # Matches notes with 'README' in the title
    text:\"panic handler\"      # Matches notes containing the phrase 'panic handler'

You can combine filters for precise matching. The following are equivalent:

    tag:bug,auth              # Matches notes tagged 'bug' and 'auth'
    tag:bug tag:auth          # Also matches 'bug' AND 'auth'

By default, filters match *all* specified fields (logical AND).
Future extensions may support NOT or OR modifiers.

Tip: You can also use `--name` or `--tag` directly for simpler cases.";


const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Yellow.on_default().bold())
    .usage(styling::AnsiColor::Yellow.on_default().bold())
    .literal(styling::AnsiColor::BrightWhite.on_default().bold())
    .placeholder(styling::AnsiColor::White.on_default())
    .error(styling::AnsiColor::BrightRed.on_default().bold())
    .valid(styling::AnsiColor::BrightGreen.on_default().bold())
    .invalid(styling::AnsiColor::BrightRed.on_default().bold());

#[derive(Parser)]
#[command(after_help="Thank you for trying out Nural!", disable_colored_help=false, 
    flatten_help=false)]
#[command(styles=STYLES, version, about=ABOUT)]
pub struct CliEntry {
    /// Execute with access to all project contexts
    #[arg(short, long)]
    pub global: bool,

    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project context 
    #[command(disable_help_flag=false, long_about=INIT_LONG_ABOUT)]
    Init {
        /// Initialize project context in same directory as repository root. Will backtrack to find repository.
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

        /// Comma seperated list of tags to apply to note (unused)
        #[arg(required=false, short, long)]
        tag: Option<String>,
    },

    /// Delete a note 
    #[command(disable_help_flag=false, long_about=DELETE_LONG_ABOUT, aliases=["d", "del"])]
    Delete {
        /// Search for note to delete by name
        #[arg(long, short, required=false)]
        name: Option<String>,

        /// Search for note to delete by tag (unused)
        #[arg(long, short, required=false)]
        tag: Option<String>,
    },

    /// Append some text to the end note 
    #[command(hide=false, disable_help_flag=false, long_about=APPEND_LONG_ABOUT, aliases=["a", "ap"])]
    Append {
        /// Search for note to append to by tag (unused)
        #[arg(long, short, required=false)]
        tag: Option<String>,

        /// Search for note to append to by name
        #[arg(long, short, required=false)]
        name: Option<String>,

        /// The text to be appended to the note
        #[arg(required=false)]
        text: Option<String>,
    },

    /// Edit a note
    #[command(disable_help_flag=false, long_about=EDIT_LONG_ABOUT, aliases=["e", "ed"])]
    Edit {
        /// Search for note to delete by name
        #[arg(required=false)]
        name: Option<String>,

        /// Search for note to delete by tag (unused)
        #[arg(long, short, required=false)]
        tag: Option<String>,
    },

    /// List notes from available context
    #[command(disable_help_flag=false, long_about=LIST_LONG_ABOUT, aliases=["l", "ls"])]
    List {
        /// Filter notes to list by name
        #[arg(required=false)]
        name: Option<String>,
    },

    //#[command(hide=true, override_usage="space seperated list of filters following the -f/--filters flag", disable_help_flag=true)]
    #[command(hide=true, about=FILTER_ABOUT, override_usage="nural list -f 'since:commit tagged:log'", disable_help_flag=true)]
    Filter {
    },
}

pub fn parse_cli() -> CliEntry {
    CliEntry::parse()
}
