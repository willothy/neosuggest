use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};
use ngrammatic::CorpusBuilder;
use rustbreak::{deser::Bincode, PathDatabase};
use serde::{Deserialize, Serialize};

use super::Source;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    #[serde(rename = "f")]
    frequency: usize,
    #[serde(rename = "u")]
    used: u64,
    #[serde(rename = "a")]
    args: HashMap<Arg, ArgKind>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(u8)]
pub enum ArgKind {
    /// The argument is a file
    File,
    /// The argument is a directory
    Dir,
    /// The argument is a flag
    Flag,
    /// The argument is a number
    Number,
    /// The argument is a string / keyword / subcommand
    Word,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Arg {
    #[serde(rename = "t")]
    text: String,
    #[serde(rename = "f")]
    frequency: usize,
    #[serde(rename = "u")]
    used: u64,
}

impl<T: AsRef<str>> From<T> for Arg {
    fn from(text: T) -> Self {
        Self {
            text: text.as_ref().to_string(),
            frequency: 0,
            used: 0,
        }
    }
}

impl Hash for Arg {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text.hash(state);
    }
}

impl PartialEq for Arg {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Eq for Arg {}

pub fn load(path: PathBuf) -> Result<Db> {
    PathDatabase::<HashMap<String, Entry>, Bincode>::load_from_path_or_default(path)
        .with_context(|| format!("Db load error"))
}

pub type Db = PathDatabase<HashMap<String, Entry>, Bincode>;

pub fn match_command<'a>(db: &'a Db, cmd: impl AsRef<str>) -> Option<String> {
    db.read(|db| -> Option<String> {
        let cmd = cmd.as_ref();
        let cmdlen = cmd.len();
        let threshold = if cmdlen > 7 { 0.7 } else { 0.1 * cmdlen as f32 };
        let mut res = CorpusBuilder::new()
            .arity(2)
            .fill(db.keys())
            .finish()
            .search(cmd.as_ref(), threshold);
        res.get_mut(0).map(|res| std::mem::take(&mut res.text))
    })
    .ok()
    .flatten()
}

/// fn(impl AsRef<str>) -> Option<(A, B, C)>
/// A: The command
/// B: The arguments
/// C: Returns the argument position of the input, where 0 is the command
pub fn parse_input(input: impl AsRef<str>) -> Option<(String, Vec<String>, usize)> {
    let words = shell_words::split(input.as_ref()).ok()?;

    let last_key = words.iter().rposition(|w| w.starts_with('-'))?;

    let (args, argv) = argmap::parse(words.iter());
    let mut args = args.into_iter();
    let cmd = args.nth(0)?.clone();

    let argpos = words.len().checked_sub(1).unwrap_or(0);
    let mut words = words.iter();

    let cmd = words.nth(0)?.clone();
    let args = words.cloned().collect();

    Some((cmd, args, argpos))
}

const ONE_HOUR: u64 = 60 * 60;
const ONE_DAY: u64 = ONE_HOUR * 24;
const ONE_WEEK: u64 = ONE_DAY * 7;

#[async_trait::async_trait]
impl Source for Db {
    ///  1. Get command
    ///  2. Get current arg position
    ///  3. Query db for command
    ///
    ///  If hit:
    ///  1. Query args for current command
    ///  2. Trim result to current arg position
    ///  3. Provide completion
    ///  4. Write new command to db
    ///
    ///  Else:
    ///  1. Fallback to pwd or zoxide
    ///  2. Write new command to db
    ///
    async fn source(&self, word: &str) -> Option<String> {
        // Todo: Autocomplete in unclosed quotes
        let words = shell_words::split(word.as_ref()).ok()?;
        let count = words.len();
        let cmd = words.get(0)?;
        let last_key = words.iter().rfind(|w| w.starts_with('-'));
        let kind = if let Some(last_key) = last_key {
            self.read(|db| db.get(cmd)?.args.get(&Arg::from(&last_key)).cloned())
                .ok()
                .flatten()
        } else {
            None
        };

        let (args, argv) = argmap::parse(words.iter());
        let mut args = args.into_iter();
        let cmd = args.nth(0)?.clone();
        todo!()
        //   if let Some((cmd, args, pos)) = parse_input(word) {
        //       if pos == 0 {
        //           match_command(self, cmd)
        //       } else {
        //           self.write(|db| -> Option<String> {
        //               let mut cmd = db.get_mut(&cmd)?;
        //
        //               // time now as unix timestamp u64
        //               cmd.used = std::time::SystemTime::now()
        //                   .duration_since(std::time::UNIX_EPOCH)
        //                   .ok()?
        //                   .as_secs();
        //               db.iter_mut().for_each(|(_, e)| {
        //                   // things
        //                   let Ok(now) = std::time::SystemTime::now()
        //                       .duration_since(std::time::UNIX_EPOCH) else {
        // 	return;
        // };
        //
        //                   let now = now.as_secs();
        //                   let diff = now - e.used;
        //
        //                   if diff <= ONE_HOUR {
        //                       e.frequency *= 4;
        //                   } else if diff <= ONE_DAY {
        //                       e.frequency *= 2;
        //                   } else if diff <= ONE_WEEK {
        //                       e.frequency /= 2;
        //                   } else {
        //                       e.frequency /= 4;
        //                   }
        //               });
        //
        //               todo!()
        //           })
        //           .ok()
        //           .flatten()
        //       }
        //   } else {
        //       match_command(self, word)
        //   }
    }
}
