use crate::shared::grid_index::GridIndex;
use anyhow::{anyhow, Error, Result};
use num::Signed;
use std::fmt::{Display, Formatter};
use strum::EnumIter;

/*-------------------------------------------------------------------------------------------------
  Directions
-------------------------------------------------------------------------------------------------*/

/*--------------------------------------------------------------------------------------
  AnyDirection Trait
--------------------------------------------------------------------------------------*/

pub trait AnyDirection<I>
where
    I: Signed,
{
    fn offset(&self) -> GridIndex<I>;
}

/*--------------------------------------------------------------------------------------
  GridDirection
--------------------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum GridDirection {
    Up,
    Down,
    Left,
    Right,
}

impl GridDirection {
    pub fn turn_clockwise(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn turn_counterclockwise(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    #[inline(always)]
    pub fn turn_right(&self) -> Self {
        self.turn_clockwise()
    }

    #[inline(always)]
    pub fn turn_left(&self) -> Self {
        self.turn_counterclockwise()
    }
}

/*-----------------------------------------------------------------------------
  Implement AnyDirection
-----------------------------------------------------------------------------*/

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

/*-----------------------------------------------------------------------------
  Display
-----------------------------------------------------------------------------*/

impl Display for GridDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

/*-----------------------------------------------------------------------------
  Type Conversions
-----------------------------------------------------------------------------*/

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
