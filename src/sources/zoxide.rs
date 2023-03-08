use std::env::current_dir;
use std::path::PathBuf;
use std::process::{Command, Output};

use super::Source;

pub struct Zoxide;

#[async_trait::async_trait]
impl Source for Zoxide {
    async fn source(&self, word: &str) -> Option<String> {
        let mut split = word.split_whitespace().rev();
        let search = split.next()?;
        let rest = split.rev().collect::<Vec<_>>();
        Command::new("zoxide")
            .args(["query", search])
            .output()
            .ok()
            .map_or(None, |zoxide: Output| {
                zoxide.status.success().then(|| {
                    let res = PathBuf::from(String::from_utf8_lossy(&zoxide.stdout).to_string());
                    let pwd = current_dir().ok()?;
                    let common = common_path::common_path(&res, &pwd)?;
                    let m = res.strip_prefix(common).ok()?;
                    // println!("pwd: {}", pwd.display());
                    // // println!("common: {}", common.display());
                    println!("m: {}", m.display());

                    println!("res: {}", res.display());
                    let dir = res.canonicalize().expect("bruh").is_dir();
                    println!("dir: {}", dir);
                    if rest.is_empty() {
                        Some(m.display().to_string() + if dir { "/" } else { "" })
                    } else {
                        Some(format!(
                            "{} {}{}",
                            rest.join(" "),
                            m.display().to_string(),
                            if dir { "/" } else { "" }
                        ))
                    }
                })?
            })
    }
}
