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
                    let res = PathBuf::from(String::from_utf8(zoxide.stdout).ok()?.trim());
                    let pwd = current_dir().ok()?;
                    let mut common = common_path::common_path(&res, &pwd)?;
                    if common == res || common == pwd {
                        common.pop();
                    }
                    let m = res.strip_prefix(common).ok()?;
                    let dir = res.is_dir();
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
