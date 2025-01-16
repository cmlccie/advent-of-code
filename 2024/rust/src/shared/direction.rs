use crate::shared::grid_index::GridIndex;
use anyhow::{anyhow, Error, Result};
use num::Signed;
use std::fmt::{Display, Formatter};
use strum::EnumIter;

/*-------------------------------------------------------------------------------------------------
  Direction
-------------------------------------------------------------------------------------------------*/

pub trait AnyDirection<I>
where
    I: Signed,
{
    fn offset(&self) -> GridIndex<I>;
}

/*-----------------------------------------------------------------------------
  Grid Direction
-----------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy)]
pub enum GridDirection {
    Up,
    Down,
    Left,
    Right,
}

impl<I: Signed> AnyDirection<I> for GridDirection {
    fn offset(&self) -> GridIndex<I> {
        match self {
            Self::Up => GridIndex::new(I::one().neg(), I::zero()),
            Self::Down => GridIndex::new(I::one(), I::zero()),
            Self::Left => GridIndex::new(I::zero(), I::one().neg()),
            Self::Right => GridIndex::new(I::zero(), I::one()),
        }
    }
}

impl Display for GridDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl TryFrom<char> for GridDirection {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            c => Err(anyhow!("Invalid GridDirection character: {:?}", c)),
        }
    }
}

impl From<GridDirection> for char {
    fn from(direction: GridDirection) -> char {
        match direction {
            GridDirection::Up => '^',
            GridDirection::Down => 'v',
            GridDirection::Left => '<',
            GridDirection::Right => '>',
        }
    }
}

/*--------------------------------------------------------------------------------------
  Compass Direction (4-Cardinal Directions)
--------------------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum CompassDirection {
    North,
    East,
    South,
    West,
}

impl CompassDirection {
    pub fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

impl<I: Signed> AnyDirection<I> for CompassDirection {
    fn offset(&self) -> GridIndex<I> {
        match self {
            Self::North => GridIndex::new(I::one().neg(), I::zero()),
            Self::South => GridIndex::new(I::one(), I::zero()),
            Self::East => GridIndex::new(I::zero(), I::one()),
            Self::West => GridIndex::new(I::zero(), I::one().neg()),
        }
    }
}
