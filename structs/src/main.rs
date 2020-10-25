#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64
}

impl User {
    fn get_username(&self) -> String {
        self.username.clone()
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    
    let first_user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1
    };

    let second_user = build_user(String::from("otherone@example.com"), String::from("otherone"));

    println!("user1 is {:?} and user2 is {:?}", first_user, second_user);

    println!("username of user1 is {} and username of user2 is {}", first_user.get_username(), second_user.get_username());
}
