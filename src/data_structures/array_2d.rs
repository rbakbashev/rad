//! Handy 2D array abstraction over a single [Vec] supporting `[y][x]` operations

use std::fmt;
use std::ops::{Index, IndexMut};

pub struct Array2D<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Copy> Array2D<T> {
    pub fn new(init: T, width: usize, height: usize) -> Self {
        let data = vec![init; width * height];

        Self {
            width,
            height,
            data,
        }
    }

    pub fn from_slice(width: usize, height: usize, data: &[T]) -> Self {
        Self {
            width,
            height,
            data: data.to_vec(),
        }
    }

    pub fn get(&self, y: usize, x: usize) -> Option<&T> {
        let idx = self.idx(x, y)?;
        self.data.get(idx)
    }

    pub fn get_mut(&mut self, y: usize, x: usize) -> Option<&mut T> {
        let idx = self.idx(x, y)?;
        self.data.get_mut(idx)
    }

    fn idx(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(y * self.width + x)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> Index<usize> for Array2D<T> {
    type Output = [T];

    fn index(&self, y: usize) -> &Self::Output {
        assert!(y < self.height);

        let start = y * self.width;

        &self.data[start..start + self.width]
    }
}

impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        assert!(y < self.height);

        let start = y * self.width;

        &mut self.data[start..start + self.width]
    }
}

impl<T> AsRef<[T]> for Array2D<T> {
    fn as_ref(&self) -> &[T] {
        self.data.as_ref()
    }
}

impl<T> AsMut<[T]> for Array2D<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.data.as_mut()
    }
}

impl<T: fmt::Debug> fmt::Debug for Array2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("Array2D");

        f.field("width", &self.width);
        f.field("height", &self.height);

        for (i, row) in self.data.chunks(self.width).enumerate() {
            let text = format!("row {}", i + 1);

            f.field(&text, &row);
        }

        f.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        let mut arr = Array2D::new(0, 3, 2);

        *arr.get_mut(1, 2).expect("unexpected out of bounds") = 123;

        assert_eq!(arr.get(1, 2), Some(&123));
        assert_eq!(arr.get(0, 0), Some(&0));
        assert_eq!(arr.get(2, 0), None);

        assert_eq!(arr.get_mut(0, 3), None);

        arr.as_mut()[0] = 321;
        assert_eq!(arr.get(0, 0), Some(&321));
    }

    #[test]
    fn via_trait() {
        let mut arr = Array2D::new(0, 3, 2);

        arr[1][2] = 123;

        assert_eq!(arr[1][2], 123);
        assert_eq!(arr[0][0], 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed: y < self.height")]
    #[allow(clippy::no_effect_underscore_binding)]
    fn trait_panic() {
        let arr = Array2D::new(0, 3, 2);
        let _x = arr[2][0];
    }

    #[test]
    fn debug() {
        let mut arr = Array2D::new(0, 3, 2);

        arr[0].clone_from_slice(&[1, 2, 3]);
        arr[1].clone_from_slice(&[4, 5, 6]);

        let fmt = format!("{:?}", arr);
        let exp = "Array2D { width: 3, height: 2, row 1: [1, 2, 3], row 2: [4, 5, 6] }";

        assert_eq!(fmt, exp);
    }
}
