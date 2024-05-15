pub struct Application(String);

impl Application {
    // Constructor to create a new MyString
    pub fn new(s: &str) -> Application {
        Application(s.to_string())
    }

    // Method to convert to &str
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct Zone(String);

// TODO: Make it an enum
impl Zone {
    // Constructor to create a new MyString
    pub fn new(s: &str) -> Zone {
        Zone(s.to_string())
    }

    // Method to convert to &str
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
