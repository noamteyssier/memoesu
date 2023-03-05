use hashbrown::HashMap;
use ndarray::Array1;
use petgraph::{Graph, Directed};
use rand::{SeedableRng, Rng};
use rand_chacha::ChaChaRng;
use crate::{enumerate::{enumerate_subgraphs, EnumResult}, switching::switching};

pub struct EnrichResult {
    pub subgraphs: Vec<Vec<u64>>,
    pub abundances: Vec<usize>,
    pub frequencies: Vec<f64>,
    pub mean_random_frequency: Vec<f64>,
    pub std_random_frequency: Vec<f64>,
    pub zscores: Vec<f64>,
}
impl EnrichResult {
    pub fn len(&self) -> usize {
        self.subgraphs.len()
    }
}

pub fn enrichment(
    graph: &Graph<(), (), Directed>,
    k: usize,
    num_random_graphs: usize,
    q: usize,
    seed: Option<usize>,
) -> EnrichResult {
    let original_results = enumerate_subgraphs(graph, k);
    let mut rng = ChaChaRng::seed_from_u64(seed.unwrap_or(rand::random()) as u64);
    let mut null_map = initialize_null_map(&original_results, num_random_graphs);

    for idx in 0..num_random_graphs {
        let random_seed = rng.gen();
        let random_graph = switching(&graph, q, random_seed);
        let random_results = enumerate_subgraphs(&random_graph, k);
        for key in original_results.counts().keys() {
            if let Some(v) = random_results.counts().get(key) {
                null_map.get_mut(key).unwrap()[idx] = *v as f64;
            } else {
                null_map.get_mut(key).unwrap()[idx] = 0.;
            }
        }
    }

    assemble_results(&original_results, null_map)
}

fn assemble_results(original_results: &EnumResult, null_map: HashMap<&Vec<u64>, Array1<f64>>) -> EnrichResult {
    let num_subgraphs = original_results.total_subgraphs();
    let num_unique = original_results.unique_subgraphs();

    let mut subgraphs = Vec::with_capacity(num_unique);
    let mut abundances = Vec::with_capacity(num_unique);
    let mut frequencies = Vec::with_capacity(num_unique);
    let mut mean_random_frequency = Vec::with_capacity(num_unique);
    let mut std_random_frequency = Vec::with_capacity(num_unique);
    let mut zscores = Vec::with_capacity(num_unique);

    for (key, abundance) in original_results.counts() {

        // Calculate the frequency of this subgraph in the original graph
        let frequency = *abundance as f64 / num_subgraphs as f64;

        // Get the null values for this subgraph
        let null_values = null_map.get(key).unwrap();

        // Calculate the mean and std of the null values
        let mean = null_values.mean().unwrap();
        let std = null_values.std(0.0);

        // Calculate the zscore and adjust to zero if infinite
        let mut zscore = (*abundance as f64 - mean) / std;
        if zscore.is_infinite() {
            zscore = 0.;
        }

        subgraphs.push(key.clone());
        abundances.push(*abundance);
        frequencies.push(frequency);
        zscores.push(zscore);
        mean_random_frequency.push(mean);
        std_random_frequency.push(std);
    }

    EnrichResult {
        subgraphs,
        abundances,
        frequencies,
        mean_random_frequency,
        std_random_frequency,
        zscores,
    }
}

fn initialize_null_map<'a>(
    results: &'a EnumResult, 

    num_random_graphs: usize
) -> HashMap<&'a Vec<u64>, Array1<f64>> {
    let mut null_map = HashMap::with_capacity(results.counts().len());
    for key in results.counts().keys() {
        null_map.insert(key, Array1::zeros(num_random_graphs));
    }
    null_map
}
