use std::slice::{Iter, IterMut};
use std::vec::IntoIter;
use SortedVec;

#[derive(Default)]
pub struct SortedSet<T> {
	v: SortedVec<T>
}

impl<T: Ord> SortedSet<T> {
	pub fn new() -> Self {
		SortedSet { v: SortedVec::new() }
	}

	pub fn with_capacity(size: usize) -> Self {
		SortedSet { v: SortedVec::with_capacity(size) }
	}

	pub fn from_vec(v: Vec<T>) -> Self {
		SortedSet { v: SortedVec::from_vec(v) }
	}

	pub fn from_vec_unchecked(v: Vec<T>) -> Self {
		SortedSet { v: SortedVec::from_vec_unchecked(v) }
	}

	pub fn into_vec(self) -> Vec<T> {
		self.v.into_vec()
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

	pub fn contains(&self, x: &T) -> bool {
		self.binary_search(x).is_ok()
	}

	pub fn insert(&mut self, x: T) -> bool {
		let search = self.binary_search(&x);
		match search {
			Ok(_) => false,
			Err(idx) => {
				self.v.insert_at(idx, x);
				true
			},
		}
	}

	pub fn remove(&mut self, x: &T) -> bool {
		let search = self.binary_search(x);
		match search {
			Ok(idx) => {
				self.v.remove_at(idx);
				true
			},
			Err(_) => false,
		}
	}

	pub fn remove_at(&mut self, idx: usize) -> T {
		self.v.remove_at(idx)
	}

	pub fn clear(&mut self) {
		self.v.clear()
	}
}
