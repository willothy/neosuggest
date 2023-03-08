use std::env::var;
use std::path::PathBuf;

use ngrammatic::CorpusBuilder;

use super::Source;

pub struct Path;

#[async_trait::async_trait]
impl Source for Path {
    async fn source(&self, search: &str) -> Option<String> {
        var("PATH").ok()?.split(":").try_for_each(|p| {
            let slen = search.len();
            let threshold = if slen > 3 { 0.5 } else { 0.1 * slen as f32 };

            let entries = PathBuf::from(p).read_dir().ok()?.filter_map(|e| {
                e.ok()
                    .filter(|v| !v.path().is_dir())
                    .map(|v| v.file_name().to_string_lossy().to_string())
            });
            let mut res = CorpusBuilder::new()
                .arity(2)
                .fill(entries)
                .finish()
                .search(&*search, threshold);

            None
        });

        None
    }
}
