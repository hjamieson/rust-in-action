#![allow(unused)]

use bincode::deserialize;
use core::panic;
use libactionkv::ActionKV;
use std::collections::HashMap;
use std::env;
use std::path::Path;

const USAGE: &str = "
Usage:
   akv_mem FILE get KEY
   akv_mem FILE delete KEY
   akv_mem FILE insert KEY
   akv_mem FILE update KEY
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(USAGE).as_ref();
    let key = args.get(3).expect(USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load();

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap();
        }
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        }
        _ => eprintln!("{}", &USAGE),
    }
}
fn old_main() {
    let file_arg = env::args().nth(1).expect("missing args!");
    println!("file_arg = {:?}", file_arg);
    let file_path = Path::new(&file_arg);
    let mut store = ActionKV::open(file_path).expect("unable to open file!");
    store.load();

    // insert_test(&mut store);
    read_test(&mut store);

    println!("done!");
}

fn insert_test(store: &mut ActionKV) {
    let r = store.insert("one".as_bytes(), "uno".as_bytes());
    match r {
        Err(err) => {
            eprintln!("insert error: {}", err);
            panic!("fail");
        }
        _ => (),
    }
}

fn read_test(store: &mut ActionKV) {
    let kv = store.get("one".as_bytes()).unwrap();
    match kv {
        None => eprintln!("key {} not found", "one"),
        Some(v) => {
            println!("value: {:?}", v);
            let data = String::from_utf8(v).expect("not utf-8!");
            println!("value = {}", data);
        }
    }
}
