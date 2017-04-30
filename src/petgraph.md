# petgraph
## graph data structure library
[![petgraph-badge]][petgraph]

### Example: Drawing the Petersen Graph
`petgraph` has powerful features for graph representation and algorithms. 
In the example below, we create and draw a graph in dot format.

*Key concepts:*
* Creating graphs from vertices and edges
* Printing in human readable form

```rust
extern crate petgraph;

use petgraph::Graph;
use petgraph::dot::{Dot, Config};

fn main() {
    // Create a new undirected graph, g
    let mut g = Graph::<u32, u32, petgraph::Undirected>::new_undirected();
    let w = 0;
    let mut v = Vec::new();

    // Add 10 vertices to G
    for i in 1..11 {
        v.push(g.add_node(i));
    }
    
    // Connect with 15 edges
    for i in 0..4 {
        g.add_edge(v[i], v[i + 1], w);
        g.add_edge(v[i], v[i + 5], w);
    }
    g.add_edge(v[0], v[4], w);
    g.add_edge(v[4], v[9], w);

    g.add_edge(v[5], v[7], w);
    g.add_edge(v[5], v[8], w);
    g.add_edge(v[6], v[8], w);
    g.add_edge(v[6], v[9], w);
    g.add_edge(v[7], v[9], w);
    
    // Print in graphviz dot format
    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}
```
![Petersen Graph](./graph.png)


<!-- Links -->

[petgraph-badge]: https://img.shields.io/crates/v/petgraph.svg?label=petgraph
[petgraph]: https://docs.rs/petgraph/0.4.3/petgraph/
