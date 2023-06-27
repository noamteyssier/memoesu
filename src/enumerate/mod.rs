mod bitgraph;
mod esu;
mod ngraph;
mod parallel_esu;
mod result;

use std::sync::Arc;

use ahash::HashMap;
pub use bitgraph::BitGraph;
pub use esu::{enumerate_subgraphs, group_subgraphs};
pub use ngraph::NautyGraph;
pub use parallel_esu::parallel_enumerate_subgraphs;
pub use result::{EnumResult, GroupResult};

pub type Counts = HashMap<Label, usize>;
pub type Label = Arc<[u64]>;
pub type Groups = HashMap<usize, HashMap<GroupInfo, usize>>;
pub type GroupInfo = (Label, NodeLabel, Orbit);
pub type Orbit = i32;
pub type NodeLabel = i32;
