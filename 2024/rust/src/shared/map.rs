#![allow(dead_code)]
use itertools::Itertools;

/*-------------------------------------------------------------------------------------------------
  Map
-------------------------------------------------------------------------------------------------*/

pub type MapIndex = (usize, usize);
pub type Coordinate = (isize, isize);
pub type Offset = (isize, isize);

pub struct Map<T> {
    map: Vec<Vec<T>>,
    rows: usize,
    columns: usize,
}

impl<T> Map<T> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn len(&self) -> usize {
        self.rows * self.columns
    }

    pub fn iter(&self) -> MapIterator<T> {
        MapIterator::new(self)
    }

    /*-------------------------------------------------------------------------
      Methods for working with Indices
    -------------------------------------------------------------------------*/

    pub fn get(&self, index: MapIndex) -> Option<&T> {
        let (row, column) = index;
        if row < self.rows && column < self.columns {
            Some(&self.map[row][column])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: MapIndex) -> Option<&mut T> {
        let (row, column) = index;
        if row < self.rows && column < self.columns {
            Some(&mut self.map[row][column])
        } else {
            None
        }
    }

    pub fn indices(&self) -> impl Iterator<Item = MapIndex> {
        (0..self.rows).cartesian_product(0..self.columns)
    }

    pub fn index_to_coordinate(&self, index: MapIndex) -> Coordinate {
        let (row, column) = index;
        (
            isize::try_from(row).unwrap(),
            isize::try_from(column).unwrap(),
        )
    }

    pub fn project_index_offset(&self, index: MapIndex, offset: Offset) -> Option<MapIndex> {
        let (row, column) = index;
        let (row_offset, column_offset) = offset;
        let coordinate = (row as isize + row_offset, column as isize + column_offset);
        self.coordinate_to_index(coordinate)
    }

    pub fn project_index_offset_to_coordinate(
        &self,
        index: MapIndex,
        offset: Offset,
    ) -> Coordinate {
        let (row, column) = index;
        let (row, column) = (
            isize::try_from(row).unwrap(),
            isize::try_from(column).unwrap(),
        );
        let (row_offset, column_offset) = offset;
        (row + row_offset, column + column_offset)
    }

    pub fn check_index_bounds(&self, index: MapIndex) -> bool {
        let (row, column) = index;
        row < self.rows && column < self.columns
    }

    /*-------------------------------------------------------------------------
      Methods for working with Coordinates
    -------------------------------------------------------------------------*/

    pub fn get_coordinate(&self, coordinate: Coordinate) -> Option<&T> {
        let index = self.coordinate_to_index(coordinate)?;
        self.get(index)
    }

    pub fn get_coordinate_mut(&mut self, coordinate: Coordinate) -> Option<&mut T> {
        let index = self.coordinate_to_index(coordinate)?;
        self.get_mut(index)
    }

    pub fn coordinates(&self) -> impl Iterator<Item = Coordinate> {
        let (rows, columns) = (
            isize::try_from(self.rows).unwrap(),
            isize::try_from(self.columns).unwrap(),
        );
        (0..rows).cartesian_product(0..columns)
    }

    pub fn coordinate_to_index(&self, coordinate: Coordinate) -> Option<MapIndex> {
        if self.check_coordinate_bounds(coordinate) {
            let (row, column) = coordinate;
            Some((row as usize, column as usize))
        } else {
            None
        }
    }

    pub fn project_coordinate_offset(&self, coordinate: Coordinate, offset: Offset) -> Coordinate {
        let (row, column) = coordinate;
        let (row_offset, column_offset) = offset;
        (row + row_offset, column + column_offset)
    }

    pub fn project_coordinate_offset_to_index(
        &self,
        coordinate: Coordinate,
        offset: Offset,
    ) -> Option<MapIndex> {
        let new_coordinate = self.project_coordinate_offset(coordinate, offset);
        self.coordinate_to_index(new_coordinate)
    }

    pub fn check_coordinate_bounds(&self, coordinate: Coordinate) -> bool {
        let (row, column) = coordinate;
        let (row, column) = (usize::try_from(row), usize::try_from(column));
        if row.is_err() || column.is_err() {
            return false;
        }
        let index = (row.unwrap(), column.unwrap());
        self.check_index_bounds(index)
    }
}

/*--------------------------------------------------------------------------------------
  Trait Implementations
--------------------------------------------------------------------------------------*/

impl<T, I> FromIterator<I> for Map<T>
where
    I: IntoIterator<Item = T>,
{
    fn from_iter<O>(iter: O) -> Self
    where
        O: IntoIterator,
        O::Item: IntoIterator<Item = T>,
    {
        let map: Vec<Vec<T>> = iter
            .into_iter()
            .map(|columns| columns.into_iter().collect())
            .collect();

        let rows = map.len();
        let columns = if rows > 0 { map[0].len() } else { 0 };

        // Verify all rows have the same number of columns
        if !map.iter().all(|row| row.len() == columns) {
            panic!("All rows must have the same number of columns");
        }

        Self { map, rows, columns }
    }
}

impl From<&str> for Map<char> {
    fn from(s: &str) -> Map<char> {
        s.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect()
    }
}

impl From<&str> for Map<u8> {
    fn from(s: &str) -> Map<u8> {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

/*--------------------------------------------------------------------------------------
  Map Iterator
--------------------------------------------------------------------------------------*/

pub struct MapIterator<'m, T> {
    map: &'m Map<T>,
    row: usize,
    column: usize,
}

impl<'m, T> MapIterator<'m, T> {
    pub fn new(map: &'m Map<T>) -> Self {
        Self {
            map,
            row: 0,
            column: 0,
        }
    }
}

impl<'m, T> Iterator for MapIterator<'m, T> {
    type Item = &'m T;

    fn next(&mut self) -> Option<&'m T> {
        let next_value = self.map.get((self.row, self.column))?;

        let next_column = self.column + 1;
        (self.row, self.column) = if next_column < self.map.columns {
            (self.row, next_column)
        } else {
            (self.row + 1, 0)
        };

        Some(next_value)
    }
}
