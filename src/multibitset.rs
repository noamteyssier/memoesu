use fixedbitset::{FixedBitSet, IndexRange};

#[derive(Debug)]
#[allow(dead_code)]
pub struct MultiBitSet {
    /// The underlying bitsets in this data structure
    data: Vec<FixedBitSet>,
    /// The number of bits in each bitset
    n: usize,
    /// The number of bitsets
    m: usize,
}
impl MultiBitSet {
    pub fn new(n: usize, m: usize) -> Self {
        let mut data = Vec::with_capacity(m);
        for _ in 0..m {
            data.push(FixedBitSet::with_capacity(n));
        }
        Self { data, n, m }
    }

    /// Returns the number of bits in each bitset
    #[allow(dead_code)]
    pub fn n(&self) -> usize {
        self.n
    }

    /// Returns the number of bitsets
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.m
    }

    /// Returns the total number of bits in this data structure
    #[allow(dead_code)]
    pub fn bitsize(&self) -> usize {
        self.n * self.m
    }

    /// Returns a reference to the bitset at index `i`
    pub fn get_row(&self, i: usize) -> &FixedBitSet {
        &self.data[i]
    }

    /// Clears the bitset at index `i`
    pub fn clear(&mut self, i: usize) {
        self.data[i].clear();
    }

    /// Sets the bit at index `j` in the bitset at index `i`
    pub fn set(&mut self, i: usize, j: usize, enabled: bool) {
        self.data[i].set(j, enabled);
    }

    pub fn inplace_external_union(&mut self, i: usize, ext: &FixedBitSet) {
        self.data[i].union_with_unchecked(ext);
    }

    /// Performs an inplace difference of the bitsets at indices `i` with an external bitset `ext`
    #[allow(dead_code)]
    pub fn inplace_external_difference(&mut self, i: usize, ext: &FixedBitSet) {
        self.data[i].difference_with(ext);
    }

    /// Returns a mutable reference to the bitsets at indices `i` and `j`
    ///
    /// The returned references are ordered such that the first reference is the one that will be
    /// operated on
    ///
    /// # Panics
    /// Panics if `i` or `j` are out of bounds
    /// Panics if `i` and `j` are equal
    #[inline]
    fn mutable_references(&mut self, i: usize, j: usize) -> (&mut FixedBitSet, &mut FixedBitSet) {
        let mid = i.min(j);
        let (left, right) = self.data.split_at_mut(mid + 1);
        if i < j {
            // operate right into left
            (&mut left[mid], &mut right[j - mid - 1])
        } else {
            // operate left into right
            (&mut right[i - mid - 1], &mut left[mid])
        }
    }

    /// Performs an inplace union of the bitsets at indices `i` and `j` onto the bitset at index `i`
    pub fn inplace_union(&mut self, i: usize, j: usize) {
        let (a, b) = self.mutable_references(i, j);
        a.union_with_unchecked(b);
    }

    /// Performs an inplace difference of the bitsets at indices `i` and `j` onto the bitset at index `i`
    #[allow(dead_code)]
    pub fn inplace_difference(&mut self, i: usize, j: usize) {
        let (a, b) = self.mutable_references(i, j);
        a.difference_with(b);
    }

    /// Performs an inplace union on self at `i` with the index `j` in another `MultiBitSet`
    pub fn union_with(&mut self, other: &Self, i: usize, j: usize) {
        self.data[i].union_with_unchecked(&other.data[j]);
    }

    /// Performs an inplace difference on self at `i` with the index `j` in another `MultiBitSet`
    pub fn difference_with(&mut self, other: &Self, i: usize, j: usize) {
        self.data[i].difference_with(&other.data[j]);
    }

    pub fn set_range<I: IndexRange>(&mut self, i: usize, range: I, enabled: bool) {
        self.data[i].set_range(range, enabled);
    }

    pub fn pprint(&self) -> String {
        let mut s = String::new();
        for i in 0..self.m {
            s.push_str(&format!(
                "{}: {:?}\n",
                i,
                self.data[i].ones().collect::<Vec<_>>()
            ));
        }
        s
    }
}
