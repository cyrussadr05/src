use std::collections::{HashMap, VecDeque, HashSet};
use std::error::Error;
use std::fs::File;
use csv::{ReaderBuilder, Trim};

pub type Graph = HashMap<String, Vec<String>>;

pub fn read_data(filename: &str) -> Result<Graph, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .trim(Trim::All)
        .from_reader(file);

    let mut graph = Graph::new();

    for result in rdr.records() {
        let record = result?;
        let id = record[0].trim().to_string();
        let friends_str = record[9].trim();

        let friends_cleaned = friends_str
            .trim_matches(|c: char| c == '[' || c == ']' || c == '"')
            .replace("\"", "");

        let friends: Vec<String> = friends_cleaned
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        graph.entry(id.clone()).or_default();
        for friend in friends {
            let friend_clone = friend.clone();
            graph.entry(friend_clone).or_default().push(id.clone());
            graph.entry(id.clone()).or_default().push(friend);
        }
    }

    Ok(graph)
}

pub fn bfs(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    visited.insert(start.to_string());
    queue.push_back((start.to_string(), 0));

    while let Some((current, dist)) = queue.pop_front() {
        distances.insert(current.clone(), dist);

        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.to_string());
                    queue.push_back((neighbor.to_string(), dist + 1));
                }
            }
        }
    }

    distances
}
