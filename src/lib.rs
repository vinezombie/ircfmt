mod colors;
mod format;
#[cfg(test)]
mod tests;

pub use colors::*;
pub use format::*;

use std::{cmp::Ordering, num::NonZeroUsize};

/// A non-empty sequence of byte offsets and formatting codes for them.
#[derive(Clone, Debug, Default)]
pub struct Formats(Format, Vec<(NonZeroUsize, Format)>);

impl Formats {
    pub fn first(&self) -> &Format {
        &self.0
    }
    pub fn first_mut(&mut self) -> &mut Format {
        &mut self.0
    }
    #[inline]
    pub fn last(&self) -> (usize, &Format) {
        if let Some((i, f)) = self.1.last() {
            (i.get(), f)
        } else {
            (0, &self.0)
        }
    }
    #[inline]
    pub fn last_mut(&mut self) -> (usize, &mut Format) {
        if let Some((i, f)) = self.1.last_mut() {
            (i.get(), f)
        } else {
            (0, &mut self.0)
        }
    }
    pub fn pop(&mut self) -> Option<(usize, Format)> {
        self.1.pop().map(|(i, f)| (i.get(), f))
    }
    fn get_(&self, iidx: usize) -> (usize, &Format) {
        if iidx == 0 {
            (0, &self.0)
        } else {
            let (i, f) = &self.1[iidx - 1];
            (i.get(), f)
        }
    }
    fn get_mut_(&mut self, iidx: usize) -> (usize, &mut Format) {
        if iidx == 0 {
            (0, &mut self.0)
        } else {
            let (i, f) = &mut self.1[iidx - 1];
            (i.get(), f)
        }
    }
    fn set_(&mut self, iidx: usize, format: Format) {
        // TODO: If we're at the end of the Vec, check if we can pop.
        *self.get_mut_(iidx).1 = format;
    }
    fn insert_after_(&mut self, idx: usize, element: (NonZeroUsize, Format)) {
        if idx == 0 {
            if self.0 != element.1 {
                self.1.insert(0, element)
            }
        } else if self.1[idx - 1].1 != element.1 {
            self.1.insert(idx, element)
        }
    }
    fn search(&self, idx: usize) -> Result<usize, usize> {
        if idx == 0 {
            return Ok(0);
        }
        // Common edge case: we're going to want to update the last element.
        let last_idx = self.last().0;
        match idx.cmp(&last_idx) {
            Ordering::Greater => Err(self.1.len() + 1),
            Ordering::Equal => Ok(self.1.len()),
            Ordering::Less => {
                // If idx is less than last_idx, then the Vec cannot be empty.
                self.1[..self.1.len()].binary_search_by_key(&idx, |(k, _)| k.get()).map(|k| k + 1)
            }
        }
    }
    #[inline]
    pub fn lookup(&self, idx: usize) -> (usize, &Format) {
        let idx = self.search(idx).unwrap_or_else(|v| v - 1);
        self.get_(idx)
    }
    #[inline]
    pub fn lookup_mut(&mut self, idx: usize) -> (usize, &mut Format) {
        let idx = self.search(idx).unwrap_or_else(|v| v - 1);
        self.get_mut_(idx)
    }
    pub fn set(&mut self, idx: usize, format: Format) -> &mut Formats {
        match self.search(idx) {
            Ok(iidx) => self.set_(iidx, format),
            Err(iidx) => {
                // If we're here, idx isn't zero.
                let nzero = NonZeroUsize::new(idx).unwrap();
                self.insert_after_(iidx, (nzero, format));
            }
        }
        self
    }
    /// Runs the provided function on every [`Format`] contained in `self`.
    pub fn transform(&mut self, mut f: impl FnMut(&mut Format)) -> &mut Self {
        f(&mut self.0);
        for (_, fmt) in &mut self.1 {
            f(fmt)
        }
        self
    }
}
