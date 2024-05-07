use std::ops::{Index, IndexMut};

use crate::Float;

#[derive(Debug)]
pub struct SquareMatrix<const N: usize>(pub [[Float; N]; N]);

impl<const N: usize> SquareMatrix<N> {
    pub fn new() -> Self {
        let mut m = Self([[0 as Float; N]; N]);
        for i in 0..N {
            m.0[i][i] = 1 as Float;
        }
        m
    }
}

impl<const N: usize> Index<usize> for SquareMatrix<N> {
    type Output = [Float; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for SquareMatrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
