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

- ###  BFS Traversal 

Breadth-First Search (BFS) is a graph traversal algorithm used to systematically explore a graph's nodes, layer by layer. Beginning from a chosen starting node, BFS iteratively visits adjacent nodes before moving deeper. It employs a queue to maintain the order of nodes to visit, ensuring all nodes at a particular level are explored before moving to the next level. BFS discovers the shortest path between nodes in unweighted graphs and efficiently finds if a path exists between two nodes.

   - Choose a starting node
   - Initialize data structures
   - Enqueue the starting node 
   - Iterate until the queue is empty:

     - Dequeue a node from the front of the queue.
     - Visit the dequeued node and process it (or collect its information).
     - Enqueue all unvisited neighboring nodes of the dequeued node.
     - Mark each newly visited node as visited and enqueue it.

![Animated Gif](https://lh5.googleusercontent.com/JKY4V8OZEs5L68Mh2ZY5ZqiqGkaj8esWnTEUwEdygFQdRFowh7aCWpibaPRqkcR3SHBh2Q4Io856f2fAzM5Ae3nD2uLj7AEU3NnQfZ55E2ni0EzXceoVTJtHzqGlUhQ9-izy5Y0v1DK0xIQ4vUxs9Ds)