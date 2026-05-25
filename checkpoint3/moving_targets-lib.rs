#[derive(Debug, PartialEq, Eq)]
pub struct Target {
    pub size: u32,
    pub xp: u32,
}

pub struct Field {
    head: Link,
}

// In Rust, recursive types must have a known size at compile time. 
// We use `Box<Node>` to allocate the node on the heap, giving the pointer a fixed size.
// We wrap it in an `Option` because a link can either point to the next node (`Some`) 
// or point to nothing at the end of the list (`None`).
type Link = Option<Box<Node>>;

struct Node {
    elem: Target,
    next: Link,
}

impl Field {
    /// Initializes an empty Field.
    pub fn new() -> Self {
        Field { head: None }
    }

    /// Adds a new Target to the front (head) of the list.
    pub fn push(&mut self, target: Target) {
        // We create a new node allocated on the heap.
        let new_node = Box::new(Node {
            elem: target,
            // `self.head.take()` is crucial here. It takes the value out of `self.head`
            // and replaces it with `None`. This allows us to take ownership of the 
            // old head and attach it to our new node without violating Rust's borrowing rules.
            next: self.head.take(),
        });
        
        // We then set the head of the list to point to our newly created node.
        self.head = Some(new_node);
    }

    /// Removes the first element from the list and returns it.
    pub fn pop(&mut self) -> Option<Target> {
        // `take()` grabs the current head, leaving `None` in `self.head`.
        // `map` is then used to process the boxed node if it exists (`Some`).
        self.head.take().map(|node| {
            // `node` is of type `Box<Node>`. 
            // We advance the head of the Field to the next node in the sequence.
            self.head = node.next; 
            
            // We return the actual `Target` data. When this closure ends, 
            // the `Box` is automatically deallocated, but we keep the `Target`.
            node.elem
        })
    }

    /// Returns a shared reference to the first Target in the list without removing it.
    pub fn peek(&self) -> Option<&Target> {
        // `as_ref()` converts `&Option<Box<Node>>` into `Option<&Box<Node>>`.
        // This is necessary because we don't want to consume (take ownership of) the head,
        // we just want to look at it.
        self.head.as_ref().map(|node| {
            // We return a reference to the element inside the node.
            &node.elem
        })
    }

    /// Returns a mutable reference to the first Target in the list without removing it.
    pub fn peek_mut(&mut self) -> Option<&mut Target> {
        // `as_mut()` converts `&mut Option<Box<Node>>` into `Option<&mut Box<Node>>`.
        // This allows us to modify the contents of the head node.
        self.head.as_mut().map(|node| {
            // We return a mutable reference to the element inside the node.
            &mut node.elem
        })
    }
}

// Implementing the `Default` trait is a standard Rust practice 
// whenever a struct has a `new()` function that takes no arguments.
impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}