#[derive(Clone, Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: usize,
}

#[derive(Debug)]
struct Color(i32, i32, i32); // Tupul Struct

#[derive(Debug)]
struct AlwaysEqual; // Unit-like Struct

fn main() {
    eample51();
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn eample51() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("thirdemail@example.com"),
        String::from("adifferentperson123"),
    );

    let user3 = User {
        email: String::from("wowmoreemails@example.com"),
        ..user1.clone()
    };

    let black = Color(0, 0, 0);
    let weird = AlwaysEqual;

    println!("User1: {:?}, User2: {:?}, User3: {:?}", user1, user2, user3);
    println!("Black: {:?}", black);
    println!("AlwaysEqual: {:?}", weird);
}
