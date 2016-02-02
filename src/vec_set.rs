use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

pub struct VecSet<T> {
	v: Vec<T>
}

impl<T: Eq> VecSet<T> {
	pub fn new() -> Self {
		VecSet { v: Vec::new() }
	}

	pub fn with_capacity(size: usize) -> Self {
		VecSet { v: Vec::with_capacity(size) }
	}

	pub fn from_vec(v: Vec<T>) -> Self {
		VecSet { v: v }
	}

	pub fn to_vec(self) -> Vec<T> {
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

	pub fn insert(&mut self, x: T) -> bool {
		for v in self.v.iter() {
			if x == *v { return false }
		}
		self.v.push(x);
		true
	}

	pub fn remove(&mut self, x: &T) -> bool {
		let mut idx = None;
		for (i, v) in self.v.iter().enumerate() {
			if v == x {
				idx = Some(i);
				break
			}
		}
		match idx {
			None => false,
			Some(idx) => {
				self.v.swap_remove(idx);
				true
			}
		}
	}

	pub fn clear(&mut self) {
		self.v.clear()
	}
}
