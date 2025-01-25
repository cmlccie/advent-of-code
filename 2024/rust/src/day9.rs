use crate::get_input;
use std::collections::VecDeque;
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

type FileId = u16;
type BlockCount = u8;
type BlockIndex = usize;
type CheckSum = u64;

#[derive(Debug, Clone, Copy)]
struct FileAllocation {
    id: FileId,
    index: BlockIndex,
    length: BlockCount,
}

#[derive(Debug, Clone, Copy)]
struct FreeSpaceAllocation {
    index: BlockIndex,
    length: BlockCount,
}

struct Disk {
    files: Vec<FileAllocation>,
    free_space: FreeSpaceAllocator,
    blocks: Vec<Option<FileId>>,
}

impl Disk {
    fn new(dense_format: &str) -> Self {
        let allocation_count = dense_format.len() / 2;

        let mut files = Vec::with_capacity(allocation_count);
        let mut free_space = FreeSpaceAllocator::with_capacity(allocation_count);

        let mut next_file_id: FileId = 0;
        let mut next_block_index: usize = 0;

        for (dense_index, allocation_length) in dense_format.chars().enumerate() {
            let allocation_length = allocation_length.to_digit(10).unwrap() as BlockCount;

            if dense_index % 2 == 0 {
                files.push(FileAllocation {
                    id: next_file_id,
                    index: next_block_index,
                    length: allocation_length,
                });
                next_file_id += 1;
            } else {
                free_space.push_back(FreeSpaceAllocation {
                    index: next_block_index,
                    length: allocation_length,
                });
            }

            next_block_index += allocation_length as usize;
        }

        let blocks = vec![None; next_block_index];

        Self {
            files,
            free_space,
            blocks,
        }
    }

    /*-------------------------------------------------------------------------
      Part 1 Methods
    -------------------------------------------------------------------------*/

    fn compact_blocks(&mut self) {
        self.allocate_files();

        let mut first_free_block_index = self.next_free_block_index(0);
        let mut last_file_block_index = self.next_file_block_index(self.blocks.len());

        while first_free_block_index < last_file_block_index {
            self.blocks
                .swap(first_free_block_index, last_file_block_index);

            first_free_block_index = self.next_free_block_index(first_free_block_index);
            last_file_block_index = self.next_file_block_index(last_file_block_index);
        }
    }

    fn next_free_block_index(&mut self, from: usize) -> BlockIndex {
        self.blocks[from..]
            .iter()
            .position(|block| block.is_none())
            .unwrap()
            + from
    }

    fn next_file_block_index(&mut self, from: usize) -> BlockIndex {
        self.blocks[..from]
            .iter()
            .rposition(|block| block.is_some())
            .unwrap()
    }

    fn allocate_files(&mut self) {
        while let Some(file) = self.files.pop() {
            self.allocate_file(file);
        }
    }

    fn allocate_file(&mut self, file: FileAllocation) {
        for block_index in file.index..(file.index + file.length as usize) {
            self.blocks[block_index] = Some(file.id);
        }
    }

    /*-------------------------------------------------------------------------
      Part 2 Methods
    -------------------------------------------------------------------------*/

    fn compact_files(&mut self) {
        while let Some(mut file) = self.files.pop() {
            if file.index < self.free_space.first_free_block_index() {
                // Stop checking for free space
                self.allocate_file(file);
                break;
            }

            // Find a free space allocation that can fit the file
            let free_space = self.free_space.get(file);

            if let Some(mut free_space) = free_space {
                // Allocate the file
                file.index = free_space.index;
                self.allocate_file(file);

                // Update the free space allocation
                if free_space.length != file.length {
                    free_space.index += file.length as usize;
                    free_space.length -= file.length;
                    self.free_space.insert(free_space);
                }
            } else {
                // Could not find a free space allocation that can fit the file
                self.allocate_file(file);
            }
        }

        // Allocate the remaining files
        self.allocate_files();
    }

    /*-------------------------------------------------------------------------
      Disk Checksum
    -------------------------------------------------------------------------*/

    fn checksum(&self) -> CheckSum {
        self.blocks
            .iter()
            .enumerate()
            .fold(0, |acc, (index, block)| {
                if let Some(file_id) = block {
                    let index = index as CheckSum;
                    let file_id = *file_id as CheckSum;
                    acc + (index * file_id)
                } else {
                    acc
                }
            })
    }
}

/*-----------------------------------------------------------------------------
  Free Space Allocator
-----------------------------------------------------------------------------*/

struct FreeSpaceAllocator {
    queues: [VecDeque<FreeSpaceAllocation>; 10],
}

impl FreeSpaceAllocator {
    fn with_capacity(allocations: usize) -> Self {
        let queue_size = allocations / 8;
        Self {
            queues: [
                VecDeque::with_capacity(0),
                VecDeque::with_capacity(queue_size),
                VecDeque::with_capacity(queue_size),
                VecDeque::with_capacity(queue_size),
                VecDeque::with_capacity(queue_size),
                VecDeque::with_capacity(queue_size),
                VecDeque::with_capacity(queue_size),
                VecDeque::with_capacity(queue_size),
                VecDeque::with_capacity(queue_size),
                VecDeque::with_capacity(queue_size),
            ],
        }
    }

    fn push_back(&mut self, allocation: FreeSpaceAllocation) {
        if allocation.length == 0 {
            return;
        }

        self.queues[allocation.length as usize].push_back(allocation);
    }

    fn insert(&mut self, allocation: FreeSpaceAllocation) {
        if allocation.length == 0 {
            return;
        }

        let index = self.queues[allocation.length as usize]
            .iter()
            .position(|existing_allocation| existing_allocation.index > allocation.index);

        if let Some(index) = index {
            self.queues[allocation.length as usize].insert(index, allocation);
        } else {
            self.queues[allocation.length as usize].push_back(allocation);
        }
    }

    fn first_free_block_index(&self) -> BlockIndex {
        self.queues
            .iter()
            .filter_map(|queue| queue.front())
            .map(|allocation| allocation.index)
            .min()
            .unwrap()
    }

    fn get(&mut self, file: FileAllocation) -> Option<FreeSpaceAllocation> {
        let selected_allocation = self.queues[file.length as usize..]
            .iter()
            .map(|queue| queue.front())
            .filter(|allocation| allocation.is_some() && allocation.unwrap().index < file.index)
            .fold(None, |acc, allocation| match (acc, allocation) {
                (_, None) => acc,
                (None, Some(_)) => allocation,
                (Some(acc), Some(allocation)) => {
                    if acc.index < allocation.index {
                        Some(acc)
                    } else {
                        Some(allocation)
                    }
                }
            })?;

        self.queues[selected_allocation.length as usize].pop_front()
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
    use crate::get_answer;

    #[test]
    fn test_example_solution_part1() {
        assert_eq!(
            part1(&get_input("../data/day9/example.txt")),
            get_answer("../data/day9/example-part1-answer.txt")
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
    fn test_example_solution_part2() {
        assert_eq!(
            part2(&get_input("../data/day9/example.txt")),
            get_answer("../data/day9/example-part2-answer.txt")
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(
            part2(&get_input("../data/day9/input.txt")),
            get_answer("../data/day9/input-part2-answer.txt")
        );
    }
}
