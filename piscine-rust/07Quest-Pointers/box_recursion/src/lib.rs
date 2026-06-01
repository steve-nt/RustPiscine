#[derive(Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match s {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            // We'll treat anything else (like "Normal Worker") as a Worker
            _ => Role::Worker, 
        }
    }
}

// A Link is either Some pointer to a boxed Worker, or None (end of the list)
pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        // Create the new worker, taking the current head (self.grade) 
        // and setting it as the `next` value for this new worker.
        let new_worker = Box::new(Worker {
            name: name.to_string(),
            role: Role::from(role),
            // .take() replaces self.grade with None and returns the previous value
            next: self.grade.take(), 
        });

        // Set the new worker as the new head of the list
        self.grade = Some(new_worker);
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        // .take() takes ownership of the current head of the list
        self.grade.take().map(|boxed_worker| {
            let worker = *boxed_worker; // Unbox the worker
            self.grade = worker.next;   // The next worker becomes the new head
            worker.name                 // Return the name of the removed worker
        })
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        // We use .as_ref() to peek at the value without taking ownership
        self.grade.as_ref().map(|boxed_worker| {
            // Since Role doesn't derive Clone in the provided boilerplate, 
            // we can just match it to create a fresh copy for the return tuple.
            let copied_role = match boxed_worker.role {
                Role::CEO => Role::CEO,
                Role::Manager => Role::Manager,
                Role::Worker => Role::Worker,
            };
            
            (boxed_worker.name.clone(), copied_role)
        })
    }
}