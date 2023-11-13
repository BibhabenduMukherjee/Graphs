use std::collections::HashMap;
use std::io;

fn main() {
    // Read the number of vertices in the graph
    println!("Enter the number of vertices:");
    let mut num_vertices = String::new();
    io::stdin().read_line(&mut num_vertices).expect("Failed to read line");
    let num_vertices: usize = num_vertices.trim().parse().expect("Invalid input for number of vertices");

    // Initialize an empty adjacency list
    let  adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();

    // Read the adjacency list values
    println!("Enter the adjacency list (vertex followed by its neighbors, space-separated):");
    for i in 0..num_vertices {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let neighbors: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for adjacency list"))
            .collect();

        adjacency_list.insert(i, neighbors);
    }

    // Display the adjacency list
    println!("Adjacency List:");
    for (vertex, neighbors) in &adjacency_list {
        print!("{}: ", vertex);
        for neighbor in neighbors {
            print!("{} ", neighbor);
        }
        println!();
    }
}
