//! Types relating to indexing into a terminal grid

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

/// Behavior for handling grid boundaries
pub enum Boundary {
    Clamp,
    Wrap,
}

/// An integral index representing a row or column in a grid
pub struct Index(usize);

impl Index {
    const FLOATING_POINT_ERROR_ADJUSTMENT: f64 = 0.0001;
}

impl From<usize> for Index {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

/// Index in the grid using row, column notation
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Point {
        Point { row, col }
    }

    pub const fn zero() -> Self {
        Self { row: 0, col: 0 }
    }

    pub fn wrapping_add(mut self, num_cols: usize, distance: usize) -> Point {
        self.row += (distance + self.col) / num_cols;
        self.col = (self.col + distance) % num_cols;
        self
    }

    pub fn wrapping_sub(mut self, num_cols: usize, distance: usize) -> Point {
        let line_changes = (distance + num_cols - 1).saturating_sub(self.col) / num_cols;
        if self.row >= line_changes {
            self.row -= line_changes;
            self.col = (num_cols + self.col - distance % num_cols) % num_cols;
            self
        } else {
            Point::new(0, 0)
        }
    }

    fn as_one_dimensional_index(&self, num_cols: usize) -> usize {
        self.row * num_cols + self.col
    }

    pub fn max_point<'a>(&'a self, other: &'a Point, num_cols: usize) -> &'a Point {
        if self.as_one_dimensional_index(num_cols) >= other.as_one_dimensional_index(num_cols) {
            self
        } else {
            other
        }
    }

    pub fn distance(&self, num_cols: usize, other: &Point) -> usize {
        let this_index = self.as_one_dimensional_index(num_cols);
        let other_index = other.as_one_dimensional_index(num_cols);
        this_index.abs_diff(other_index)
    }

    pub fn to_visible_point(&self, history_size: usize) -> VisiblePoint {
        VisiblePoint {
            row: VisibleRow(self.row.saturating_sub(history_size)),
            col: self.col,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        match (self.row.cmp(&other.row), self.col.cmp(&other.col)) {
            (Ordering::Equal, ord) | (ord, _) => ord,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct VisibleRow(pub usize);

impl Sub<usize> for VisibleRow {
    type Output = VisibleRow;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<Self> for VisibleRow {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Sub<Self> for VisibleRow {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Add<Self> for VisibleRow {
    type Output = VisibleRow;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<usize> for VisibleRow {
    type Output = VisibleRow;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<usize> for VisibleRow {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl VisibleRow {
    pub fn saturating_sub(&self, rhs: usize) -> VisibleRow {
        VisibleRow(self.0.saturating_sub(rhs))
    }
}

impl fmt::Display for VisibleRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct VisiblePoint {
    pub row: VisibleRow,
    pub col: usize,
}

impl VisiblePoint {
    pub fn zero() -> Self {
        Self {
            row: VisibleRow(0),
            col: 0,
        }
    }

    pub fn wrapping_add(mut self, num_cols: usize, distance: usize) -> VisiblePoint {
        self.row += (distance + self.col) / num_cols;
        self.col = (self.col + distance) % num_cols;
        self
    }

    pub fn wrapping_sub(mut self, num_cols: usize, distance: usize) -> VisiblePoint {
        let line_changes = (distance + num_cols - 1).saturating_sub(self.col) / num_cols;
        if self.row >= VisibleRow(line_changes) {
            self.row = self.row - line_changes;
            self.col = (num_cols + self.col - distance % num_cols) % num_cols;
            self
        } else {
            VisiblePoint {
                row: VisibleRow(0),
                col: 0,
            }
        }
    }

    pub fn wrap(self, num_cols: usize) -> VisiblePoint {
        self.wrapping_add(num_cols, 0)
    }
}

impl PartialOrd for VisiblePoint {
    fn partial_cmp(&self, other: &VisiblePoint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VisiblePoint {
    fn cmp(&self, other: &VisiblePoint) -> Ordering {
        match (self.row.cmp(&other.row), self.col.cmp(&other.col)) {
            (Ordering::Equal, ord) | (ord, _) => ord,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_zero() {
        let p = Point::zero();
        assert_eq!(p.row, 0);
        assert_eq!(p.col, 0);
    }

    #[test]
    fn test_point_wrapping_add() {
        let p = Point::new(0, 5);
        let p = p.wrapping_add(10, 8);
        assert_eq!(p.row, 1);
        assert_eq!(p.col, 3);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(0, 5);
        assert_eq!(p1.distance(10, &p2), 5);
    }
}
