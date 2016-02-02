#![crate_type = "lib"]
#![crate_name = "vesema"]
mod vec_set;
mod vec_map;
mod sorted_vec;
mod sorted_set;
mod sorted_map;

pub use vec_set::VecSet;
pub use vec_map::VecMap;
pub use sorted_vec::SortedVec;
pub use sorted_set::SortedSet;
pub use sorted_map::SortedMap;

#[cfg(test)]
mod test {
	use VecSet;
    #[test]
    fn vecset() {
		let mut a = VecSet::new();
		assert!(a.insert(0));
		assert!(a.insert(1));
		assert!(!a.insert(0));
		assert!(a.remove(&0));
		assert!(!a.remove(&0));
    }
}
