use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::iter::once;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/*-------------------------------------------------------------------------------------------------
  Day 23: LAN Party
-------------------------------------------------------------------------------------------------*/

fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let (_, connections) = parse_input_file(input);
    let node_map = build_node_graph(connections);

    let network_count = identify_sets_of_three_connected_nodes(&node_map)
        .iter()
        .filter(|nodes| nodes.iter().any(|node| node.starts_with("t")))
        .count();

    Some(network_count.to_string())
}

fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> Option<String> {
    let (_, connections) = parse_input_file(input);
    let node_map = build_node_graph(connections);

    let largest_network = find_largest_network(&node_map);

    Some(largest_network.join(","))
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Node = Rc<str>;
type NodeTriple = [Node; 3];
type Network = Vec<Node>;
type Connection = (Node, Node);
type NodeSet = HashSet<Node>;
type NodeMap = HashMap<Node, NodeSet>;

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> (NodeSet, Vec<Connection>) {
    let mut nodes = NodeSet::new();

    let connections: Vec<Connection> = read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split('-');
            let n1 = split.next().unwrap();
            let n2 = split.next().unwrap();
            let n1 = git_or_insert(&mut nodes, n1);
            let n2 = git_or_insert(&mut nodes, n2);
            (n1, n2)
        })
        .collect();

    (nodes, connections)
}

fn git_or_insert(node_set: &mut NodeSet, node: &str) -> Node {
    if let Some(existing_node) = node_set.get(node) {
        existing_node.clone()
    } else {
        let new_node: Rc<str> = Rc::from(node);
        node_set.insert(new_node.clone());
        new_node
    }
}

/*-----------------------------------------------------------------------------
  Build Node Graph
-----------------------------------------------------------------------------*/

fn build_node_graph(connections: Vec<Connection>) -> NodeMap {
    let mut node_map = NodeMap::new();

    for (n1, n2) in connections {
        let n1_connections = node_map.entry(n1.clone()).or_default();
        n1_connections.insert(n2.clone());

        let n2_connections = node_map.entry(n2.clone()).or_default();
        n2_connections.insert(n1.clone());
    }

    node_map
}

/*-----------------------------------------------------------------------------
  Identify Sets of Three Mutually Connected Nodes
-----------------------------------------------------------------------------*/

fn identify_sets_of_three_connected_nodes(node_map: &NodeMap) -> HashSet<NodeTriple> {
    let mut triples: HashSet<NodeTriple> = HashSet::new();
    for n1 in node_map.keys() {
        for n2 in node_map.get(n1).unwrap() {
            for n3 in node_map.get(n2).unwrap() {
                if node_map.get(n3).unwrap().contains(n1) {
                    let mut nodes: NodeTriple = [n1.clone(), n2.clone(), n3.clone()];
                    nodes.sort();
                    triples.insert(nodes);
                }
            }
        }
    }

    triples
}

/*-----------------------------------------------------------------------------
  Identify Largest Set of Mutually Connected Nodes
-----------------------------------------------------------------------------*/

fn find_largest_network(node_map: &NodeMap) -> Network {
    let mut networks: HashSet<Network> = HashSet::new();
    let mut largest_network: Network = Network::new();

    for start_node in node_map.keys() {
        let connected_nodes = node_map.get(start_node).unwrap();
        let mut stack = vec![(vec![start_node.clone()], once(start_node.clone()).collect())];
        while let Some((current_network, current_nodes)) = stack.pop() {
            if networks.contains(&current_network) {
                continue;
            }

            let remaining_nodes = connected_nodes - &current_nodes;

            for next_node in remaining_nodes {
                let next_node_connections = node_map.get(&next_node).unwrap();
                if next_node_connections.intersection(&current_nodes).count() == current_nodes.len()
                {
                    // Next node is connected to all nodes in the current network
                    let mut next_nodes = current_nodes.clone();
                    next_nodes.insert(next_node.clone());

                    let mut next_network: Network = next_nodes.iter().cloned().collect();
                    next_network.sort();

                    stack.push((next_network, next_nodes));
                }
            }

            if current_network.len() > largest_network.len() {
                largest_network = current_network.clone();
            }

            networks.insert(current_network);
        }
    }

    largest_network
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 23: LAN Party")]
pub enum Args {
    Part1 { input: PathBuf },
    Part2 { input: PathBuf },
}

pub fn main(args: Args) -> Option<String> {
    match args {
        Args::Part1 { input } => part1(&input),
        Args::Part2 { input } => part2(&input),
    }
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::answers::answer;

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1("../data/day23/example.txt"),
            answer("../data/day23/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day23/input.txt"),
            answer("../data/day23/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2("../data/day23/example.txt"),
            answer("../data/day23/example-part2-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2("../data/day23/input.txt"),
            answer("../data/day23/input-part2-answer.txt")
        );
    }
}
