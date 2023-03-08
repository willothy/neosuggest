use std::env::current_dir;
use std::path::PathBuf;

use ngrammatic::CorpusBuilder;

use super::Source;

pub struct Pwd;

#[async_trait::async_trait]
impl Source for Pwd {
    fn cond(&self, s: &str) -> bool {
        s.split_whitespace().count() > 1
    }

    async fn source(&self, search_word: &str) -> Option<String> {
        let word = search_word.split_whitespace().last()?;
        let rest = search_word[..search_word.len() - word.len()].trim_end();
        let pwd = current_dir().ok()?;
        let path = PathBuf::from(word);
        let len = path.components().count();
        let (base_path, search) = if len > 1 {
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
        // Shoud I use async here?? idk
        // let entries = tokio::fs::read_dir(&path).await.ok()?;
        // while let Some(entry) = entries.next_entry().await.ok()? {
        //     let is_dir = entry.path().is_dir();
        //     println!(
        //         "entry: {}{}",
        //         entry.file_name().to_string_lossy().to_string(),
        //         if is_dir { "/" } else { "" }
        //     );
        //     corpus.add_text(&*format!(
        //         "{}{}",
        //         entry.file_name().to_string_lossy().to_string(),
        //         if is_dir { "/" } else { "" }
        //     ));
        // }

        let slen = search.len();
        let threshold = if slen > 3 { 0.7 } else { 0.20 * slen as f32 };

        let entries = path
            .read_dir()
            .ok()?
            .filter_map(|e| e.ok().map(|v| v.file_name().to_string_lossy().to_string()));
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
