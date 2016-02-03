use std::slice::{Iter, IterMut};
use std::vec::IntoIter;
use std::cmp::*;

pub struct SortedVec<T> {
	v: Vec<T>
}

impl<T: Ord> SortedVec<T> {
	pub fn new() -> Self {
		SortedVec { v: Vec::new() }
	}

	pub fn with_capacity(size: usize) -> Self {
		SortedVec { v: Vec::with_capacity(size) }
	}

	pub fn from_vec(mut v: Vec<T>) -> Self {
		v.sort();
		SortedVec { v: v }
	}

	pub fn from_vec_unchecked(v: Vec<T>) -> Self {
		SortedVec { v: v }
	}

	pub fn into_vec(self) -> Vec<T> {
		self.v
	}

	pub fn capacity(&self) -> usize {
		self.v.capacity()
	}

	pub fn reserve(&mut self, additional: usize) {
		self.v.reserve(additional)
	}

	pub fn shrink_to_fit(&mut self) {
		self.v.shrink_to_fit()
	}

	pub fn iter(&self) -> Iter<T> {
		self.v.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		self.v.iter_mut()
	}

	pub fn into_iter(self) -> IntoIter<T> {
		self.v.into_iter()
	}

	pub fn first(&self) -> Option<&T> {
		self.v.first()
	}

	pub fn last(&self) -> Option<&T> {
		self.v.last()
	}

	pub fn binary_search(&self, x: &T) -> Result<usize, usize> {
		self.v.binary_search(x)
	}

	pub fn contains(&self, x: &T) {
		self.binary_search(x).is_ok()
	}

	pub fn insert(&mut self, x: T) {
		let search = self.binary_search(x);
		self.v.insert(match search {
			Ok(idx) => idx,
			Err(idx) => idx,
		}, x)
	}

	pub fn remove(&mut self, x: &T) -> bool {
		let search = self.binary_search(x);
		match search {
			Ok(idx) => {
				self.v.remove(idx);
				true
			},
			Err(idx) => false,
		}
	}

	pub fn remove_at(&mut self, idx: usize) -> T {
		self.v.remove(idx)
	}

	pub fn clear(&mut self) {
		self.v.clear()
	}
}
