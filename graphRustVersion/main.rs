use std::io;

fn main() {
    // Read the number of vertices in the graph
    println!("Enter the number of vertices:");
    let mut num_vertices = String::new();
    io::stdin().read_line(&mut num_vertices).expect("Failed to read line");
    let num_vertices: usize = num_vertices.trim().parse().expect("Invalid input for number of vertices");

    // Initialize an empty adjacency matrix
    let mut adjacency_matrix = vec![vec![0; num_vertices]; num_vertices];

    // Read the adjacency matrix values
    println!("Enter the adjacency matrix (row by row):");
    for i in 0..num_vertices {
        let mut row = String::new();
        io::stdin().read_line(&mut row).expect("Failed to read line");
        let row_values: Vec<u32> = row
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for adjacency matrix"))
            .collect();

        if row_values.len() != num_vertices {
            panic!("Invalid number of values in the row. Should be equal to the number of vertices.");
        }

        adjacency_matrix[i] = row_values;
    }

    // Display the adjacency matrix
    println!("Adjacency Matrix:");
    for row in &adjacency_matrix {
        for value in row {
            print!("{:4} ", value);
        }
        println!();
    }
}
