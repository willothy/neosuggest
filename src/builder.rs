use crate::sources::{Source, Sources};
use itertools::Itertools;
use std::sync::Arc;

pub struct SourcesBuilder {
    sources: Vec<Arc<dyn Source>>,
    priorities: Vec<usize>,
}

impl SourcesBuilder {
    pub fn new() -> Self {
        Self {
            sources: vec![],
            priorities: vec![],
        }
    }

    pub fn using<T: Source + 'static>(mut self, source: T) -> Self {
        self.priorities.push(source.priority());
        self.sources.push(Arc::new(source) as Arc<dyn Source>);
        self
    }

    pub fn with_priority(mut self, mut priority: usize) -> Self {
        self.priorities.last_mut().replace(&mut priority);
        self
    }

    pub fn finalize(self) -> Sources {
        // Sort sources by priority
        let (sources, _): (Vec<Arc<dyn Source>>, Vec<usize>) = self
            .sources
            .into_iter()
            .zip(self.priorities)
            .sorted_by(|(_, p1), (_, p2)| p1.cmp(&p2))
            .unzip();
        Sources::new(sources)
    }
}
