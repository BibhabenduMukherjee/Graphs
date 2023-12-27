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


- ### Number of Islands
Problem Statement: Given a grid of size NxM (N is the number of rows and M is the number of columns in the grid) consisting of ‘0’s (Water) and ‘1’s(Land). Find the number of islands.
Note: An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically or diagonally i.e., in all 8 directions.


![Alt text](https://takeuforward.org/wp-content/uploads/2022/08/eees.png)

There are 3 islands as the different components are surrounded by water (i.e. 0), and there is no land connectivity in either of the 8 directions hence separating them into 3 islands.

![Alt text](https://takeuforward.org/wp-content/uploads/2022/08/image-4.png)

All lands are connected. So, only 1 island is present.

Approach:

In any traversal technique, we have one starting node and it traverses all the nodes in the graph. We know about both the traversals, Breadth First Search (BFS) and Depth First Search (DFS). We can use any of the traversals to solve this problem, in our case we will be using BFS.
The algorithm steps are as follows:
 - The pairs of row and column (<row, column>) will represent the node numbers.
 - For BFS traversal, we need a queue data structure and a visited array. Create a replica of the given array, i.e., create another array of the same size and call it a visited array. We can use the same matrix, but we will avoid alteration of the original data. 
 - In the queue, insert a vertex (pair of <row, column>) and mark it as visited. 
 - While BFS traversal, pop out an element from the queue and travel to all its neighbours. In a graph, we store the list of neighbours in an adjacency list but here we know the neighbours are in 8 directions. 
 - We go in all 8 directions and check for unvisited land neighbours. To travel in 8 directions we will use nested loops, you can find the implementation details in the code. 
 - BFS function call will make sure that it starts the BFS call from that unvisited land, and visits all the nodes that are on that island, and at the same time, it will also mark them as visited. 
- Since the nodes travelled in a traversal will be marked as visited, they will no further be called for any further BFS traversal. 
 - Keep repeating these steps, for every land that you find unvisited, and visit the entire island. 
Add a counter variable to count the number of times the BFS function is called