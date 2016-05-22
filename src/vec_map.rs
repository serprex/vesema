use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

#[derive(Default)]
pub struct VecMap<K, V> {
	v: Vec<(K, V)>
}

impl<K: Eq, V> VecMap<K, V> {
	pub fn new() -> Self {
		VecMap { v: Vec::new() }
	}

	pub fn with_capacity(size: usize) -> Self {
		VecMap { v: Vec::with_capacity(size) }
	}

	pub fn from_vec(v: Vec<(K, V)>) -> Self {
		VecMap { v: v }
	}

	pub fn to_vec(self) -> Vec<(K, V)> {
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

	pub fn iter(&self) -> Iter<(K, V)> {
		self.v.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<(K, V)> {
		self.v.iter_mut()
	}

	pub fn into_iter(self) -> IntoIter<(K, V)> {
		self.v.into_iter()
	}

	pub fn contains_key(&mut self, x: &K) -> bool {
		self.v.iter().any(|&(ref k, _)| x == k)
	}

	pub fn insert(&mut self, k: K, v: V) -> Option<V> {
		use std::mem::replace;
		for &mut (ref mut vk, ref mut vv) in self.v.iter_mut() {
			if k == *vk {
				return Some(replace(vv, v))
			}
		}
		self.v.push((k, v));
		None
	}

	pub fn remove(&mut self, k: &K) -> bool {
		let mut idx = None;
		for (i, v) in self.v.iter().enumerate() {
			if &v.0 == k {
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

impl<K: Eq, V: PartialEq> VecMap<K, V> {
	pub fn contains_value(&mut self, x: &V) -> bool {
		self.v.iter().any(|&(_, ref v)| x == v)
	}

}
