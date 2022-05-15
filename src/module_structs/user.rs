pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

impl User {
    // Method
    pub fn say(&self) -> String {
        format!("{} active: {}", self.username, self.active)
    }

    // Associated Function
    pub fn get_example_user() -> User {
        User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        }
    }

    // Associated Function
    pub fn new(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }
}
