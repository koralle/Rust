// 公開するモジュールには`pub`を付ける
pub mod greeting {
    pub fn say_hello() {
        println!("Hello, world!");
    }

    pub fn say_goodbye() {
        println!("Goodbye!");
    }
}
