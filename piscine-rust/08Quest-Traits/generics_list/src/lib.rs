#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

// We must declare the generic type <T> on the implementation block
impl<T> List<T> {
    // Returns a new empty list
    pub fn new() -> Self {
        List { head: None }
    }

    // Adds a new element to the beginning of the list
    pub fn push(&mut self, value: T) {
        // We take the current head, map it into a Box, and assign it as the new node's `next`.
        let new_node = Node {
            value,
            next: self.head.take().map(Box::new),
        };
        // The new node becomes the new head of the list.
        self.head = Some(new_node);
    }

    // Removes the first element from the list
    pub fn pop(&mut self) {
        // If there is a head, take it out of the Option
        if let Some(old_head) = self.head.take() {
            // Unbox the next node (if it exists) and make it the new head
            self.head = old_head.next.map(|boxed_node| *boxed_node);
        }
    }

    // Returns the size of the list
    pub fn len(&self) -> usize {
        let mut count = 0;
        
        // We start looking at the head as an Option<&Node<T>> so we don't take ownership
        let mut current = self.head.as_ref();
        
        // Traverse the list until we hit a None
        while let Some(node) = current {
            count += 1;
            // .as_deref() converts Option<&Box<Node<T>>> into Option<&Node<T>>
            current = node.next.as_deref();
        }
        
        count
    }
}