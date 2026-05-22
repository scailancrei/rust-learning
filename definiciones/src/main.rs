fn main() {
    let user1 = build_user(
        "juanAntonio".to_string(),
        "juanantonio.dev@ħotmail.com".to_string(),
    );

    println!("{}", user1.active);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in: u64,
}
