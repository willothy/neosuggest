use std::collections::HashMap;
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
        let pwd = current_dir().ok()?;
        let (mut path, word) = if split.clone().count() > 1 {
            let word = split.last()?;
            (PathBuf::from(word), word)
        } else {
            (pwd.clone().unexpand()?, "")
        };
        let rest = search_word[..search_word.len() - word.len()].trim_end();
        let home_abbr = path.starts_with("~");
        path = path.expand()?;
        let len = path.components().count();
        let (base_path, mut search) = if len > 1 && word != "" {
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
        let path = if search == "" || (path.exists() && word.ends_with('/')) {
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
        let start = if show_next {
            ' '
        } else {
            search.chars().nth(0)?
        };
        let entries = WalkBuilder::new(&path)
            .max_depth(Some(1))
            .standard_filters(true)
            .overrides(overrides)
            .hidden(!search.starts_with('.'))
            .filter_entry(move |e| show_next || e.file_name().to_string_lossy().starts_with(start))
            .build()
            .filter_map(|e| {
                e.ok().map(|v| {
                    (
                        v.file_name().to_string_lossy().to_string(),
                        (
                            v.metadata().ok().and_then(|m| m.accessed().ok()),
                            v.path().is_dir(),
                        ),
                    )
                })
            })
            .skip(1)
            .collect::<HashMap<_, _>>();
        if (show_next || search == "") && !entries.is_empty() {
            search = entries.keys().find(|s| !s.starts_with('.'))?.clone();
        }
        let slen = search.len();
        let threshold = if slen > 7 { 0.7 } else { 0.10 * slen as f32 };

        let mut res = CorpusBuilder::new()
            .arity(2)
            .fill(entries.keys())
            .finish()
            .search(&*search, threshold);
        let res_count = res.len();
        if res_count == 0 {
            return None;
        } else if res_count > 2 {
            // Average similarity
            let mut mean_similarity = 0.;
            res.iter().for_each(|r| mean_similarity += r.similarity);
            mean_similarity /= res_count as f32;
            // Filter by above average match
            res = res
                .into_iter()
                .filter(|r| r.similarity > mean_similarity)
                .collect();
        }
        // Sort by similarity
        res.sort_unstable_by(|x, y| entries.get(&x.text).cmp(&entries.get(&y.text)));

        if let Some(search_res) = res.first_mut() {
            let matching = std::mem::take(&mut search_res.text);
            let is_dir = entries.get(&matching).map(|(_, d)| *d).unwrap_or(false);

            if show_next {
                let dir = if is_dir { "/" } else { "" };
                return Some(format!("{} {}{}", rest, matching, dir,));
            }

            let path = if base_path == pwd {
                let p = PathBuf::from(&matching);
                if home_abbr {
                    p.unexpand()?.to_string_lossy().to_string()
                } else {
                    p.to_string_lossy().to_string()
                }
            } else {
                let p = base_path.join(matching);
                if home_abbr {
                    p.unexpand()?.to_string_lossy().to_string()
                } else {
                    p.to_string_lossy().to_string()
                }
            };
            let result = if rest.is_empty() {
                path + if is_dir { "/" } else { "" }
            } else {
                rest.to_owned() + " " + &path + if is_dir { "/" } else { "" }
            };
            Some(result)
        } else {
            None
        }
    }
}
