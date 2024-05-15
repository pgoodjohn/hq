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