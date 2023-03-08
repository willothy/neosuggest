use builder::SourcesBuilder;
use sources::pwd::Pwd;
use sources::zoxide::Zoxide;
use std::env::args;
use std::io::{stdout, Write};
use std::process::ExitCode;

use self::sources::path::Path;

pub mod builder;
pub mod sources;

// const HISTFILE: &str = "HISTFILE";

#[tokio::main]
async fn main() -> ExitCode {
    let word = match args().nth(1) {
        Some(word) if !word.is_empty() => word,
        _ => return ExitCode::SUCCESS,
    };

    let sources = SourcesBuilder::new()
        .using(Pwd) // Matches entries in pwd or path query
        .using(Path)
        .using(Zoxide) // Matches with Zoxide
        .finalize();

    let mut stdout = stdout();
    if let Some(res) = sources.search(word.clone()).await {
        let Ok(_) = write!(stdout, "{}", res) else {
			return ExitCode::FAILURE
		};
        let Ok(_) = stdout.flush() else {
			return ExitCode::FAILURE
		};
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
