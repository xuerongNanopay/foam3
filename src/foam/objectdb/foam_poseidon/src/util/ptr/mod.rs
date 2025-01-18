mod weak_vec;

//TODO:
struct WeakVec<'a, T> {
    data: &'a [T], // Slice, does not own the memory
}

impl<'a, T> WeakVec<'a, T> {
    // Constructor
    pub fn new(data: &'a [T]) -> Self {
        Self { data }
    }

    // Get the length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Access element by index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    // Iterate over the elements
    pub fn iter(&self) -> std::slice::Iter<'a, T> {
        self.data.iter()
    }
}