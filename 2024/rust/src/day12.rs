use crate::shared::inputs::get_input;
use crate::shared::map::{Coordinate, Map, MapIndex, Offset};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 12: Garden Groups
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let map = parse_input(input);
    let mut regions = Regions::new(&map);
    regions.map_regions();

    let cost = calculate_fencing_cost_part1(&regions);

    Some(cost.to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let map = parse_input(input);
    let mut regions = Regions::new(&map);
    regions.map_regions();

    let cost = calculate_fencing_cost_part2(&regions);

    Some(cost.to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

type Plant = char;
type RegionID = u64;
type Region = HashSet<MapIndex>;

type Perimeter = u64;
type Area = u64;
type FenceCost = u64;

type SideCount = u64;
type Fence = (Coordinate, Coordinate);

const NEIGHBOR_OFFSETS: [Offset; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn parse_input(input: &str) -> Map<char> {
    input.into()
}

fn calculate_fencing_cost_part1(regions: &Regions) -> FenceCost {
    regions
        .regions
        .values()
        .map(|region| {
            let area = calculate_region_area(region);
            let perimeter = calculate_region_perimeter(regions.map, region);
            area * perimeter
        })
        .sum()
}

fn calculate_fencing_cost_part2(regions: &Regions) -> FenceCost {
    regions
        .regions
        .values()
        .map(|region| {
            let area = calculate_region_area(region);
            let sides = calculate_region_sides(regions.map, region);
            area * sides
        })
        .sum()
}

fn calculate_region_area(region: &Region) -> Area {
    region.len() as u64
}

fn calculate_region_perimeter(map: &Map<char>, region: &Region) -> Perimeter {
    region
        .iter()
        .map(|plot| {
            let region_neighbors = NEIGHBOR_OFFSETS
                .iter()
                .flat_map(move |offset| map.project_index_offset(*plot, *offset))
                .filter(|neighbor| region.contains(neighbor))
                .count();
            4 - region_neighbors
        })
        .sum::<usize>() as u64
}

fn calculate_region_sides(map: &Map<char>, region: &Region) -> SideCount {
    let boundary_plots: HashSet<(usize, usize)> = region
        .iter()
        .filter(|index| {
            NEIGHBOR_OFFSETS
                .iter()
                .filter_map(|offset| map.project_index_offset(**index, *offset)) // Neighboring plots
                .filter(|index| region.contains(index)) // In-region neighbors
                .count()
                != 4 // All 4 neighbors are NOT in the region (boundary plot)
        })
        .copied()
        .collect();

    let fences: Vec<Fence> = boundary_plots
        .iter()
        .map(|index| map.index_to_coordinate(*index))
        .flat_map(|plot| {
            let inside_neighbor = plot;
            NEIGHBOR_OFFSETS
                .iter()
                .map(move |offset| map.project_coordinate_offset(plot, *offset)) // Neighbors (including off-map)
                .filter(|neighbor| {
                    map.coordinate_to_index(*neighbor)
                        .and_then(|neighbor| region.contains(&neighbor).then_some(neighbor))
                        .is_none()
                }) // Out-of-region neighbors
                .map(move |outside_neighbor| (inside_neighbor, outside_neighbor))
        })
        .collect();

    let mut fence_adjacency_map: HashMap<Fence, HashSet<Fence>> = HashMap::new();
    // Add adjacent fences to the adjacency map
    for pair in fences.iter().combinations(2) {
        let fence0 = *pair[0];
        let fence1 = *pair[1];
        if fences_are_adjacent(fence0, fence1) {
            fence_adjacency_map
                .entry(fence0)
                .or_default()
                .insert(fence1);
            fence_adjacency_map
                .entry(fence1)
                .or_default()
                .insert(fence0);
        };
    }
    // Add fences with no adjacent fences to the adjacency map
    for fence in fences {
        fence_adjacency_map.entry(fence).or_default();
    }

    identify_and_count_sides(&mut fence_adjacency_map)
}

fn identify_and_count_sides(fence_adjacency_map: &mut HashMap<Fence, HashSet<Fence>>) -> SideCount {
    let mut sides: Vec<HashSet<Fence>> = Vec::new();

    while !fence_adjacency_map.is_empty() {
        let mut side: HashSet<Fence> = HashSet::new();
        let mut stack: Vec<Fence> = Vec::new();

        let fence = fence_adjacency_map.keys().next().unwrap();
        stack.push(*fence);

        while let Some(fence) = stack.pop() {
            side.insert(fence);
            let adjacent_fences = fence_adjacency_map.remove(&fence).unwrap();
            for adjacent_fence in adjacent_fences {
                if side.insert(adjacent_fence) {
                    stack.push(adjacent_fence);
                }
            }
        }

        sides.push(side);
    }

    sides.len() as u64
}

fn fences_are_adjacent(fence0: Fence, fence1: Fence) -> bool {
    let (inside_plot0, outside_plot0) = fence0;
    let (inside_plot1, outside_plot1) = fence1;

    let inside_shift = (
        inside_plot0.0 - inside_plot1.0,
        inside_plot0.1 - inside_plot1.1,
    );

    let outside_shift = (
        outside_plot0.0 - outside_plot1.0,
        outside_plot0.1 - outside_plot1.1,
    );

    let magnitude = inside_shift.0.abs() + inside_shift.1.abs();

    magnitude == 1 && inside_shift.0 == outside_shift.0 && inside_shift.1 == outside_shift.1
}

/*-----------------------------------------------------------------------------
  Regions
-----------------------------------------------------------------------------*/

struct Regions<'m> {
    map: &'m Map<char>,

    regions: HashMap<RegionID, Region>,
    plots: HashMap<MapIndex, RegionID>,

    next_region_id: RegionID,
}

impl<'m> Regions<'m> {
    fn new(map: &'m Map<char>) -> Self {
        Self {
            map,
            regions: HashMap::new(),
            plots: HashMap::new(),
            next_region_id: 0,
        }
    }

    fn map_regions(&mut self) {
        for index in self.map.indices() {
            if self.plots.contains_key(&index) {
                continue;
            }

            self.create_region(index);
        }
    }

    fn create_region(&mut self, start: MapIndex) {
        let region_id = self.next_region_id;
        self.next_region_id += 1;

        let mut region: Region = HashSet::new();
        self.plot_region(region_id, &mut region, *self.map.get(start).unwrap(), start);
        self.regions.insert(region_id, region);
    }

    fn plot_region(
        &mut self,
        region_id: RegionID,
        region: &mut Region,
        plant: Plant,
        index: MapIndex,
    ) {
        // Skip if this plot has already been assigned to the region
        if region.contains(&index) {
            return;
        }

        // Add this plot to the region
        region.insert(index);
        self.plots.insert(index, region_id);

        // Check the surrounding plots
        NEIGHBOR_OFFSETS
            .iter()
            .filter_map(|offset| self.map.project_index_offset(index, *offset))
            .filter(|index| self.map.get(*index) == Some(&plant))
            .for_each(|index| self.plot_region(region_id, region, plant, index));
    }
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 12: Garden Groups")]
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
    use crate::shared::answers::get_answer;

    #[test]
    fn test_example2_part1() {
        assert_eq!(
            part1(&get_input("../data/day12/example2.txt")),
            get_answer("../data/day12/example2-part1-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day12/input.txt")),
            get_answer("../data/day12/input-part1-answer.txt")
        );
    }

    #[test]
    fn test_example2_part2() {
        assert_eq!(
            part2(&get_input("../data/day12/example2.txt")),
            get_answer("../data/day12/example2-part2-answer.txt")
        );
    }

    #[test]
    fn test_example3_part2() {
        assert_eq!(
            part2(&get_input("../data/day12/example3.txt")),
            get_answer("../data/day12/example3-part2-answer.txt")
        );
    }

    #[test]
    fn test_example4_part2() {
        assert_eq!(
            part2(&get_input("../data/day12/example4.txt")),
            get_answer("../data/day12/example4-part2-answer.txt")
        );
    }

    #[test]
    fn test_example5_part2() {
        assert_eq!(
            part2(&get_input("../data/day12/example5.txt")),
            get_answer("../data/day12/example5-part2-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day12/input.txt")),
            get_answer("../data/day12/input-part2-answer.txt")
        );
    }
}
