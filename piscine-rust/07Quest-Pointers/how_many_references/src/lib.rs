use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    // Creates a new Node with the given initial state.
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Node { ref_list }
    }

    // Adds a cloned Rc to the ref_list.
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    // Removes all elements from the vector that point to the SAME allocation as `element`.
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        // `retain` keeps only the elements for which the closure returns true.
        // We use `Rc::ptr_eq` to check pointer addresses instead of comparing the String values.
        self.ref_list.retain(|e| !Rc::ptr_eq(e, &element));
    }
}

// Returns how many active clones of the exact same Rc allocation exist.
pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}