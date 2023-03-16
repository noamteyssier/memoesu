use ahash::HashMap;

type Counts = HashMap<Vec<u64>, usize>;

#[derive(Debug)]
pub struct EnumResult {
    canon_counts: Counts,
    num_subgraphs: usize,
}
impl EnumResult {
    pub fn new(canon_counts: Counts, num_subgraphs: usize) -> Self {
        Self {
            canon_counts,
            num_subgraphs,
        }
    }

    pub fn counts(&self) -> &Counts {
        &self.canon_counts
    }

    pub fn total_subgraphs(&self) -> usize {
        self.num_subgraphs
    }

    pub fn unique_subgraphs(&self) -> usize {
        self.canon_counts.len()
    }
}
