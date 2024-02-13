#![allow(unused)]

#[derive(Copy,Clone,Debug)]
struct CubeSat {
    id: u64,
}

enum StatusMessage{
    Ok,
}

#[derive(Clone)]
struct Demo {
    name: String,
}
// impl Clone for Demo {
//     fn clone(&self) -> Demo {
//         *self
//     }
// }
// impl Copy for Demo {}

fn check_status(sat: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn check_demo(demo: Demo) -> String {
    demo.name
}

fn main() {
    let sat = CubeSat{id: 123};
    let msg = check_status(sat);
    let msg2 = check_status(sat);

    let demo = Demo{name: String::from("hello")};
    let name = check_demo(demo.clone());
    println!("{name}");
    let name2 = check_demo(demo);
}
