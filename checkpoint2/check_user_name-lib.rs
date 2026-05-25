// Define the AccessLevel enum
pub enum AccessLevel {
    Guest,
    Normal,
    Admin,
}

// Define the User struct with public fields
pub struct User {
    pub name: String,
    pub access_level: AccessLevel,
}

impl User {
    // Constructor to initialize the User
    pub fn new(name: String, access_level: AccessLevel) -> Self {
        Self { name, access_level }
    }

    // Method to check access level and return the name conditionally
    pub fn send_name(&self) -> Option<&str> {
        match self.access_level {
            AccessLevel::Guest => None,
            AccessLevel::Normal | AccessLevel::Admin => Some(&self.name),
        }
    }
}

// Function to validate the user and format the output
pub fn check_user_name(user: &User) -> (bool, &str) {
    match user.send_name() {
        Some(name) => (true, name),
        None => (false, "ERROR: User is guest"),
    }
}