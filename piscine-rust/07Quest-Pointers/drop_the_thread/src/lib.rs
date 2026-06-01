use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl ThreadPool {
    // Initializes the ThreadPool with default empty values
    pub fn new() -> Self {
        ThreadPool {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    // Creates a new thread, records its initial state as 'false' (not dropped)
    pub fn new_thread(&self, c: String) -> (usize, Thread) {
        let pid = self.thread_len();
        self.states.borrow_mut().push(false);
        let thread = Thread::new(pid, c, self);
        
        (pid, thread)
    }

    // Returns the current number of threads registered
    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    // Checks if the thread at the given id has been dropped
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    // Mutates the state to reflect a dropped thread
    pub fn drop_thread(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        
        if states[id] {
            panic!("{} is already dropped", id);
        } else {
            states[id] = true;
            self.drops.set(self.drops.get() + 1);
        }
    }
}

#[derive(Debug)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    // Initializes a new Thread
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        Thread {
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    // Takes ownership of the thread and immediately goes out of scope,
    // thereby triggering the `Drop` trait automatically.
    pub fn skill(self) {}
}

impl Drop for Thread<'_> {
    // Automatically called when a Thread instance goes out of scope or is explicitly dropped
    fn drop(&mut self) {
        // The instructions mention calling "add_drop", but based on the provided
        // function signatures, the intended function on ThreadPool is `drop_thread`.
        self.parent.drop_thread(self.pid);
    }
}