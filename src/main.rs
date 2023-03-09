use builder::SourcesBuilder;

use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::ExitCode;

pub mod builder;
pub mod sources;
pub mod util;

use rustbreak::{deser::Bincode, FileDatabase, PathDatabase};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Entry {
    path: PathBuf,
    importance: usize,
}

// const HISTFILE: &str = "HISTFILE";

#[tokio::main]
async fn main() -> ExitCode {
    let word = match args().nth(1) {
        Some(word) => word,
        _ => return ExitCode::SUCCESS,
    };

    //    let Some(config) = dirs::config_dir() else {
    // 	return ExitCode::FAILURE
    // };
    //    let path = config.join("neosuggest.db");
    //    let Ok(db) =
    //           PathDatabase::<HashMap<String, Entry>, Bincode>::load_from_path_or_default(path) else {
    //    	return ExitCode::FAILURE
    //    };

    let sources = SourcesBuilder::new()
        .using(sources::Pwd) // Matches entries in pwd or path query
        .with_priority(0)
        .using(sources::Zoxide) // Matches with Zoxide
        .with_priority(1)
        // .using(Path)
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
