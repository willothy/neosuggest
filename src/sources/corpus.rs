use ngrammatic::Corpus;

use super::Source;

#[async_trait::async_trait]
impl Source for Corpus {
    async fn source(&self, word: &str) -> Option<String> {
        self.search(word, 0.25).first().map(|m| m.text.clone())
    }
}
