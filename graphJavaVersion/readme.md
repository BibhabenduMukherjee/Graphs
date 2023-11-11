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



### Animation 

![Animated Gif](https://lh6.googleusercontent.com/RJXRgXqBBveetZajhikyA8q29MhMzKKDUXH0WuQb9-k7owhUIi6rZxqvyU-gNicg7HqRYHOE44_C5ojbCpQlbi0QDIn2ZuwWUA6LD5wYSWz6ehdDthINzqofzZ5NZJyEbShmYhJmn7j7IZFlhvyh_uY)