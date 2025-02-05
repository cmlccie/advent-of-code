use crate::get_input;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;
use std::rc::Rc;

/*-------------------------------------------------------------------------------------------------
  Day 23: LAN Party
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let node_graph = parse_input(input);
    let network_count = count_sets_of_three_mutually_connected_nodes(&node_graph);
    Some(network_count.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let node_graph = parse_input(input);

    let largest_network = node_graph
        .iter()
        .map(|node| find_largest_network(&node_graph, vec![node.id]))
        .fold(Network::new(), |acc, network| {
            if network.len() > acc.len() {
                network
            } else {
                acc
            }
        });

    let largest_network: Vec<Rc<str>> = largest_network
        .iter()
        .map(|id| node_graph.nodes[*id].name.clone())
        .collect();

    Some(largest_network.join(","))
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type NodeName = Rc<str>;
type NodeID = usize;

type NodeCount = usize;
type Network = Vec<NodeID>;

fn parse_input(input: &str) -> NodeGraph {
    let mut temp_graph: BTreeMap<NodeName, BTreeSet<NodeName>> = BTreeMap::new();

    for line in input.lines() {
        let mut split = line.split('-');
        let n1 = split.next().unwrap();
        let n2 = split.next().unwrap();
        temp_graph.entry(n1.into()).or_default().insert(n2.into());
        temp_graph.entry(n2.into()).or_default().insert(n1.into());
    }

    let name_index: HashMap<NodeName, NodeID> = temp_graph
        .keys()
        .enumerate()
        .map(|(index, node_name)| (node_name.clone(), index))
        .collect();

    temp_graph
        .into_iter()
        .map(|(node_name, connections)| {
            Node::new(
                node_name.clone(),
                *name_index.get(&node_name).unwrap(),
                connections
                    .into_iter()
                    .map(|connected_node| *name_index.get(&connected_node).unwrap())
                    .collect(),
            )
        })
        .collect()
}

/*--------------------------------------------------------------------------------------
  Node Graph
--------------------------------------------------------------------------------------*/

struct NodeGraph {
    nodes: Vec<Node>,
}

impl NodeGraph {
    fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }

    fn forward_nodes<'g, 'n>(
        &'g self,
        node: &'n Node,
    ) -> impl Iterator<Item = &'g Node> + use<'g, 'n> {
        node.forward_connections().iter().map(|&id| &self.nodes[id])
    }
}

impl FromIterator<Node> for NodeGraph {
    fn from_iter<I: IntoIterator<Item = Node>>(iter: I) -> Self {
        let nodes: Vec<Node> = iter.into_iter().collect();
        Self { nodes }
    }
}

/*-----------------------------------------------------------------------------
  Node
-----------------------------------------------------------------------------*/

struct Node {
    name: NodeName,
    id: NodeID,
    connections: Vec<NodeID>,
    forward_index: usize,
}

impl Node {
    fn new(name: NodeName, id: NodeID, connections: Vec<NodeID>) -> Self {
        let index_greater_than_self = connections
            .iter()
            .position(|&connection| connection > id)
            .unwrap_or(connections.len());

        Self {
            name,
            id,
            connections,
            forward_index: index_greater_than_self,
        }
    }

    fn forward_connections(&self) -> &[NodeID] {
        &self.connections[self.forward_index..]
    }

    fn connects_to(&self, id: &NodeID) -> bool {
        self.connections.binary_search(id).is_ok()
    }
}

/*-----------------------------------------------------------------------------
  Identify Sets of Three Mutually Connected Nodes That Contain a `t*` Node
-----------------------------------------------------------------------------*/

fn count_sets_of_three_mutually_connected_nodes(node_graph: &NodeGraph) -> NodeCount {
    let mut count = 0;

    for n1 in node_graph.iter() {
        for n2 in node_graph.forward_nodes(n1) {
            for n3 in node_graph.forward_nodes(n2) {
                if n3.connects_to(&n1.id)
                    && [n1, n2, n3].iter().any(|node| node.name.starts_with("t"))
                {
                    count += 1;
                }
            }
        }
    }

    count
}

/*-----------------------------------------------------------------------------
  Identify Largest Set of Mutually Connected Nodes
-----------------------------------------------------------------------------*/

fn find_largest_network(node_graph: &NodeGraph, network: Network) -> Network {
    let current_node = node_graph.nodes.get(*network.last().unwrap()).unwrap();
    node_graph
        .forward_nodes(current_node)
        .filter(|next_node| network.iter().all(|id| next_node.connects_to(id)))
        .map(|next_node| {
            let mut new_network = network.clone();
            new_network.push(next_node.id);
            find_largest_network(node_graph, new_network)
        })
        .fold(network.clone(), |acc, network| {
            if network.len() > acc.len() {
                network
            } else {
                acc
            }
        })
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
        Args::Part1 { input } => part1(&get_input(&input)),
        Args::Part2 { input } => part2(&get_input(&input)),
    }
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_answer;

    #[test]
    fn test_example_part1() {
        assert_eq!(
            part1(&get_input("../data/day23/example.txt")),
            get_answer("../data/day23/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day23/input.txt")),
            get_answer("../data/day23/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(
            part2(&get_input("../data/day23/example.txt")),
            get_answer("../data/day23/example-part2-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day23/input.txt")),
            get_answer("../data/day23/input-part2-answer.txt")
        );
    }
}
