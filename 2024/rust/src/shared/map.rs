#![allow(dead_code)]
use crate::shared::direction::AnyDirection;
use crate::shared::grid_index::GridIndex;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use num::{Integer, Signed};
use std::cell::OnceCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/*-------------------------------------------------------------------------------------------------
  Map
-------------------------------------------------------------------------------------------------*/

#[derive(Debug)]
pub struct Map<I, T>
where
    I: Integer + Copy + TryInto<usize> + TryFrom<usize>,
    <I as TryInto<usize>>::Error: std::fmt::Debug,
    <I as TryFrom<usize>>::Error: std::fmt::Debug,
{
    data: Vec<T>,
    bounds: GridIndex<I>,
}

impl<I, T> Map<I, T>
where
    I: Integer + Copy + TryInto<usize> + TryFrom<usize>,
    <I as TryInto<usize>>::Error: std::fmt::Debug,
    <I as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub fn new(rows: I, columns: I, default: T) -> Self
    where
        T: Clone,
    {
        let data = vec![default; rows.try_into().unwrap() * columns.try_into().unwrap()];
        let bounds = GridIndex::new(rows, columns);
        Self { data, bounds }
    }

    pub fn rows(&self) -> I {
        self.bounds.row
    }

    pub fn columns(&self) -> I {
        self.bounds.column
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (GridIndex<I>, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(move |(internal_index, item)| {
                let grid_index = self.grid_index(internal_index).unwrap();
                (grid_index, item)
            })
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self.bounds.column.try_into().unwrap())
    }

    pub fn find<F>(&self, predicate: F) -> Option<GridIndex<I>>
    where
        F: FnMut(&T) -> bool,
    {
        let internal_index = self.data.iter().position(predicate)?;
        self.grid_index(internal_index)
    }

    pub fn check_is_in_bounds(&self, index: GridIndex<I>) -> bool {
        (I::zero()..self.bounds.row).contains(&index.row)
            && (I::zero()..self.bounds.column).contains(&index.column)
    }

    pub fn is_in_bounds(&self, index: GridIndex<I>) -> Option<GridIndex<I>> {
        self.check_is_in_bounds(index).then_some(index)
    }

    pub fn get(&self, index: GridIndex<I>) -> Option<&T> {
        let internal_index = self.internal_index(index)?;
        Some(&self.data[internal_index])
    }

    pub fn get_mut(&mut self, index: GridIndex<I>) -> Option<&mut T> {
        let internal_index = self.internal_index(index)?;
        Some(&mut self.data[internal_index])
    }

    pub fn set(&mut self, index: GridIndex<I>, value: T) -> Result<()> {
        let internal_index = self
            .internal_index(index)
            .ok_or(anyhow!("Index out of bounds"))?;
        self.data[internal_index] = value;
        Ok(())
    }

    pub fn project_offset(
        &self,
        index: GridIndex<I>,
        offset: GridIndex<I>,
    ) -> Option<GridIndex<I>> {
        let new_index = index + offset;
        self.check_is_in_bounds(new_index).then_some(new_index)
    }

    /*-------------------------------------------------------------------------
      Private Methods for working with internal index
    -------------------------------------------------------------------------*/

    fn internal_index(&self, index: GridIndex<I>) -> Option<usize> {
        self.check_is_in_bounds(index).then(|| {
            index.row.try_into().unwrap() * self.bounds.column.try_into().unwrap()
                + index.column.try_into().unwrap()
        })
    }

    fn grid_index(&self, internal_index: usize) -> Option<GridIndex<I>> {
        (0..self.data.len()).contains(&internal_index).then(|| {
            let row: I = (internal_index / self.bounds.column.try_into().unwrap())
                .try_into()
                .unwrap();
            let column: I = (internal_index % self.bounds.column.try_into().unwrap())
                .try_into()
                .unwrap();
            GridIndex::new(row, column)
        })
    }
}

impl<I, T> Map<I, T>
where
    I: Integer + Signed + Copy + TryInto<usize> + TryFrom<usize>,
    <I as TryInto<usize>>::Error: std::fmt::Debug,
    <I as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub fn project_direction<D: AnyDirection<I>>(
        &self,
        index: GridIndex<I>,
        direction: D,
    ) -> Option<GridIndex<I>> {
        let offset = direction.offset();
        self.project_offset(index, offset)
    }
}

/*-----------------------------------------------------------------------------
  Display Methods
-----------------------------------------------------------------------------*/

impl<I, T> Display for Map<I, T>
where
    T: Copy + Into<char>,
    I: Integer + Copy + TryInto<usize> + TryFrom<usize>,
    <I as TryInto<usize>>::Error: std::fmt::Debug,
    <I as TryFrom<usize>>::Error: std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.data.chunks(self.bounds.column.try_into().unwrap()) {
            for c in row {
                write!(f, "{}", (*c).into())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<I> Map<I, char>
where
    I: Integer + Copy + TryInto<usize> + TryFrom<usize>,
    <I as TryInto<usize>>::Error: std::fmt::Debug,
    <I as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub fn display_with_actor(&self, actor: char, actor_index: GridIndex<I>) -> String {
        self.data
            .chunks(self.bounds.column.try_into().unwrap())
            .enumerate()
            .map(|(row, columns)| {
                columns
                    .iter()
                    .enumerate()
                    .map(|(column, &value)| {
                        if GridIndex::new(row.try_into().unwrap(), column.try_into().unwrap())
                            == actor_index
                        {
                            actor
                        } else {
                            value
                        }
                    })
                    .collect::<String>()
            })
            .join("\n")
    }
}

impl<I> Map<I, char>
where
    I: Integer + Copy + TryInto<usize> + TryFrom<usize> + std::hash::Hash + Eq,
    <I as TryInto<usize>>::Error: std::fmt::Debug,
    <I as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub fn display_with_overlay(&self, overlay: &HashMap<GridIndex<I>, char>) -> String {
        self.data
            .chunks(self.bounds.column.try_into().unwrap())
            .enumerate()
            .map(|(row, columns)| {
                columns
                    .iter()
                    .enumerate()
                    .map(|(column, &value)| {
                        let index =
                            GridIndex::new(row.try_into().unwrap(), column.try_into().unwrap());
                        overlay.get(&index).copied().unwrap_or(value)
                    })
                    .collect::<String>()
            })
            .join("\n")
    }
}

/*--------------------------------------------------------------------------------------
  Conversion Trait Implementations
--------------------------------------------------------------------------------------*/

impl<R, I, T> FromIterator<R> for Map<I, T>
where
    R: IntoIterator<Item = T> + ExactSizeIterator,
    I: Integer + Copy + TryInto<usize> + TryFrom<usize>,
    <I as TryInto<usize>>::Error: std::fmt::Debug,
    <I as TryFrom<usize>>::Error: std::fmt::Debug,
{
    fn from_iter<O>(iter: O) -> Self
    where
        O: IntoIterator,
        O::Item: IntoIterator<Item = T> + ExactSizeIterator,
    {
        let column_count: OnceCell<usize> = OnceCell::new();
        let mut row_count: usize = 0;

        let data: Vec<T> = iter
            .into_iter()
            .flat_map(|row| {
                column_count.get_or_init(|| row.len());
                row_count += 1;
                row.into_iter()
            })
            .collect();

        let bounds: GridIndex<I> = GridIndex::new(
            row_count.try_into().unwrap(),
            (*OnceCell::get(&column_count).unwrap()).try_into().unwrap(),
        );

        if data.len() != (bounds.row.try_into().unwrap() * bounds.column.try_into().unwrap()) {
            panic!("All rows must have the same number of columns");
        };

        Self { data, bounds }
    }
}

impl<I> From<&str> for Map<I, char>
where
    I: Integer + Copy + TryInto<usize> + TryFrom<usize>,
    <I as TryInto<usize>>::Error: std::fmt::Debug,
    <I as TryFrom<usize>>::Error: std::fmt::Debug,
{
    fn from(s: &str) -> Self {
        let data: Vec<char> = s.lines().flat_map(|line| line.chars()).collect();
        let bounds: GridIndex<I> = GridIndex::new(
            s.lines().count().try_into().unwrap(),
            s.lines()
                .next()
                .unwrap()
                .chars()
                .count()
                .try_into()
                .unwrap(),
        );

        if data.len() != (bounds.row.try_into().unwrap() * bounds.column.try_into().unwrap()) {
            panic!("All rows must have the same number of columns");
        };

        Self { data, bounds }
    }
}

impl<I, T> Map<I, T>
where
    I: Integer + Copy + TryInto<usize> + TryFrom<usize>,
    <I as TryInto<usize>>::Error: std::fmt::Debug,
    <I as TryFrom<usize>>::Error: std::fmt::Debug,
{
    pub fn from_char_map<F: Fn(char) -> T>(s: &str, f: F) -> Self {
        let data: Vec<T> = s.lines().flat_map(|line| line.chars().map(&f)).collect();
        let bounds: GridIndex<I> = GridIndex::new(
            s.lines().count().try_into().unwrap(),
            s.lines()
                .next()
                .unwrap()
                .chars()
                .count()
                .try_into()
                .unwrap(),
        );

        if data.len() != (bounds.row.try_into().unwrap() * bounds.column.try_into().unwrap()) {
            panic!("All rows must have the same number of columns");
        };

        Self { data, bounds }
    }
}

impl<I, T> From<Map<I, T>> for String
where
    I: Integer + Copy + TryInto<usize> + TryFrom<usize>,
    <I as TryInto<usize>>::Error: std::fmt::Debug,
    <I as TryFrom<usize>>::Error: std::fmt::Debug,
    T: Copy + Into<char>,
{
    fn from(map: Map<I, T>) -> String {
        map.data
            .chunks(map.bounds.column.try_into().unwrap())
            .map(|row| row.iter().map(|&c| c.into()).collect::<String>())
            .join("\n")
    }
}
