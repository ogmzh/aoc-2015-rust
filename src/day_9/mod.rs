use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Result},
};

#[derive(Debug)]
struct Graph {
    vertices: HashSet<String>,
    edges: HashMap<(String, String), i32>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            vertices: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: &str, to: &str, distance: i32) {
        self.vertices.insert(from.to_string());
        self.vertices.insert(to.to_string());

        self.edges
            .insert((from.to_string(), to.to_string()), distance);
        self.edges
            .insert((to.to_string(), from.to_string()), distance);
    }

    fn get_distance(&self, from: &str, to: &str) -> i32 {
        *self.edges.get(&(from.to_string(), to.to_string())).unwrap()
    }
}

pub struct Day9 {}

impl Day9 {
    pub fn part1() -> Result<usize> {
        let file = File::open("src/day_9/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 9 input file: {e}"));
        let reader = BufReader::new(file);
        let mut graph = Graph::new();
        for line in reader.lines() {
            let line = line?;
            let split = line.split(" = ").collect::<Vec<&str>>();
            let vertices = split[0].split(" to ").collect::<Vec<&str>>();
            graph.add_edge(vertices[0], vertices[1], split[1].parse::<i32>().unwrap());
        }

        let mut shortest_distance = i32::MAX;
        for start_city in graph.vertices.iter() {
            let mut remaining = graph.vertices.clone();
            remaining.remove(start_city);
            let distance =
                Self::find_shortest_route(start_city, &remaining, 0, shortest_distance, &graph);
            shortest_distance = shortest_distance.min(distance);
        }

        Ok(shortest_distance as usize)
    }

    pub fn part2() -> Result<usize> {
        let file = File::open("src/day_9/input.txt")
            .unwrap_or_else(|e| panic!("could not open day 9 input file: {e}"));
        let reader = BufReader::new(file);
        let mut graph = Graph::new();
        for line in reader.lines() {
            let line = line?;
            let split = line.split(" = ").collect::<Vec<&str>>();
            let vertices = split[0].split(" to ").collect::<Vec<&str>>();
            graph.add_edge(vertices[0], vertices[1], split[1].parse::<i32>().unwrap());
        }

        let mut longest_distance = 0;
        for start_city in graph.vertices.iter() {
            let mut remaining = graph.vertices.clone();
            remaining.remove(start_city);
            let distance =
                Self::find_longest_route(start_city, &remaining, 0, longest_distance, &graph);
            longest_distance = longest_distance.max(distance);
        }

        Ok(longest_distance as usize)
    }

    fn find_shortest_route(
        current_city: &str,
        cities: &HashSet<String>,
        current_distance: i32,
        shortest: i32,
        graph: &Graph,
    ) -> i32 {
        if current_distance > shortest {
            return shortest;
        }

        if cities.is_empty() {
            return current_distance;
        }

        let mut min_distance = shortest;

        for next_city in cities.iter() {
            let next_distance = graph.get_distance(current_city, next_city);
            let mut remaining = cities.clone();
            remaining.remove(next_city);

            let total_distance = Self::find_shortest_route(
                next_city,
                &remaining,
                current_distance + next_distance,
                min_distance,
                graph,
            );

            if total_distance < min_distance {
                min_distance = total_distance;
            }
        }

        min_distance
    }

    fn find_longest_route(
        current_city: &str,
        remaining: &HashSet<String>,
        current_distance: i32,
        longest_distance: i32,
        graph: &Graph,
    ) -> i32 {
        if remaining.is_empty() {
            return current_distance;
        }

        let mut max_distance = longest_distance;
        for next_city in remaining.iter() {
            let next_distance = graph.get_distance(current_city, next_city);
            let mut remaining_cities = remaining.clone();

            remaining_cities.remove(next_city);
            let total_distance = Self::find_longest_route(
                next_city,
                &remaining_cities,
                current_distance + next_distance,
                max_distance,
                graph,
            );

            max_distance = max_distance.max(total_distance);
        }
        max_distance
    }
}
