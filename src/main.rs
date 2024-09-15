use std::env;
use std::ffi::OsString;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional unparsed paths argument to parse instead of the value of PATH.
    #[arg(index = 1)]
    unsplit_paths: Option<OsString>,
}

fn main() {
    let unsplit_paths = Cli::parse()
        .unsplit_paths
        .or_else(|| env::var_os("PATH"))
        .expect("unparsed paths on command line or in PATH");

    for (i, path) in env::split_paths(&unsplit_paths).enumerate() {
        let repr = path
            .to_str()
            .map(|p| format!("[{p}]"))
            .unwrap_or_else(|| format!("{path:?}"));

        println!("{i:3}: {repr}");
    }
}
