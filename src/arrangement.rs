use std::fmt;
use std::iter::FusedIterator;

use super::lazy_buffer::LazyBuffer;
use alloc::vec::Vec;

use crate::adaptors::checked_binomial;


/// An iterator to iterate through all the `k`-length combinations in an iterator.
///
/// See [`.combinations()`](crate::Itertools::combinations) for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Arrrangement<I: Iterator> {
    indices: Vec<usize>,
    pool: LazyBuffer<I>,
    first: bool,
    n: usize,
    k: usize,
    i: usize,
    j: usize
}


impl<I> fmt::Debug for Arrrangement<I>
where
    I: Iterator + fmt::Debug,
    I::Item: fmt::Debug,
{
    debug_fmt_fields!(Arrrangement, indices, pool, first);
}


/// Create a new `Combinations` from a clonable iterator.
pub fn arrangement<I>(iter: I, k: usize) -> Arrrangement<I>
where
    I: Iterator,
{
    Arrrangement {
        n: k,
        pool: LazyBuffer::new(iter),
        k,
        i: k.saturating_sub(1),
        j: k,
        first: true,
        indices: (0..k).collect()
    }
}

impl<I: Iterator> Arrrangement<I> {
    /// Returns the length of a combination produced by this iterator.
    #[inline]
    pub fn k(&self) -> usize {
        self.k
    }

    /// Returns the (current) length of the pool from which combination elements are
    /// selected. This value can change between invocations of [`next`](Combinations::next).
    #[inline]
    pub fn n(&self) -> usize {
        self.pool.len()
    }

    /// Returns a reference to the source pool.
    #[inline]
    pub(crate) fn src(&self) -> &LazyBuffer<I> {
        &self.pool
    }

    /// Resets this `Combinations` back to an initial state for combinations of length
    /// `k` over the same pool data source. If `k` is larger than the current length
    /// of the data pool an attempt is made to prefill the pool so that it holds `k`
    /// elements.
    // pub(crate) fn reset(&mut self, k: usize) {
    //     self.first = true;

    //     if k < self.indices.len() {
    //         self.indices.truncate(k);
    //         self.indices = self.memory.iter();
    //     } else {
    //         self.indices = self.memory.iter();
    //         self.pool.prefill(k);
    //     }
    // }
    pub(crate) fn reset(&mut self, k: usize) {
        self.first = true;

        if k < self.indices.len() {
            self.indices.truncate(k);
            for i in 0..k {
                self.indices[i] = i;
            }
        } else {
            for i in 0..self.indices.len() {
                self.indices[i] = i;
            }
            self.indices.extend(self.indices.len()..k);
            self.pool.prefill(k);
        }
    }

}



impl<I> Iterator for Arrrangement<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            self.pool.prefill(self.k());
            if self.k() > self.n() {
                return None;
            }
            return match (self.k, self.n) {
                (0, _) => None,
                (_, 0) => None,
                (_, _) => Some(self.indices.iter().map(|i| self.pool[*i].clone()).collect())

            }
        }
        // here check for more in the pool
        if self.pool.get_next() {
            // here we need to make some sort of lookback
            // 
            self.n = self.pool.len();
        }
        if self.indices[0] >= self.n - self.k {
            return None
        }
        if self.indices[self.i] < self.n - 1 {
            // println!("hhhhere with n : {}", self.n);

            self.indices[self.i] += 1;
            return Some(self.indices.iter().map(|i| self.pool[*i].clone()).collect())
        } else {
            let mut i = self.i;
            // println!("ici i vaut : {i}");
            while self.indices[i] == self.indices[i-1] + 1 {
                i -= 1;
            }
            // println!("et là i vaut : {i}");

            self.j = i - 1;

            self.indices[self.j] += 1;
            let mut p = self.indices[self.j];
            for j in self.indices[i..].iter_mut() {
                p += 1;
                *j = p;
            }
            self.i = self.k-1;

            return Some(self.indices.iter().map(|i| self.pool[*i].clone()).collect())
        }
       
    }

}






