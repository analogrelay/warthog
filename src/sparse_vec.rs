#[derive(Clone, PartialEq)]
pub struct SparseVec<T>(Vec<(usize, T)>);

impl<T> SparseVec<T> {
    pub fn new() -> SparseVec<T> {
        SparseVec(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn set(&mut self, index: usize, val: T) {
        match self.0.binary_search_by(|x| x.0.cmp(&index)) {
            Ok(idx) => {
                // Already exists, overwrite it
                self.0[idx] = (index, val);
            }
            Err(idx) => {
                // Doesn't exist, insert it
                self.0.insert(idx, (index, val));
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match self.0.binary_search_by(|x| x.0.cmp(&index)) {
            Ok(idx) => Some(&self.0[idx].1),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match self.0.binary_search_by(|x| x.0.cmp(&index)) {
            Ok(idx) => Some(&mut self.0[idx].1),
            Err(_) => None,
        }
    }

    pub fn get_or_add<F: FnOnce(usize) -> T>(&mut self, index: usize, factory: F) -> &mut T {
        match self.0.binary_search_by(|x| x.0.cmp(&index)) {
            Ok(idx) => &mut self.0[idx].1,
            Err(idx) => {
                let mut item = factory(index);
                self.0.insert(idx, (index, item));
                &mut self.0[idx].1
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(usize, T)> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn set_only_tracks_present_items() {
        let mut svec = SparseVec::new();

        svec.set(0, "Item 0");
        svec.set(10, "Item 10");
        svec.set(100, "Item 100");

        assert_eq!(
            vec![(0, "Item 0"), (10, "Item 10"), (100, "Item 100")],
            svec.0
        );
    }

    pub fn set_keeps_items_ordered() {
        let mut svec = SparseVec::new();

        svec.set(0, "Item 0");
        svec.set(10, "Item 10");
        svec.set(100, "Item 100");
        svec.set(5, "Item 5");

        assert_eq!(
            vec![
                (0, "Item 0"),
                (5, "Item 5"),
                (10, "Item 10"),
                (100, "Item 100")
            ],
            svec.0
        );
    }

    pub fn get_returns_expected_item() {
        let mut svec = SparseVec::new();

        svec.set(0, "Item 0");
        svec.set(10, "Item 10");
        svec.set(100, "Item 100");

        assert_eq!(Some(&"Item 10"), svec.get(10));
    }

    pub fn get_returns_none_if_no_item_at_index() {
        let mut svec = SparseVec::new();

        svec.set(0, "Item 0");
        svec.set(10, "Item 10");
        svec.set(100, "Item 100");

        assert_eq!(None, svec.get(5));
    }
}
