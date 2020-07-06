use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

pub(crate) mod command;
pub(crate) mod config;
pub(crate) mod ops;

use command::contains::ContainsCommand;
use command::find::FindCommand;
use command::replace::ReplaceCommand;
use command::walk::WalkCommand;
use command::Command;
use config::Config;
/// The command line argument implementation
#[derive(StructOpt, Debug)]
#[structopt(about = "Recursive directory traversal file management tool")]
enum Recurse {
    #[structopt(about = "Test for string in text files")]
    Contains {
        /// File extension filter
        #[structopt(short = "e", long = "ext", help = "File extension filter")]
        extension: Option<String>,

        /// Include hidden files under dot directory or dot file paths
        /// The default is to not include these files
        #[structopt(short = "a", long = "all", help = "Include hidden files")]
        hidden: bool,

        /// Define the minimum depth of the directory traversal
        #[structopt(long = "mindepth", help = "Minimum directory depth")]
        mindepth: Option<usize>,

        /// Define the maximum depth of the directory traversal
        #[structopt(long = "maxdepth", help = "Maximum directory depth")]
        maxdepth: Option<usize>,

        /// Follow symbolic links
        /// Default is to not follow symbolic links
        #[structopt(long = "symlinks", help = "Follow symbolic links")]
        symlinks: bool,

        /// Find string
        #[structopt(help = "Find regular expression pattern")]
        find: String,

        /// Input file
        #[structopt(parse(from_os_str), help = "Traversal start path")]
        inpath: PathBuf,
    },
    #[structopt(about = "Find strings in text files")]
    Find {
        /// File extension filter
        #[structopt(short = "e", long = "ext", help = "File extension filter")]
        extension: Option<String>,

        /// Include hidden files under dot directory or dot file paths
        /// The default is to not include these files
        #[structopt(short = "a", long = "all", help = "Include hidden files")]
        hidden: bool,

        /// Define the minimum depth of the directory traversal
        #[structopt(long = "mindepth", help = "Minimum directory depth")]
        mindepth: Option<usize>,

        /// Define the maximum depth of the directory traversal
        #[structopt(long = "maxdepth", help = "Maximum directory depth")]
        maxdepth: Option<usize>,

        /// Follow symbolic links
        /// Default is to not follow symbolic links
        #[structopt(long = "symlinks", help = "Follow symbolic links")]
        symlinks: bool,

        /// Find string
        #[structopt(help = "Find regular expression pattern")]
        find: String,

        /// Input file
        #[structopt(parse(from_os_str), help = "Traversal start path")]
        inpath: PathBuf,
    },
    #[structopt(about = "Replace strings in text files")]
    Replace {
        /// File extension filter
        #[structopt(short = "e", long = "ext", help = "File extension filter")]
        extension: Option<String>,

        /// Include hidden files under dot directory or dot file paths
        /// The default is to not include these files
        #[structopt(short = "a", long = "all", help = "Include hidden files")]
        hidden: bool,

        /// Skip backup write of original file
        #[structopt(long = "nobu", help = "Write inplace without backup")]
        nobu: bool,

        /// Define the minimum depth of the directory traversal
        #[structopt(long = "mindepth", help = "Minimum directory depth")]
        mindepth: Option<usize>,

        /// Define the maximum depth of the directory traversal
        #[structopt(long = "maxdepth", help = "Maximum directory depth")]
        maxdepth: Option<usize>,

        /// Follow symbolic links
        /// Default is to not follow symbolic links
        #[structopt(long = "symlinks", help = "Follow symbolic links")]
        symlinks: bool,

        /// Find string
        #[structopt(short = "f", long = "find", help = "Find regular expression pattern")]
        find: String,

        /// Replace string
        #[structopt(short = "r", long = "replace", help = "Replace string")]
        replace: String,

        /// Input file
        #[structopt(parse(from_os_str), help = "Traversal start path")]
        inpath: PathBuf,
    },
    #[structopt(about = "Walk the directory structure for paths")]
    Walk {
        /// File extension filter
        #[structopt(short = "e", long = "ext", help = "File path extension filter")]
        extension: Option<String>,

        /// Directory only filter
        #[structopt(short = "d", long = "dir", help = "Include directory paths only")]
        dir_only: bool,

        /// Include hidden files under dot directory or dot file paths
        /// The default is to not include these files
        #[structopt(short = "a", long = "all", help = "Include hidden paths")]
        hidden: bool,

        /// Input file
        #[structopt(parse(from_os_str), help = "Traversal start path")]
        inpath: PathBuf,

        /// Define the minimum depth of the directory traversal
        #[structopt(long = "mindepth", help = "Minimum directory depth")]
        mindepth: Option<usize>,

        /// Define the maximum depth of the directory traversal
        #[structopt(long = "maxdepth", help = "Maximum directory depth")]
        maxdepth: Option<usize>,

        /// Follow symbolic links
        /// Default is to not follow symbolic links
        #[structopt(long = "symlinks", help = "Follow symbolic links")]
        symlinks: bool,
    },
}

/// `recurse` executable execution entry point
pub fn run() -> Result<()> {
    let config = Config::new(Recurse::from_args());
    match &config.subcmd {
        Recurse::Contains { .. } => return ContainsCommand::execute(config.subcmd),
        Recurse::Find { .. } => return FindCommand::execute(config.subcmd),
        Recurse::Replace { .. } => return ReplaceCommand::execute(config.subcmd),
        Recurse::Walk { .. } => return WalkCommand::execute(config.subcmd),
    }
}
