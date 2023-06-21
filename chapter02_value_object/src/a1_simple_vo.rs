#![allow(dead_code)]

#[derive(Debug)]
struct FullName {
    first_name: String,
    last_name: String,
}

impl FullName {
    fn new(first_name: &str, last_name: &str) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    fn first_name(&self) -> String {
        self.first_name.clone()
    }

    fn last_name(&self) -> String {
        self.last_name.clone()
    }
}

impl PartialEq for FullName {
    fn eq(&self, other: &Self) -> bool {
        self.first_name == other.first_name && self.last_name == other.last_name
    }
}

#[test]
fn _2_2_2() {
    let full_name = String::from("taro tanaka");
    println!("{}", full_name);
    let tokens = full_name.split_whitespace().collect::<Vec<_>>();
    let last_name = tokens[0];
    println!("{}", last_name);
}

#[test]
fn _2_2_3() {
    let full_name = String::from("suzuki jiro");
    let tokens = full_name.split_whitespace().collect::<Vec<_>>();
    let last_name = tokens[0];
    println!("{}", last_name);
}

#[test]
#[warn(non_snake_case)]
fn _2_2_4___2_2_6() {
    let full_name = FullName::new("taro", "tanaka");
    println!("{}", full_name.first_name());
    println!("{}", full_name.last_name());
}

// DDD ではシステム固有の値を表す型をValue Objectとして定義する

// fullnameの例のように，システムに最適な値はプリミティブな型であるとは限らない

// 値の性質は主に以下のようになる
/*
    - 不変である
    - 同じ値を持つものは等価である
    - 同じ値を持つものは置き換え可能である
*/
#[test]
fn _2_2_7() {
    let mut greet: String = From::from("hello");

    greet = From::from("こんにちは");
    println!("{}", greet);
}
/*
    上の例の様に代入は"値"の置き換えではなく，"変数"の置き換えである
*/

#[test]
fn _2_2_10() {
    let _full_name = FullName::new("taro", "tanaka");
    // full_nameは,Value Object なので，不変であるべきである
    // Rust の変数はデフォルトで不変なので，以下の書き方はできない
    // full_name.first_name = "jiro".to_string();
}

#[test]
// 等価性で比較する
// そのためには，PartialEqトレイトを実装する必要がある
fn _2_2_13() {
    let full_name = FullName::new("taro", "tanaka");
    let full_name2 = FullName::new("taro", "tanaka");
    // 2つの値が等価であるかどうかを判定する
    assert_eq!(full_name, full_name2);
    if full_name == full_name2 {
        println!("full_nameとfull_name2は等価です");
    }
}
