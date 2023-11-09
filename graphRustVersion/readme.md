## [1 . Graph as input in the form of an adjacency matrix ](https://github.com/BibhabenduMukherjee/Graphs/blob/main/graphRustVersion/GraphInputAdjMatrix.rs)

- We first read the number of vertices in the graph from the user.

- We then create an empty adjacency matrix as a 2D vector with all values initialized to 0.
```cpp
let mut adjacency_matrix = vec![vec![0; num_vertices]; num_vertices];
```
- Next, we read the adjacency matrix values row by row from the user. We split each row's input into space-separated values, parse them into u32, and store them in the adjacency matrix.
```cpp
 let row_values: Vec<u32> = row
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for adjacency matrix"))
            .collect();
```

- Finally, we display the adjacency matrix to verify that it has been correctly input.


![Animated Gif](https://lh6.googleusercontent.com/RJXRgXqBBveetZajhikyA8q29MhMzKKDUXH0WuQb9-k7owhUIi6rZxqvyU-gNicg7HqRYHOE44_C5ojbCpQlbi0QDIn2ZuwWUA6LD5wYSWz6ehdDthINzqofzZ5NZJyEbShmYhJmn7j7IZFlhvyh_uY)