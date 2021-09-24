// 通常の構造体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Coordinate(u64, u64, u64);

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}

impl User {
    fn update_username(&mut self, username: String) {
        self.username = username
    }
}

fn main() {
    let user1 = build_user(String::from("koralle@koralle.com"), String::from("koralle"));

    // 同一構造体の他のインスタンスから新しいインスタンスを生成するときに特殊な記法で書ける
    let mut user2 = User {
        email: String::from("mugicha@koralle.com"),
        ..user1
    };

    user2.update_username(String::from("mugicha"));

    println!("{}", user2.username);
    println!("{}", user2.email);
    println!("{}", user2.sign_in_count);
    println!("{}", user2.active);

    let coordinate = Coordinate(45, 32, 0);

    println!("{}", coordinate.0);
    println!("{}", coordinate.1);
    println!("{}", coordinate.2);
}
