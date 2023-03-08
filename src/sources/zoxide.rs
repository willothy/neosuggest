use std::env::current_dir;
use std::path::PathBuf;
use std::process::{Command, Output};

use super::Source;

pub struct Zoxide;

#[async_trait::async_trait]
impl Source for Zoxide {
    fn cond(&self, s: &str) -> bool {
        s.split_whitespace().count() > 1
    }

    async fn source(&self, word: &str) -> Option<String> {
        let search = word.split_whitespace().last()?.trim_end();
        let rest = &word[..word.len() - search.len()].trim();
        Command::new("zoxide")
            .args(["query", search])
            .output()
            .ok()
            .map_or(None, |zoxide: Output| {
                zoxide.status.success().then(|| {
                    let res = PathBuf::from(
                        String::from_utf8_lossy(zoxide.stdout.as_slice()).to_string(),
                    );
                    let pwd = current_dir().ok()?;
                    let mut common = common_path::common_path(&res, &pwd).expect("bruh");
                    if common == res || common == pwd {
                        common.pop();
                    }
                    let m = res.strip_prefix(common).ok()?;
                    // let dir = res.is_dir();
                    if rest.is_empty() {
                        Some(m.file_name()?.to_string_lossy().to_string())
                    } else {
                        Some(format!(
                            "{} {}",
                            rest,
                            m.file_name()?.to_string_lossy().to_string()
                        ))
                    }
                })?
            })
    }
}
