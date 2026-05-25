use std::cell::{Cell, RefCell};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Blog {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Blog {
    /// Creates an empty blog with 0 drops and an empty states vector.
    pub fn new() -> Self {
        Blog {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    /// Creates a new article, registers its state as 'false' (not dropped), 
    /// and returns its new id and the Article instance.
    pub fn new_article(&self, body: String) -> (usize, Article<'_>) {
        let id = self.new_id();
        // We use borrow_mut() on the RefCell to push a new state
        self.states.borrow_mut().push(false);
        
        let article = Article::new(id, body, self);
        (id, article)
    }

    /// Returns the length of the states vector, representing the next available id.
    pub fn new_id(&self) -> usize {
        self.states.borrow().len()
    }

    /// Returns the dropped state of a specific article id.
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    /// Marks an article as dropped. Panics if it was already dropped.
    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        
        if states[id] {
            panic!("{} is already dropped", id);
        }
        
        states[id] = true;
        // Cell allows us to mutate the value without a mutable reference to `self`
        self.drops.set(self.drops.get() + 1);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Article<'a> {
    pub id: usize,
    pub body: String,
    pub parent: &'a Blog,
}

impl<'a> Article<'a> {
    /// Initializes a new article.
    pub fn new(id: usize, body: String, parent: &'a Blog) -> Self {
        Article { id, body, parent }
    }

    /// Takes ownership of `self` and drops it. 
    /// Because `self` goes out of scope at the end of this function block, 
    /// Rust will automatically call the `Drop` implementation.
    pub fn discard(self) {
        // Nothing needed here. 
        // The magic happens when the function ends!
    }
}

/// The Drop trait allows us to customize what happens right before the value 
/// goes out of scope and is destroyed.
impl<'a> Drop for Article<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.id);
    }
}