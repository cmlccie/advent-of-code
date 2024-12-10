/*-------------------------------------------------------------------------------------------------
  Day 9: Disk Fragmenter
-------------------------------------------------------------------------------------------------*/

use std::fs::read_to_string;
use std::io::prelude::*;
use std::path::Path;

/*--------------------------------------------------------------------------------------
  Part 1
--------------------------------------------------------------------------------------*/

pub fn part1<P: AsRef<Path> + ?Sized>(input: &P) -> i64 {
    let disk_map = parse_input_file(input);
    let mut blocks = disk_map.blocks();
    compact_file_fragments(&mut blocks);
    compute_checksum(&blocks)
}

/*--------------------------------------------------------------------------------------
  Part 2
--------------------------------------------------------------------------------------*/

pub fn part2<P: AsRef<Path> + ?Sized>(input: &P) -> i64 {
    let disk_map = parse_input_file(input);
    let mut blocks = disk_map.blocks();
    compact_whole_files(&mut blocks);

    println!("Allocations: {}\n", disk_map.allocations.len());
    // print_disk_map(&disk_map.blocks());
    print_disk_map(&blocks);

    let checksum = compute_checksum(&blocks);

    write_disk_to_file(&blocks, checksum);

    checksum
}

/*--------------------------------------------------------------------------------------
  Core
--------------------------------------------------------------------------------------*/

/*-----------------------------------------------------------------------------
  Parse Input File
-----------------------------------------------------------------------------*/

fn parse_input_file<P: AsRef<Path> + ?Sized>(input: &P) -> DiskMap {
    let file_contents = read_to_string(input).unwrap();
    println!("File Contents: {}", file_contents.len());
    println!(
        "Line Length: {}",
        file_contents.lines().next().unwrap().len()
    );

    let block_count: usize = file_contents
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| parse_digit(c) as usize)
        .sum();

    let mut allocations: Vec<Allocation> = Vec::new();
    let mut file_id: FileId = 0;
    let mut digits = file_contents
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(parse_digit);
    while let Some(file_digit) = digits.next() {
        allocations.push(Allocation::File(File::new(file_id, file_digit)));
        file_id += 1;

        if let Some(free_space_digit) = digits.next() {
            allocations.push(Allocation::FreeSpace(free_space_digit));
        }
    }

    DiskMap {
        allocations,
        block_count,
    }
}

fn parse_digit(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("Invalid character in disk map"),
    }
}

/*-----------------------------------------------------------------------------
  Disk Map
-----------------------------------------------------------------------------*/

struct DiskMap {
    allocations: Vec<Allocation>,
    block_count: usize,
}

impl DiskMap {
    fn blocks(&self) -> Vec<Block> {
        let mut blocks: Vec<Block> = Vec::with_capacity(self.block_count);
        for allocation in &self.allocations {
            match allocation {
                Allocation::File(file) => {
                    for _ in 0..file.size {
                        blocks.push(Some(file.id));
                    }
                }
                Allocation::FreeSpace(size) => {
                    for _ in 0..*size {
                        blocks.push(None);
                    }
                }
            }
        }
        blocks
    }
}

enum Allocation {
    File(File),
    FreeSpace(u8),
}

struct File {
    id: FileId,
    size: u8,
}

impl File {
    fn new(id: FileId, size: u8) -> Self {
        Self { id, size }
    }
}

type Block = Option<i64>;
type FileId = i64;

/*-----------------------------------------------------------------------------
  Compact Files Fragments
-----------------------------------------------------------------------------*/

fn compact_file_fragments(blocks: &mut [Block]) {
    let mut left_cursor: usize = 0;
    let mut right_cursor: usize = blocks.len() - 1;

    loop {
        // Find left most empty block
        while blocks[left_cursor].is_some() {
            left_cursor += 1;
        }

        // Find right most populated block
        while blocks[right_cursor].is_none() {
            right_cursor -= 1;
        }

        if left_cursor >= right_cursor {
            break;
        }

        blocks.swap(left_cursor, right_cursor);
    }
}

/*-----------------------------------------------------------------------------
  Compact Whole Files
-----------------------------------------------------------------------------*/

fn compact_whole_files(blocks: &mut [Block]) {
    let mut cursor: usize = blocks.len() - 1;
    let mut last_file_id: Option<FileId> = None;

    loop {
        let file = seek_file(blocks, &mut cursor, &mut last_file_id);

        if cursor == 0 {
            break;
        }

        if file.is_none() {
            println!("cursor: {}", cursor);
            continue;
        }

        let (file_start, file_length) = file.unwrap();

        let (left, right) = blocks.split_at_mut(file_start);

        if left.len() < file_length {
            continue;
        }

        // Search for free space
        for free_space_cursor in 0..(left.len() - file_length) {
            if left[free_space_cursor..(free_space_cursor + file_length)]
                .iter()
                .all(|block| block.is_none())
            {
                let free_slice = &mut left[free_space_cursor..(free_space_cursor + file_length)];
                let file_slice = &mut right[..file_length];
                free_slice.swap_with_slice(file_slice);
                break;
            }
        }
    }
}

type FileStart = usize;
type FileLength = usize;

fn seek_file(
    blocks: &[Block],
    cursor: &mut usize,
    last_file_id: &mut Option<FileId>,
) -> Option<(FileStart, FileLength)> {
    while blocks[*cursor].is_none() && *cursor > 0 {
        *cursor -= 1;
    }
    let file_end = *cursor;
    let file_id = blocks[file_end];

    while (blocks[*cursor] == file_id) && *cursor > 0 {
        *cursor -= 1;
    }

    if last_file_id.is_some() && file_id.unwrap() >= last_file_id.unwrap() {
        return None;
    }

    *last_file_id = file_id;

    println!(
        "file_id: {} last_file_id: {}",
        file_id.unwrap(),
        last_file_id.unwrap()
    );

    let file_start = *cursor + 1;
    let file_length = file_end - file_start + 1;
    Some((file_start, file_length))
}

/*-----------------------------------------------------------------------------
  Print Disk Map
-----------------------------------------------------------------------------*/

fn print_disk_map(blocks: &[Block]) {
    for block in blocks {
        if let Some(block) = block {
            print!("[{}]", block);
        } else {
            print!(".");
        }
    }
}

/*-----------------------------------------------------------------------------
  Write Blocks to File
-----------------------------------------------------------------------------*/

fn write_disk_to_file(blocks: &[Block], checksum: i64) {
    let mut file = std::fs::File::create("disk.txt").unwrap();
    for block in blocks {
        if let Some(block) = block {
            file.write_all(format!("{}\n", block).as_bytes()).unwrap();
        } else {
            file.write_all(".\n".as_bytes()).unwrap();
        }
    }
    file.write_all(format!("Checksum: {}", checksum).as_bytes())
        .unwrap();
}

/*-----------------------------------------------------------------------------
  Compute Checksum
-----------------------------------------------------------------------------*/

fn compute_checksum(blocks: &[Block]) -> i64 {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, block)| block.is_some())
        .map(|(i, file_id)| i as i64 * file_id.unwrap())
        .sum()
}

/*-------------------------------------------------------------------------------------------------
  Tests
-------------------------------------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::solution;

    #[test]
    fn test_example_solution_part1() {
        assert_eq!(
            part1("../data/day9/example.txt"),
            solution("../data/day9/example-part1-answer.txt").unwrap()
        );
    }

    #[test]
    fn test_example_solution_part2() {
        assert_eq!(
            part2("../data/day9/example.txt"),
            solution("../data/day9/example-part2-answer.txt").unwrap()
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1("../data/day9/input.txt"),
            solution("../data/day9/input-part1-answer.txt").unwrap()
        );
    }

    // #[test]
    // fn test_part2_solution() {
    //     assert_eq!(
    //         part2("../data/day9/input.txt"),
    //         solution("../data/day9/input-part2-answer.txt").unwrap()
    //     );
    // }
}
