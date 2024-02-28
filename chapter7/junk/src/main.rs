#![allow(unused)]

#[macro_use]
extern crate serde_json;

use std::num::ParseIntError;

type Special = Result<i32, String>;

fn main() -> Result<(), ParseIntError> {
    // let result = mult2("5", "7");
    // print(result);

    // let bad_result = mult3("5", "t");
    // print(bad_result);

    // let bad_result_2 = mult3("t", "5");
    // print(bad_result_2);

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("the first doubled is {:?}", double_first(numbers));
    println!("the first doubled is {:?}", double_first(empty));
    println!("the first doubled is {:?}", double_first(strings));

    Ok(())
}

fn basic_hash(s: &str) -> u8 {
    let a = s.as_bytes()[0];
    a
}

fn basic_hash_2(key: &str) -> u32 {
    let first = key.chars().next().unwrap_or('\0');
    u32::from(first)
}

fn to_i32(s: &str) -> Result<i32, &str> {
    match s.parse::<i32>() {
        Err(e) => Err("kaboom!"),
        _ => Ok(5),
    }
}

fn always_fails() -> Result<String, i32> {
    Err(42i32)
}

fn mod_two(val: i32) -> Result<bool, String> {
    match val % 2 {
        0 => Ok(true),
        _ => Err(String::from("odd number")),
    }
}

fn multiply(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let first = a.parse::<i32>()?;
    let second = b.parse::<i32>()?;
    Ok(first * second)
}

fn mult2(a: &str, b: &str) -> Result<i32, ParseIntError> {
    a.parse::<i32>()
        .and_then(|n1| b.parse::<i32>().map(|n2| n1 * n2))
}

fn mult3(a: &str, b: &str) -> Special {
    a.parse::<i32>()
        .map_err(|_| "a is wrong!".to_string())
        .and_then(|n1| {
            b.parse::<i32>()
                .map_err(|e| "b is messed up".to_string())
                .map(|n2| n1 * n2)
        })
}

fn print<T: std::fmt::Debug>(result: Result<i32, T>) {
    match result {
        Ok(val) => println!("value is {}", val),
        Err(e) => println!("fail: {:?}", e),
    }
}

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    #[test]
    fn basic_hash_test() {
        let my_str = "abc";
        println!("a: {}, A: {}", basic_hash("a"), basic_hash("A"));
        assert_eq!(basic_hash(my_str), 97);
    }
    #[test]
    fn basic_hash_2_test() {
        assert_eq!(97, basic_hash_2("abc"));
        assert_eq!(65, basic_hash_2("ABC"));
        println!("{}", u32::from('4'));
        println!("{}", u32::from('0'));
    }

    #[test]
    fn hashmap_1() {
        let mut capitals = HashMap::new();
        capitals.insert("Cook Islands", "Avurua");
        capitals.insert("Fiji", "Suva");
        capitals.insert("Kiribati", "South Tarawa");
        let fiji = capitals["Fiji"];
        assert_eq!(fiji, "Suva");
        let maybe_italy = capitals.get("Italy");
        assert!(maybe_italy.is_none());
    }

    #[test]
    fn test_json() {
        let capitals = json!({
            "Cook Isalds":"Avarua",
            "Fiji":"Suva",
            "Niue":"Alofi",
            "Tomga":"Nuku alofa"
        });
        assert_eq!("Alofi", capitals["Niue"]);
    }

    #[test]
    fn result_test_1() {
        assert_eq!(5, to_i32("1").unwrap());
        assert!(to_i32("foo").is_err());
        match to_i32("foo") {
            Err(m) => assert_eq!(m, "kaboom!"),
            _ => panic!("should err"),
        };
        let a = to_i32("12").unwrap();
        assert_eq!(a, 12);
    }

    #[test]
    fn result_test_2() {
        assert!(always_fails().is_err());
        assert!(mod_two(22).unwrap());
        assert!(mod_two(21).is_err());
    }

    #[test]
    fn mult1() {
        assert_eq!(multiply("10", "5"), Ok(50));
        assert!(multiply("10", "five").is_err());
    }

    #[test]
    fn mult2_test() {
        assert!(mult2("5", "7").is_ok());
        assert_eq!(mult2("5", "7"), Ok(35));
        assert!(mult2("5", "f").is_err());
    }
}
