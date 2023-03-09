use anyhow::anyhow;
use builder::SourcesBuilder;

use std::env::args;
use std::io::{stdout, Write};
use std::process::ExitCode;

use anyhow::Result;

pub mod builder;
mod init;
pub mod sources;
pub mod util;
// const HISTFILE: &str = "HISTFILE";

async fn run(word: &String) -> Result<()> {
    // let Some(config) = dirs::config_dir() else {
    // 	return Err(anyhow!("Could not find config dir"))
    // };
    // let path = config.join("neosuggest.db");
    // let db = sources::db::load(path)?;

    let sources = SourcesBuilder::new()
        .using(sources::Pwd) // Matches entries in pwd or path query
        .with_priority(0)
        .using(sources::Zoxide) // Matches with Zoxide
        .with_priority(1)
        // .using(Path)
        .finalize();

    let mut stdout = stdout();
    if let Some(res) = sources.search(word.clone()).await {
        write!(stdout, "{}", res)?;
        stdout.flush()?;
        Ok(())
    } else {
        Err(anyhow!(""))
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = args().skip(1).take(2).collect::<Vec<_>>();
    let res = match args.get(0) {
        Some(word) if &*word == "run" => match args.get(1) {
            Some(word) => run(word).await,
            None => Ok(()),
        },
        Some(word) if &*word == "init" => init::init(),
        _ => Ok(()),
    };
    if res.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
