## [1 . Graph as input in the form of an adjacency matrix ](https://github.com/BibhabenduMukherjee/Graphs/blob/main/graphJavaVersion/GraphInputAdjMatrix.java)

```java
        Scanner scanner = new Scanner(System.in);

        System.out.println("Enter the number of vertices in the graph:");
        int numVertices = scanner.nextInt();

        System.out.println("Enter the number of edges in the graph:");
        int numEdges = scanner.nextInt();

        int[][] adjacencyMatrix = new int[numVertices][numVertices]; 

```

This block of code creates a new Scanner object to read input from the user. It then asks the user to enter the number of vertices and edges in the graph. Finally, it creates a new two-dimensional array called adjacencyMatrix to store the adjacency matrix of the graph. The array has dimensions numVertices x numVertices.


```java
            for (int i = 0; i < numEdges; i++) {
            System.out.println("Enter the source vertex for edge " + (i + 1) + ":");
            int sourceVertex = scanner.nextInt();

            System.out.println("Enter the destination vertex for edge " + (i + 1) + ":");
            int destinationVertex = scanner.nextInt();

            adjacencyMatrix[sourceVertex][destinationVertex] = 1;
            adjacencyMatrix[destinationVertex][sourceVertex] = 1; // For undirected graphs
        }

```

This for loop reads the edges of the graph from the user and updates the adjacency matrix accordingly. For each edge, the loop sets the corresponding element in the adjacency matrix to 1. For undirected graphs, the loop also sets the corresponding element in the reverse direction to 1.


```java

        System.out.println("The adjacency matrix is:");
        for (int i = 0; i < numVertices; i++) {
            for (int j = 0; j < numVertices; j++) {
                System.out.print(adjacencyMatrix[i][j] + " ");
            }
            System.out.println();
        }
        scanner.close();


```
This block of code prints the adjacency matrix to the console. It then closes the Scanner object.

Overall, the code creates an adjacency matrix for a graph based on the input provided by the user.


### Animation 

![Animated Gif](https://lh6.googleusercontent.com/RJXRgXqBBveetZajhikyA8q29MhMzKKDUXH0WuQb9-k7owhUIi6rZxqvyU-gNicg7HqRYHOE44_C5ojbCpQlbi0QDIn2ZuwWUA6LD5wYSWz6ehdDthINzqofzZ5NZJyEbShmYhJmn7j7IZFlhvyh_uY)


## [2 . Bfs Traversal ](https://github.com/BibhabenduMukherjee/Graphs/blob/main/graphJavaVersion/BfsTraversal.java)

BFS, or Breadth-First Search, is a powerful algorithm used to traverse or search tree and graph data structures. It prioritizes exploring all the direct neighbors of a node before moving on to the next level. Imagine exploring a maze by visiting all the rooms connected to the current one before venturing further down any corridors.

## [3 . Dfs Traversal ](https://github.com/BibhabenduMukherjee/Graphs/blob/main/graphJavaVersion/DfsTraversal.java)

DFS, or Depth-First Search, is another powerful algorithm for traversing or searching tree and graph data structures. Unlike BFS, it prioritizes exploring one branch as deep as possible before backtracking and considering other options. Imagine exploring a maze by venturing down a single path until you reach a dead end, then backtracking and trying another path. That's the essence of DFS!




