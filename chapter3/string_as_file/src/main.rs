#![allow(unused_variables)]

use rand::random;

static mut ERROR: isize = 0;

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str, data: &Vec<u8>) -> File {
        File {
            name: String::from(name),
            data: data.clone(),
        }
    }
}

fn open(f: &mut File) -> Result<bool, String> {
    Result::Ok(true)
}

fn close(f: &mut File) -> Result<bool, String> {
    Result::Ok(true)
}

#[allow(dead_code)]
fn read(f: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
    if random() && random() && random() {
        Result::Err("An error has occured".to_string())
    } else {
        let mut tmp = f.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Result::Ok(read_length)
    }
}

fn main() {
    let mut f2 = File::new("2.txt", &vec![114, 117, 115, 116, 33]);

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);

    let result = read(&f2, &mut buffer);

    let f2_length = result.unwrap();
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text);
}

mod other_stuff;
