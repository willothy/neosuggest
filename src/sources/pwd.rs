use std::env::current_dir;
use std::path::PathBuf;

use ignore::{overrides::OverrideBuilder, WalkBuilder};
use ngrammatic::CorpusBuilder;

use super::Source;

pub struct Pwd;

#[async_trait::async_trait]
impl Source for Pwd {
    fn cond(&self, s: &str) -> bool {
        s.split_whitespace().count() > 1 || s.ends_with(|s: char| s.is_whitespace())
    }

    async fn source(&self, search_word: &str) -> Option<String> {
        let split = search_word.split_whitespace();
        let word = if split.clone().count() > 1 {
            split.last()?
        } else {
            ""
        };
        let rest = search_word[..search_word.len() - word.len()].trim_end();
        let pwd = current_dir().ok()?;
        let path = PathBuf::from(word);
        let len = path.components().count();
        let (base_path, mut search) = if len > 1 {
            let search = path
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_string_lossy()
                .to_string();
            let path = path.parent().unwrap().to_path_buf();

            if !(path.exists() && path.is_dir()) {
                return None;
            }

            (path, search)
        } else {
            (pwd.clone(), word.to_owned())
        };

        let path = base_path.canonicalize().ok()?;

        let overrides = OverrideBuilder::new(&path)
            .add("!.git/")
            .ok()?
            .build()
            .ok()?;
        let entries = WalkBuilder::new(path)
            .max_depth(Some(1))
            .standard_filters(true)
            .overrides(overrides)
            .hidden(!search.starts_with('.'))
            .build()
            .filter_map(|e| e.ok().map(|v| v.file_name().to_string_lossy().to_string()))
            .skip(1)
            .collect::<Vec<_>>();

        if search == "" && !entries.is_empty() {
            let t = entries
                .iter()
                .find(|s| !s.starts_with('.') && !s.ends_with(".lock"))?
                .clone();
            search = t.to_owned();
        }
        let slen = search.len();
        let threshold = if slen > 3 { 0.7 } else { 0.10 * slen as f32 };
        let mut res = CorpusBuilder::new()
            .arity(2)
            .fill(entries)
            .finish()
            .search(&*search, threshold);
        if let Some(res) = res.first_mut() {
            let matching = std::mem::take(&mut res.text);

            let res = if base_path == pwd {
                let p = PathBuf::from(&matching);
                if rest.is_empty() {
                    matching + if p.is_dir() { "/" } else { "" }
                } else {
                    rest.to_owned() + " " + &matching + if p.is_dir() { "/" } else { "" }
                }
            } else {
                let p = base_path.join(matching);
                if rest.is_empty() {
                    p.to_string_lossy().to_string() + if p.is_dir() { "/" } else { "" }
                } else {
                    rest.to_owned()
                        + " "
                        + &*p.to_string_lossy().to_string()
                        + if p.is_dir() { "/" } else { "" }
                }
            };

            Some(res)
        } else {
            None
        }
    }
}
