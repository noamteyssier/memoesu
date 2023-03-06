use hashbrown::HashMap;

type CanonCounts = HashMap<Vec<u64>, usize>;

#[derive(Debug)]
pub struct EnumResult {
    canon_counts: CanonCounts,
    num_subgraphs: usize,
    num_duplicates: usize,
}
impl EnumResult {
    pub fn new(canon_counts: CanonCounts, num_subgraphs: usize, num_duplicates: usize) -> Self {
        Self {
            canon_counts,
            num_subgraphs,
            num_duplicates,
        }
    }

    pub fn counts(&self) -> &CanonCounts {
        &self.canon_counts
    }

    pub fn total_subgraphs(&self) -> usize {
        self.num_subgraphs
    }

    pub fn unique_subgraphs(&self) -> usize {
        self.canon_counts.len()
    }

    pub fn num_duplicates(&self) -> usize {
        self.num_duplicates
    }
}
