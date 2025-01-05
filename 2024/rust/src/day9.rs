use crate::shared::inputs::get_input;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

/*-------------------------------------------------------------------------------------------------
  Day 9: Disk Fragmenter
-------------------------------------------------------------------------------------------------*/

pub fn part1(input: &str) -> Option<String> {
    let mut disk = parse_input(input);
    disk.compact_blocks();

    Some(disk.checksum().to_string())
}

pub fn part2(input: &str) -> Option<String> {
    let mut disk = parse_input(input);
    disk.compact_files();

    Some(disk.checksum().to_string())
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

fn parse_input(input: &str) -> Disk {
    let allocation_string = input.lines().next().unwrap();
    Disk::new(allocation_string)
}

/*-----------------------------------------------------------------------------
  Disk
-----------------------------------------------------------------------------*/

type FileId = u64;
type BlockCount = u32;
type BlockIndex = u64;
type DiskCursor = BlockIndex;

enum Allocation {
    File(BlockCount),
    FreeSpace(BlockCount),
}

struct Disk {
    files: BTreeMap<FileId, File>,
    blocks: BTreeMap<DiskCursor, FileId>,

    first_free_block_cache: DiskCursor,
}

impl Disk {
    fn new(dense_format: &str) -> Self {
        let allocations: Vec<Allocation> = dense_format
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .map(|(index, digit)| {
                if index % 2 == 0 {
                    Allocation::File(digit)
                } else {
                    Allocation::FreeSpace(digit)
                }
            })
            .collect();

        let mut files: BTreeMap<FileId, File> = BTreeMap::new();
        let mut blocks: BTreeMap<DiskCursor, FileId> = BTreeMap::new();
        allocate_files(&allocations, &mut files, &mut blocks);

        Self {
            files,
            blocks,

            first_free_block_cache: 0,
        }
    }

    fn compact_blocks(&mut self) {
        loop {
            let last_block_index = *self.blocks.last_key_value().unwrap().0;
            let first_free_index = self.get_first_free_block_index();

            // Exit if there are no more free blocks to the left of the last file block
            if last_block_index < first_free_index {
                break;
            }

            // Move the last file block to the first free block
            self.move_file_block(last_block_index, first_free_index);
        }
    }

    fn compact_files(&mut self) {
        let last_file_id = *self.files.last_key_value().unwrap().0;

        for file_id in (0..=last_file_id).rev() {
            let free_index = {
                let file = self.files.get(&file_id).unwrap();
                self.find_free_range(file.length, file.lowest_index())
            };

            if let Some(free_index) = free_index {
                self.move_file(file_id, free_index);
                log::debug!("Moved file {} to index {}", file_id, free_index);
            } else {
                log::debug!(
                    "Could not move file {}; no free range to left of file",
                    file_id
                );
            }
        }
    }

    fn get_first_free_block_index(&mut self) -> BlockIndex {
        while self.blocks.contains_key(&self.first_free_block_cache) {
            self.first_free_block_cache += 1;
        }
        self.first_free_block_cache
    }

    fn find_free_range(
        &mut self,
        length: BlockCount,
        stop_index: BlockIndex,
    ) -> Option<BlockIndex> {
        let first_free_block = self.get_first_free_block_index();

        for start_index in first_free_block..stop_index {
            let range_is_free = (start_index..(start_index + length as BlockIndex))
                .all(|index| !self.blocks.contains_key(&index));

            if range_is_free {
                return Some(start_index);
            }
        }

        None
    }

    fn move_file_block(&mut self, from: BlockIndex, to: BlockIndex) {
        assert!(!self.blocks.contains_key(&to)); // Verify destination block is free
        let file_id = self.blocks.remove(&from).expect("File block exists");

        // Update the file
        self.files
            .entry(file_id)
            .and_modify(|file| file.move_block(from, to));

        self.blocks.insert(to, file_id);
    }

    fn move_file(&mut self, file_id: FileId, to: BlockIndex) {
        let file = self.files.get_mut(&file_id).expect("File exists");
        let new_blocks = file
            .blocks
            .iter()
            .zip(to..(to + file.length as BlockIndex))
            .map(|(from, to)| {
                assert!(!self.blocks.contains_key(&to)); // Destination must be free
                let file_id = self.blocks.remove(from).expect("File block exists");
                assert_eq!(file_id, file.id); // Source block must be from the file
                self.blocks.insert(to, file_id);
                to // Return the new block index
            })
            .collect();

        file.blocks = new_blocks;
    }

    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .map(|(cursor, file_id)| *cursor * *file_id)
            .sum()
    }
}

/*-----------------------------------------------------------------------------
  File
-----------------------------------------------------------------------------*/

struct File {
    id: FileId,
    length: BlockCount,
    blocks: BTreeSet<BlockIndex>,
}

impl File {
    fn new(id: FileId, length: BlockCount, cursor: BlockIndex) -> Self {
        let blocks: BTreeSet<BlockIndex> = (cursor..(cursor + length as BlockIndex)).collect();
        Self { id, length, blocks }
    }

    fn lowest_index(&self) -> BlockIndex {
        *self.blocks.first().unwrap()
    }

    fn move_block(&mut self, from: BlockIndex, to: BlockIndex) {
        assert!(self.blocks.remove(&from)); // Source block should exist
        self.blocks.insert(to);
    }
}

/*-----------------------------------------------------------------------------
  Allocate Files
-----------------------------------------------------------------------------*/

fn allocate_files(
    allocations: &[Allocation],
    files: &mut BTreeMap<FileId, File>,
    blocks: &mut BTreeMap<DiskCursor, FileId>,
) {
    let mut file_id: FileId = 0;
    let mut cursor: DiskCursor = 0;

    for allocation in allocations.iter() {
        match allocation {
            Allocation::File(length) => {
                files.insert(file_id, File::new(file_id, *length, cursor));
                let file = files.get(&file_id).unwrap();
                for block in file.blocks.iter() {
                    blocks.insert(*block, file.id);
                }

                file_id += 1;
                cursor += *length as BlockIndex;
            }
            Allocation::FreeSpace(length) => {
                cursor += *length as BlockIndex;
            }
        }
    }
}

/*-------------------------------------------------------------------------------------------------
  CLI
-------------------------------------------------------------------------------------------------*/

#[derive(clap::Subcommand)]
#[command(long_about = "Day 9: Disk Fragmenter")]
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
    fn test_example_solution_part1() {
        assert_eq!(
            part1(&get_input("../data/day9/example.txt")),
            get_answer("../data/day9/example-part1-answer.txt")
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2(&get_input("../data/day9/example.txt")),
            get_answer("../data/day9/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(&get_input("../data/day9/input.txt")),
            get_answer("../data/day9/input-part1-answer.txt")
        );
    }

    #[test]
    #[cfg_attr(not(feature = "slow_tests"), ignore)]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day9/input.txt")),
            get_answer("../data/day9/input-part2-answer.txt")
        );
    }
}
