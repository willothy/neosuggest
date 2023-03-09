use std::env::current_dir;
use std::path::PathBuf;

use ignore::{overrides::OverrideBuilder, WalkBuilder};
use ngrammatic::CorpusBuilder;

use crate::util::ExpandHome;

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
        let mut path = PathBuf::from(word);
        let home_abbr = path.starts_with("~");
        path = path.expand()?;
        let len = path.components().count();
        let (mut base_path, mut search) = if len > 1 {
            let search = path
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_string_lossy()
                .to_string();
            let path = path.parent()?;
            if !(path.exists() && path.is_dir()) {
                return None;
            }

            (path.to_path_buf(), search)
        } else {
            (pwd.clone(), word.to_owned())
        };

        let mut show_next = false;
        let path = if path.exists() && word.ends_with('/') {
            show_next = true;
            path
        } else {
            base_path.canonicalize().ok()?
        };

        let overrides = OverrideBuilder::new(&path)
            .add("!.git/")
            .ok()?
            .build()
            .ok()?;
        let start = search.chars().nth(0)?;
        let entries = WalkBuilder::new(&path)
            .max_depth(Some(1))
            .standard_filters(true)
            .overrides(overrides)
            .hidden(!search.starts_with('.'))
            .filter_entry(move |e| show_next || e.file_name().to_string_lossy().starts_with(start))
            .build()
            .filter_map(|e| e.ok().map(|v| v.file_name().to_string_lossy().to_string()))
            .skip(1)
            .collect::<Vec<_>>();

        if show_next || search == "" && !entries.is_empty() {
            let t = entries.iter().find(|s| !s.starts_with('.'))?.clone();
            search = t.to_owned();
        }
        let slen = search.len();
        let threshold = if slen > 7 { 0.7 } else { 0.10 * slen as f32 };
        let mut res = CorpusBuilder::new()
            .arity(2)
            .fill(entries)
            .finish()
            .search(&*search, threshold);
        if let Some(search_res) = res.first_mut() {
            let matching = if show_next {
                return Some(format!(
                    "{} {}{}",
                    rest,
                    &word,
                    std::mem::take(&mut search_res.text)
                ));
            } else {
                std::mem::take(&mut search_res.text)
            };

            let result = if base_path == pwd {
                let p = PathBuf::from(&matching);
                let (is_dir, p) = if home_abbr {
                    (p.is_dir(), p.unexpand()?.to_string_lossy().to_string())
                } else {
                    (p.is_dir(), p.to_string_lossy().to_string())
                };
                if rest.is_empty() {
                    p + if is_dir { "/" } else { "" }
                } else {
                    rest.to_owned() + " " + &p + if is_dir { "/" } else { "" }
                }
            } else {
                let p = base_path.join(matching);
                let (is_dir, p) = if home_abbr {
                    (p.is_dir(), p.unexpand()?.to_string_lossy().to_string())
                } else {
                    (p.is_dir(), p.to_string_lossy().to_string())
                };
                if rest.is_empty() {
                    p + if is_dir { "/" } else { "" }
                } else {
                    rest.to_owned() + " " + &p + if is_dir { "/" } else { "" }
                }
            };

            Some(
                result, // + &format!(
                       //     " \n{}",
                       //     res.iter()
                       //         .skip(1)
                       //         .map(|v| v.text.clone())
                       //         .collect::<Vec<_>>()
                       //         .join("\n")
                       // ),
            )
        } else {
            None
        }
    }
}
