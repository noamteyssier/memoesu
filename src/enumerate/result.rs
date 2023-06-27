use crate::enumerate::{Counts, Groups};

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

pub struct GroupResult {
    groups: Groups,
    num_total_subgraphs: usize,
    num_unique_subgraphs: usize,
}
impl GroupResult {
    pub fn new(groups: Groups, num_total_subgraphs: usize, num_unique_subgraphs: usize) -> Self {
        Self { groups, num_total_subgraphs, num_unique_subgraphs }
    }

    pub fn groups(&self) -> &Groups {
        &self.groups
    }

    pub fn total_subgraphs(&self) -> usize {
        self.num_total_subgraphs
    }

    pub fn unique_subgraphs(&self) -> usize {
        self.num_unique_subgraphs
    }
}
