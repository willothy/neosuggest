use ngrammatic::CorpusBuilder;

use super::Source;

pub struct Basics;

#[async_trait::async_trait]
impl Source for Basics {
    async fn source(&self, search: &str) -> Option<String> {
        let mut res = CorpusBuilder::new()
            .arity(2)
            .fill(["cd", "pwd"])
            .finish()
            .search(search, 0.4);
        res.get_mut(0).map(|r| std::mem::take(&mut r.text))
    }
}
