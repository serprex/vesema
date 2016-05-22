use std::cmp::Ordering;
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;
use SortedVec;

pub struct SortableKeyPair<K: Ord, V>(pub K, pub V);

impl<K: Ord, V> PartialEq for SortableKeyPair<K, V> {
	fn eq(&self, y: &Self) -> bool {
		self.0 == y.0
	}

	fn ne(&self, y: &Self) -> bool {
		self.0 != y.0
	}
}

impl<K: Ord, V> Eq for SortableKeyPair<K, V> {
}

impl<K: Ord, V> PartialOrd for SortableKeyPair<K, V> {
	fn partial_cmp(&self, y: &Self) -> Option<Ordering> {
		self.0.partial_cmp(&y.0)
	}
}

impl<K: Ord, V> Ord for SortableKeyPair<K, V> {
	fn cmp(&self, y: &Self) -> Ordering {
		self.0.cmp(&y.0)
	}
}

pub struct SortedMap<K: Ord, V> {
	v: SortedVec<SortableKeyPair<K, V>>
}

impl<K: Ord, V> SortedMap<K, V> {
	pub fn new() -> Self {
		SortedMap { v: SortedVec::new() }
	}

	pub fn with_capacity(size: usize) -> Self {
		SortedMap { v: SortedVec::with_capacity(size) }
	}

	pub fn from_vec(mut v: Vec<SortableKeyPair<K, V>>) -> Self {
		v.sort_by(|x, y| x.0.cmp(&y.0));
		SortedMap::from_vec_unchecked(v)
	}

	pub fn from_vec_unchecked(v: Vec<SortableKeyPair<K, V>>) -> Self {
		SortedMap { v: SortedVec::from_vec_unchecked(v) }
	}

	pub fn into_vec(self) -> Vec<SortableKeyPair<K, V>> {
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

	pub fn iter(&self) -> Iter<SortableKeyPair<K, V>> {
		self.v.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<SortableKeyPair<K, V>> {
		self.v.iter_mut()
	}

	pub fn into_iter(self) -> IntoIter<SortableKeyPair<K, V>> {
		self.v.into_iter()
	}

	pub fn first(&self) -> Option<&SortableKeyPair<K, V>> {
		self.v.first()
	}

	pub fn last(&self) -> Option<&SortableKeyPair<K, V>> {
		self.v.last()
	}

	pub fn binary_search(&self, k: &K) -> Result<usize, usize> {
		self.v.binary_search_by(|x| x.0.cmp(k))
	}

	pub fn contains_key(&self, k: &K) -> bool {
		self.binary_search(k).is_ok()
	}

	pub fn insert(&mut self, x: SortableKeyPair<K, V>) -> bool {
		let bs = self.binary_search(&x.0);
		match bs {
			Ok(idx) => {
				self.v[idx].1 = x.1;
				false
			},
			Err(idx) => {
				self.v.insert_at(idx, x);
				true
			},
		}
	}

	pub fn remove(&mut self, k: &K) -> bool {
		let bs = self.binary_search(k);
		match bs {
			Ok(idx) => {
				self.v.remove_at(idx);
				true
			},
			Err(_) => false,
		}
	}

	pub fn remove_at(&mut self, idx: usize) -> SortableKeyPair<K, V> {
		self.v.remove_at(idx)
	}

	pub fn clear(&mut self) {
		self.v.clear()
	}
}

impl<K: Ord, V: PartialEq> SortedMap<K, V> {
	pub fn contains_value(&self, v: &V) -> bool {
		self.v.iter().any(|x| x.1 == *v)
	}

	pub fn contains(&self, kv: &SortableKeyPair<K, V>) -> bool {
		let bs = self.binary_search(&kv.0);
		match bs {
			Ok(idx) => self.v[idx].1 == kv.1,
			Err(_) => false,
		}
	}
}
