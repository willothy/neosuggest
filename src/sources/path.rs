use std::env::var;

use ngrammatic::CorpusBuilder;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::path::PathBuf;

use super::Source;

pub struct Path;

#[async_trait::async_trait]
impl Source for Path {
    async fn source(&self, search: &str) -> Option<String> {
        // var("PATH").ok()?.split(":").find_map(|p| {
        //     let slen = search.len();
        //     let threshold = if slen > 3 { 0.5 } else { 0.1 * slen as f32 };
        //
        //     let entries = PathBuf::from(p)
        //         .read_dir()
        //         .ok()?
        //         .filter_map(|e| e.ok())
        //         .filter(|v| !v.path().is_dir())
        //         .map(|v| v.file_name().to_string_lossy().to_string())
        //         .collect::<Vec<String>>();
        //     let mut res = CorpusBuilder::new()
        //         .arity(2)
        //         .fill(entries)
        //         .finish()
        //         .search(&*search, threshold);
        //     println!("{:?}", res);
        //
        //     res.get_mut(0).map(|r| std::mem::take(&mut r.text))
        // })
        // ------------------------------------------------
        // for path in paths {
        //     // let entries = path
        //     //     .read_dir()
        //     //     .ok()?
        //     //     .filter_map(|e| e.ok())
        //     //     .filter(|v| !v.path().is_dir())
        //     //     .map(|v| v.file_name().to_string_lossy().to_string())
        //     //     .collect::<Vec<String>>();
        //     // let mut res = CorpusBuilder::new()
        //     //     .arity(2)
        //     //     .fill(entries)
        //     //     .finish()
        //     //     .search(&*search, threshold);
        //     // println!("{:?}", res);
        // }
        // let mut res = corpus.search(&*search, threshold);

        let path = var("PATH").ok()?;
        let start = &search[0..search.len().min(2)];
        let slen = search.len();
        let threshold = if slen > 3 { 0.4 } else { 0.1 * slen as f32 };
        let paths = path
            .split(":")
            .collect::<Vec<_>>()
            .into_par_iter()
            .filter_map(|path| {
                let path = PathBuf::from(path);
                let Ok(mut entries) = path.read_dir() else {
					return None;
				};
                let mut results = vec![];
                loop {
                    match entries.next() {
                        Some(Ok(e)) => {
                            if e.path().is_dir() {
                                continue;
                            }
                            let name = e.file_name().to_string_lossy().to_string();
                            if name.starts_with(start) {
                                results.push(name);
                            }
                        }
                        Some(Err(_)) => continue,
                        None => {
                            if results.is_empty() {
                                return None;
                            } else {
                                return Some(results);
                            }
                        }
                    }
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        let mut res = CorpusBuilder::new()
            .arity(5)
            .fill(paths)
            .finish()
            .search(&*search, threshold);

        res.get_mut(0).map(|r| std::mem::take(&mut r.text))
    }
}
