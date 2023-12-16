# Complete Graph Series
- [C++ Series](https://github.com/BibhabenduMukherjee/Graphs/tree/main/graphCppVersion)

- [Java Series](https://github.com/BibhabenduMukherjee/Graphs/tree/main/graphJavaVersion)

- [Rust Series](https://github.com/BibhabenduMukherjee/Graphs/tree/main/graphRustVersion)


## List of All Concepts
- Graph Representation | C++
- Connected Components | Logic Explanation only
- BFS,DFS Overview
- Problems on BFS and DFS 
  - Word Ladder1/2
  - Bipertite Graph,Cycle detection
  - Number of Distinct Islands 
  - Cycle Detection in Directed Graph (DFS)
  - Connected Components Problem in Matrix
  - Rotten Oranges
- Problems on Toposort
  - Topo Sort
  - Kahn's Algo
  - Cycle Detection in Directed Graph (BFS)
  - Course Schedule - I
  - Course Schedule - II
  - Find eventual safe states
- Shortest Path Algorithm 
  - Shortest Path in UG with unit weights
  - Shortest Path in UG with unit weights

- Minimum Spanning Tree 
  - Prim's Algorithm
  - Disjoint Set [Union by Rank]
  - Accounts merge



## Some concepts that is important to understand 
 - ### Connected Components 
  In graph theory, connected components refer to the distinct subgraphs within a larger graph where each node is reachable from every other node in that subgraph, directly or indirectly.

  Imagine a simple example:

  Let's say you have a graph with 7 nodes and 6 edges:

  Nodes: A, B, C, D, E, F, G
  Edges: (A-B), (B-C), (C-A), (D-E), (F-G)

  In this case, the graph has two connected components:

Component 1: {A, B, C}
Component 2: {D, E} and {F, G} are separate isolated components as there are no connections between them or to the rest of the nodes.
 
Each node within a connected component can reach any other node in the same component by traversing edges in the graph. Components can vary in size, from a single node (in the case of isolated nodes) to encompassing the entire graph