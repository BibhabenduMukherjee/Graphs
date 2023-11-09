package graphJavaVersion;
import java.util.Scanner;
//here is an example of how to create an adjacency matrix and take input of a graph in Java:
public class GraphInputAdjMatrix {

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        System.out.println("Enter the number of vertices in the graph:");
        int numVertices = scanner.nextInt();

        System.out.println("Enter the number of edges in the graph:");
        int numEdges = scanner.nextInt();

        int[][] adjacencyMatrix = new int[numVertices][numVertices];

        for (int i = 0; i < numEdges; i++) {
            System.out.println("Enter the source vertex for edge " + (i + 1) + ":");
            int sourceVertex = scanner.nextInt();

            System.out.println("Enter the destination vertex for edge " + (i + 1) + ":");
            int destinationVertex = scanner.nextInt();

            adjacencyMatrix[sourceVertex][destinationVertex] = 1;
            adjacencyMatrix[destinationVertex][sourceVertex] = 1; // For undirected graphs
        }

        System.out.println("The adjacency matrix is:");
        for (int i = 0; i < numVertices; i++) {
            for (int j = 0; j < numVertices; j++) {
                System.out.print(adjacencyMatrix[i][j] + " ");
            }
            System.out.println();
        }
        scanner.close();
    }
}
