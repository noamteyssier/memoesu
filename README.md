# MEMO-ESU

Enumeration of subgraphs using a memoized parallel ESU algorithm.

## Background

Subgraph enumeration is the process of counting how many times a specific
subgraph appears in a larger graph.

This requires traversing the graph, avoiding double counting sets of nodes,
and calculating isomorphism for each subgraph to increments its abundance.

The ESU algorithm was described by Wernicke in 2005<sup>[1](#references)</sup>
which describes a graph traversal method similar to DFS but only following child
nodes with larger node labels.

This is a very fast subgraph identification method, but at the end of every subgraph
identification a call is made to NAUTY<sup>[2](#references)</sup> to calculate that
subgraphs canonical labeling, which is the rate limiting step of the algorithm.

This program is a rust implementation of the ESU algorithm, but with an
additional memoization step to avoid making multiple calls to NAUTY by hashing
the bitvector representing the adjacency matrix of the subgraph.
It also allows the user to run the ESU algorithm in parallel across multiple
threads to speed up the enumeration.

## Usage

### Enumeration

The basic usage of this tool is to run the `enumerate` subcommand, which accepts at minimum
the path to a plaintext graph and the size of the subgraphs to enumerate.

In the following command we enumerate all size 4 subgraphs in the ecoli graph.

```bash
memoesu enumerate -i example/ecoli.txt -s 4
```

By default, the graph is assumed to be directed, but you can also force
the graph to be undirected and count all undirected subgraphs.

```bash
memoesu enumerate -i example/ecoli.txt -s 4 -u
```

You can also specify multiple threads, in this case 8.

```bash
memoesu enumerate -i example/ecoli.txt -s 4 -t 8
```

### Format

`memoesu` will only accept networks with integer label graphs.

However, you can reformat a string labeled graph into an integer graph
using the `format` subcommand

```bash
memoesu enumerate -i example/unformatted.txt -o formatted
```

Which will generate two new files with the `formatted` prefix:

`formatted.network.tsv` and `formatted.dictionary.tsv`

which give the integer labeled network and a dictionary relating every
label to their respective integer.

### Switch

To calculate network motifs we need to first create a background set of random
graphs that are comparable to the original network.

The method we employ here is the random switching method, originally used in
the `mfinder` tool, and described by Milo<sup>[3](#references)</sup>, which
describes an algorithm to generate random graphs with equivalent degree
sequences to the original graph.

To perform the switching algorithm to generate a random graph we can use
the `switch` subcommand:

```bash
memoesu switch -i example/example.txt
```

This creates a new random graph with an identical degree sequence to the original
graph.

## References

1. S. Wernicke, “Efficient Detection of Network Motifs,” IEEE/ACM Trans. Comput. Biol. and Bioinf., vol. 3, no. 4, pp. 347–359, Oct. 2006, doi: 10.1109/TCBB.2006.51.
2. B. D. McKay and A. Piperno, “Practical graph isomorphism, II,” Journal of Symbolic Computation, vol. 60, pp. 94–112, Jan. 2014, doi: 10.1016/j.jsc.2013.09.003.
3. R. Milo, N. Kashtan, S. Itzkovitz, M. E. J. Newman, and U. Alon, “On the uniform generation of random graphs with prescribed degree sequences.” arXiv, May 30, 2004. Accessed: Jun. 26, 2023. [Online]. Available: http://arxiv.org/abs/cond-mat/0312028
