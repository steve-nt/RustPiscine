use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: RefCell<usize>,
    pub max: usize,
}

impl Tracker {
    // Initializes the structure with a passed max value.
    pub fn new(max: usize) -> Self {
        Tracker {
            messages: RefCell::new(Vec::new()),
            value: RefCell::new(0),
            max,
        }
    }

    // Sets the value to the passed argument's reference count.
    pub fn set_value<T>(&self, value: &Rc<T>) {
        let count = Rc::strong_count(value);
        let percentage = (count * 100) / self.max;

        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else {
            // Update the internal value
            *self.value.borrow_mut() = count;

            if percentage >= 70 {
                self.messages.borrow_mut().push(format!(
                    "Warning: You have used up over {}% of your quota!",
                    percentage
                ));
            }
        }
    }

    // Takes a peek at how much usage the passed argument value already has.
    pub fn peek<T>(&self, value: &Rc<T>) {
        let count = Rc::strong_count(value);
        let percentage = (count * 100) / self.max;
        
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            percentage
        ));
    }
}