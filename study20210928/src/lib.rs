// 外部ファイルに定義したモジュールをCrate Rootに定義することでコンパイル対象にする
mod sample_module;
mod sample_module2;
use sample_module2::sample_module2::cooking;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(test)]
mod greeting_tests {
    // useを使うとモジュールを呼び出すための手続きを簡略化できる
    use crate::sample_module::greeting;

    #[test]
    fn say_hello_works() {
        assert_eq!(greeting::say_hello(), ())
    }

    #[test]
    fn say_goodbye_works() {
        assert_eq!(greeting::say_goodbye(), ())
    }
}
