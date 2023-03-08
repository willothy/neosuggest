use std::sync::Arc;

pub mod corpus;
pub mod path;
pub mod pwd;
pub mod zoxide;

#[async_trait::async_trait]
pub trait Source
where
    Self: Send + Sync,
{
    /// Only enable for specific word types
    fn cond(&self, _word: &str) -> bool {
        true
    }

    /// Get/set priority for the source
    fn priority(&self) -> usize {
        0
    }

    /// Search the source for a suggestion
    async fn source(&self, word: &str) -> Option<String>;
}

pub struct Sources {
    sources: Vec<Arc<dyn Source>>,
}

impl Sources {
    pub fn new(sources: Vec<Arc<dyn Source>>) -> Self {
        Self { sources }
    }

    pub async fn search(&self, word: String) -> Option<String> {
        let tasks = self
            .sources
            .iter()
            .cloned()
            .filter_map(|source| {
                if source.cond(&*word) {
                    Some(tokio::task::spawn({
                        let word = word.clone();
                        async move { source.source(&*word).await }
                    }))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if tasks.is_empty() {
            return None;
        };
        let r = futures::future::join_all(tasks)
            .await
            .into_iter()
            .filter_map(|r| r.ok().flatten())
            .collect::<Vec<_>>();
        r.get(0).cloned()
    }
}
